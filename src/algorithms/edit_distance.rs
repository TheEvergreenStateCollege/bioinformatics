use crate::data_structures::matrix::Matrix;

pub fn edit_distance(s1: &str, s2: &str) -> u8 {
    // An extra row and column is required for the base case
    let matrix = Matrix::<u8>::new(s2.len() + 1, s1.len() + 1);

    0
}
