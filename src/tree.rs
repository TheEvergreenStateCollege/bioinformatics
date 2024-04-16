struct Leaf<'a, T> {
    value: T,
    left: Option<&'a Leaf<'a, T>>,
    right: Option<&'a Leaf<'a, T>>,
}
