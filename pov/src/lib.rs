use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    label: T,
    subtrees: Vec<Self>,
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self {
            label,
            subtrees: Default::default(),
        }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(mut self, child: Self) -> Self {
        self.subtrees.push(child);
        self.subtrees.sort_unstable();
        self
    }

    fn path_to_t(&self, t: &T, path: &mut Vec<usize>) -> bool {
        if self.label == *t {
            true
        } else {
            for (i, sub) in self.subtrees.iter().enumerate() {
                path.push(i);
                if sub.path_to_t(t, path) {
                    return true;
                }
                path.pop().unwrap();
            }
            false
        }
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        if self.label == *from {
            return true;
        }

        let mut path = Vec::new();
        if !self.path_to_t(from, &mut path) {
            // POV target is not in the tree
            return false;
        }

        for i in &path {
            let mut sub = self.subtrees.remove(*i);
            std::mem::swap(self, &mut sub);
            self.subtrees.push(sub);
        }

        fn fixup<T2: Debug + Ord>(t: &mut Tree<T2>, c: usize) {
            if c > 0 {
                fixup(t.subtrees.last_mut().unwrap(), c - 1);
                t.subtrees.sort_unstable();
            }
        }
        fixup(self, path.len());

        true
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        let mut p1 = Vec::new();
        let mut p2 = Vec::new();
        if !self.path_to_t(from, &mut p1) {
            return None;
        }
        if !self.path_to_t(to, &mut p2) {
            return None;
        }

        // Find common path and skip it
        let mut p1 = &p1[..];
        let mut p2 = &p2[..];
        let mut root = &*self;
        while matches!((p1.first(), p2.first()), (Some(x), Some(y)) if x == y) {
            root = &root.subtrees[p1[0]];
            p1 = &p1[1..];
            p2 = &p2[1..];
        }

        fn walk<'a, 'b: 'a, T2: Debug + Ord>(
            res: &mut Vec<&'a T2>,
            t: &'b Tree<T2>,
            rev: bool,
            path: &[usize],
            skip_first: bool,
        ) {
            if !rev {
                res.push(&t.label);
            }
            if !path.is_empty() {
                walk(res, &t.subtrees[path[0]], rev, &path[1..], false);
            }
            if rev && !skip_first {
                res.push(&t.label);
            }
        }

        let mut res = Vec::with_capacity(p1.len() + p2.len());
        walk(&mut res, root, true, p1, true);
        walk(&mut res, root, false, p2, false);
        Some(res)
    }
}
