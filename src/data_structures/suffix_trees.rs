struct Node {
    start: u32,
    end: u32,
    children: Vec<Node>,
}

struct SuffixTree {
    root: Node,
    string: String,
}
