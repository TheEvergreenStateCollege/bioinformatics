#![allow(dead_code)]

use std::rc::Rc;

struct Leaf<T> {
    value: Option<T>,
    left: Option<Rc<Leaf<T>>>,
    right: Option<Rc<Leaf<T>>>,
}

struct Tree<T> {
    root: Leaf<T>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            root: Leaf {
                value: None,
                left: None,
                right: None,
            },
        }
    }
}
