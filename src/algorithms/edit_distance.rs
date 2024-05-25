use crate::data_structures::matrix::Matrix;

/// Return the minimum amount of changes needed to transform `s1` into `s2`.
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    // An extra row and column are required for the base cases
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

    // for col in s1.len() - 1..-1 {}

    // The answer is stored in the bottom right cell
    // We know this won't panic, because the width and height of
    // the matrix obviously can't be out of its own bounds.
    *m.get(m.width() - 1, m.height() - 1).unwrap()
}
