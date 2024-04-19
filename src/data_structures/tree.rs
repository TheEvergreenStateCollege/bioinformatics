#![allow(dead_code)]

struct Leaf<'a, T> {
    value: Option<T>,
    left: Option<&'a Leaf<'a, T>>,
    right: Option<&'a Leaf<'a, T>>,
}

struct Tree<'a, T> {
    root: &'a Leaf<'a, T>,
}

impl<T> Tree<'_, T> {
    pub fn new() -> Self {
        Self {
            root: &Leaf {
                value: None,
                left: None,
                right: None,
            },
        }
    }
}
