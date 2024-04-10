#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

enum LinkResult {
    /// There should be a link to the first from the second node
    To(i32),
    /// There should be a link from the first to the second node
    From(i32),
    /// There should be no link between the two nodes
    NoLink,
}

struct Graph<T> {
    nodes: Vec<T>,
    adjacency_matrix: Vec<Vec<i32>>,
}

impl<T> Graph<T> {
    pub fn new(initial_size: usize) -> Self {
        Self {
            nodes: Vec::new(),
            adjacency_matrix: vec![vec![0; initial_size]; initial_size],
        }
    }

    pub fn add_node(&mut self, value: T, link_checker: fn(&T, &T) -> LinkResult) {
        self.nodes.push(value);
        let new_node_index = self.nodes.len() - 1;

        // If the adjacency matrix is too small, increase the size by 1000 cells
        if self.nodes.len() > self.adjacency_matrix.len() {
            self.adjacency_matrix
                .resize(self.nodes.len() + 100, vec![0; self.nodes.len() + 100]);
        }

        for (index, node) in self.nodes.iter().enumerate() {
            match link_checker(&value, node) {
                LinkResult::To(weight) => self.adjacency_matrix[index][new_node_index] = weight,
                LinkResult::From(weight) => self.adjacency_matrix[new_node_index][index] = weight,
                LinkResult::NoLink => (),
            }
        }
    }
}
