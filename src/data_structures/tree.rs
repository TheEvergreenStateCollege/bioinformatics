#![allow(dead_code)]

use std::rc::Rc;

struct Leaf<T> {
    value: Option<T>,
    leaves: Option<Vec<Rc<Leaf<T>>>>,
}

struct Tree<T> {
    root: Leaf<T>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            root: Leaf {
                value: None,
                leaves: None,
            },
        }
    }
}
