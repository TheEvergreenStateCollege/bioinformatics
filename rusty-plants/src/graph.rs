fn main() {
    println!("Hello, world!");
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

    pub fn add_node(&mut self, value: T) {
        // TODO: Should this add another row and column to the matrix?
        self.nodes.push(value);
    }

    // TODO: We should probably add edges based on node values, not indexes.
    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        // Make sure the specified nodes indexes actually exist
        if self.nodes.get(from).is_none() || self.nodes.get(to).is_none() {
            // TODO: Return Err
            panic!("Nodes don't exist");
        }

        if from == to {
            // TODO: Return Err
            panic!("Same node");
        }

        // TODO: Use .get() so it doesn't panic
        self.adjacency_matrix[from][to] = weight;
    }
}
