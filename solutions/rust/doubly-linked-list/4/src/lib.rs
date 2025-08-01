use std::cell::UnsafeCell;
use std::rc::Rc;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

struct NodeRef<T>(Rc<UnsafeCell<Node<T>>>);

impl<T> Clone for NodeRef<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> From<Node<T>> for NodeRef<T> {
    fn from(value: Node<T>) -> Self {
        NodeRef(Rc::new(UnsafeCell::new(value)))
    }
}

pub struct Node<T> {
    value: T,
    prev: Option<NodeRef<T>>,
    next: Option<NodeRef<T>>,
}

pub struct LinkedList<T> {
    first: Option<NodeRef<T>>,
    last: Option<NodeRef<T>>,
    len: usize,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    cur: Option<NodeRef<T>>,
}

pub struct Iter<'a, T>(Option<&'a NodeRef<T>>);


impl<T> LinkedList<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
            len: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let cur = self.first.as_ref().cloned();
        Cursor { list: self, cur }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let cur = self.last.as_ref().cloned();
        Cursor { list: self, cur }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self.first.as_ref())
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() { }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // SAFETY: we hold &mut on Cursor, therefore on the whole LinkedList, so we are the only one taking reference on the value
        self.cur.as_mut().map(|f| &mut (unsafe { &mut *f.0.get() }).value)
    }

    pub fn peek(&self) -> Option<&T> {
        // SAFETY: we hold reference on Cursor, therefore on the whole LinkedList, so nobody has &mut reference and we are taking only shared reference
        self.cur.as_ref().map(|f| &(unsafe { &*f.0.get() }).value)
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        let cur = self.cur.take()?;
        // SAFETY: we hold &mut on Cursor, therefore on the whole LinkedList, so we are the only one taking reference on the value
        self.cur = (unsafe { &*cur.0.get() }).next.clone();

        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        let cur = self.cur.take()?;
        // SAFETY: we hold &mut on Cursor, therefore on the whole LinkedList, so we are the only one taking reference on the value
        self.cur = (unsafe { &*cur.0.get() }).prev.clone();
        
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let cur = self.cur.take()?;

        // SAFETY: we hold &mut to the Cursor, therefore to the whole LinkedList. No other refs should exist at this time
        let node = unsafe { &*cur.0.get() };
        if let Some(next) = &node.next {
            (unsafe { &mut *next.0.get() }).prev = node.prev.clone();
        }
        if let Some(prev) = &node.prev {
            (unsafe { &mut *prev.0.get() }).next = node.next.clone();
        }
        if Rc::ptr_eq(&self.list.first.as_ref().unwrap().0, &cur.0) {
            self.list.first = node.next.clone();
        }
        if Rc::ptr_eq(&self.list.last.as_ref().unwrap().0, &cur.0) {
            self.list.last = node.prev.clone();
        }

        self.cur = node.next.clone().or_else(|| node.prev.clone());
        self.list.len -= 1;

        let cell = Rc::into_inner(cur.0).expect("Noone else can hold ref to our Rc");
        Some(cell.into_inner().value)
    }

    fn insert_between(&mut self, left: Option<NodeRef<T>>, right: Option<NodeRef<T>>, element: T) {
        let node = NodeRef::from(Node {
            value: element,
            prev: left.clone(),
            next: right.clone(),
        });

        if let Some(left) = left {
            // SAFETY: we hold &mut on Cursor, therefore on the whole LinkedList, so we are the only one taking reference on the value
            unsafe { &mut *left.0.get() }.next = Some(node.clone());
        } else {
            self.list.first = Some(node.clone());
        }
        if let Some(right) = right {
            // SAFETY: we hold &mut on Cursor, therefore on the whole LinkedList, so we are the only one taking reference on the value
            unsafe { &mut *right.0.get() }.prev = Some(node.clone());
        } else {
            self.list.last = Some(node.clone());
        }

        self.list.len += 1;
    }

    pub fn insert_after(&mut self, element: T) {
        let next = self.cur.as_ref().and_then(|cur| {
            // SAFETY: we hold &mut on Cursor, therefore on the whole LinkedList, so we are the only one taking reference on the value
            let cur_node = unsafe { &*cur.0.get() };
            cur_node.next.clone()
        });
        self.insert_between(self.cur.clone(), next, element);
    }

    pub fn insert_before(&mut self, element: T) {
        let prev = self.cur.as_ref().and_then(|cur| {
            // SAFETY: we hold &mut on Cursor, therefore on the whole LinkedList, so we are the only one taking reference on the value
            let cur_node = unsafe { &*cur.0.get() };
            cur_node.prev.clone()
        });
        self.insert_between(prev, self.cur.clone(), element);
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let cur = self.0?;

        // SAFETY: we hold reference to an item from our LinkedList, so nobody holds a &mut ref and we are allowed to take a shared reference
        let cur_node = unsafe { &*cur.0.get() };
        self.0 = cur_node.next.as_ref();

        Some(&cur_node.value)
    }
}
