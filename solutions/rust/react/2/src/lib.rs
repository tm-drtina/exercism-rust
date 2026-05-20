#![allow(clippy::type_complexity)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    downstream: Vec<ComputeCellId>,
    value: T,
}

struct ComputeCell<T> {
    deps: Box<[CellId]>,
    downstream: Vec<ComputeCellId>,
    inputs: BTreeSet<InputCellId>,
    f: Box<dyn Fn(&[T]) -> T>,
    value: T,
}

pub struct Reactor<'a, T> {
    inputs: Vec<InputCell<T>>,
    computes: Vec<ComputeCell<T>>,
    callbacks: BTreeMap<ComputeCellId, Vec<Option<Box<dyn 'a + FnMut(T)>>>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            inputs: Default::default(),
            computes: Default::default(),
            callbacks: Default::default(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.inputs.push(InputCell {
            downstream: Default::default(),
            value: initial,
        });
        InputCellId(self.inputs.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let mut deps_values = Vec::with_capacity(dependencies.len());
        let mut inputs = BTreeSet::new();

        // This can return early, so avoid modifying other cells
        for dep in dependencies {
            deps_values.push(self.value(*dep).ok_or(*dep)?);
        }

        // No longer can return an error, safe to modify
        let id = ComputeCellId(self.computes.len());
        for dep in dependencies {
            match dep {
                CellId::Input(input_cell_id) => {
                    inputs.insert(*input_cell_id);
                    self.inputs[input_cell_id.0].downstream.push(id);
                }
                CellId::Compute(compute_cell_id) => {
                    inputs.extend(self.computes[compute_cell_id.0].inputs.iter());
                    self.computes[compute_cell_id.0].downstream.push(id);
                }
            }
        }

        let value = compute_func(&deps_values);

        self.computes.push(ComputeCell {
            deps: Box::from(dependencies),
            downstream: Vec::new(),
            inputs,
            f: Box::new(compute_func),
            value,
        });
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            // Note: the tests include variables from a different reactor!
            CellId::Input(InputCellId(index)) if self.inputs.len() > index => {
                Some(self.inputs[index].value)
            }
            CellId::Compute(ComputeCellId(index)) if self.computes.len() > index => {
                Some(self.computes[index].value)
            }
            _ => None,
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let index = id.0;
        if index >= self.inputs.len() {
            false
        } else {
            if self.inputs[index].value != new_value {
                self.inputs[index].value = new_value;
                self.propagate_update(id);
            }
            true
        }
    }

    fn propagate_update(&mut self, input: InputCellId) {
        let mut candidates = self.inputs[input.0]
            .downstream
            .iter()
            .copied()
            .collect::<VecDeque<_>>();
        let mut stable_cells = BTreeSet::<ComputeCellId>::new();

        while let Some(cell_id) = candidates.pop_front() {
            if stable_cells.contains(&cell_id) {
                continue;
            }
            let is_stable = self.computes[cell_id.0].deps.iter().all(|d| match d {
                CellId::Input(_) => true,
                CellId::Compute(compute_cell_id) => {
                    !self.computes[compute_cell_id.0].inputs.contains(&input)
                        || stable_cells.contains(compute_cell_id)
                }
            });
            if is_stable {
                stable_cells.insert(cell_id);

                let cell = &self.computes[cell_id.0];
                let deps_values = cell
                    .deps
                    .iter()
                    .map(|d| self.value(*d).unwrap())
                    .collect::<Vec<_>>();

                let value = (*cell.f)(&deps_values);
                if value != cell.value {
                    candidates.extend(cell.downstream.iter());
                    self.computes[cell_id.0].value = value;
                    #[allow(clippy::manual_flatten)]
                    if let Some(callbacks) = self.callbacks.get_mut(&cell_id) {
                        for callback in callbacks {
                            if let Some(c) = callback {
                                (*c)(value);
                            }
                        }
                    }
                }
            } else {
                // retry later
                candidates.push_back(cell_id);
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if id.0 >= self.computes.len() {
            None
        } else {
            let callbacks = self.callbacks.entry(id).or_default();
            let id = CallbackId(callbacks.len());
            callbacks.push(Some(Box::new(callback)));
            Some(id)
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if cell.0 >= self.computes.len() {
            Err(RemoveCallbackError::NonexistentCell)
        } else {
            let Some(callbacks) = self.callbacks.get_mut(&cell) else {
                return Err(RemoveCallbackError::NonexistentCallback);
            };
            let Some(cb) = callbacks.get_mut(callback.0) else {
                return Err(RemoveCallbackError::NonexistentCallback);
            };
            if cb.is_some() {
                *cb = None;
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        }
    }
}
