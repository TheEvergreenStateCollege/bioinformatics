use crate::data_structures::matrix::Matrix;

pub fn edit_distance(s1: &str, s2: &str) -> u8 {
    // An extra row and column is required for the base cases
    let mut m = Matrix::<usize>::new(s2.len() + 1, s1.len() + 1);

    // Fill the rightmost column with base case values
    for row in 0..m.height() {
        let value = s2.len() - row;
        m.set(s1.len(), row, value);
    }

    // Fill the bottom row with base case values
    for col in 0..m.width() {
        let value = s1.len() - col;
        m.set(col, s2.len(), value);
    }

    for col in s1.len() - 1..-1 {}

    println!("{}", m);

    0
}
