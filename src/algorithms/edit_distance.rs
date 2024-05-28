use crate::data_structures::matrix::Matrix;

/// Return the minimum amount of changes needed to transform `s1` into `s2`.
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    // An extra row and column are required for the base cases
    let mut m = Matrix::<usize>::new(s2.len() + 1, s1.len() + 1);

    // The base cases represent how many operations it takes to convert
    // an empty string into each substring.
    // To convert an empty string into each substring, we simply have to insert
    // the number of characters in that substring.
    // Fill the leftmost column with base case values
    for y in 0..m.height() {
        m.set(0, y, y);
    }
    // Fill the top row with base case values
    for x in 0..m.width() {
        m.set(x, 0, x);
    }

    for y in 1..m.height() {
        for x in 1..m.width() {
            let replace_cost = *m.get(x - 1, y - 1).unwrap();
            let delete_cost = *m.get(x - 1, y).unwrap();
            let insert_cost = *m.get(x, y - 1).unwrap();

            let mut edit_distance = *[replace_cost, delete_cost, insert_cost]
                .iter()
                .min()
                .unwrap();

            // If characters are equal
            if s1.chars().nth(y) != s2.chars().nth(x) {
                edit_distance += 1;
            }

            m.set(x, y, edit_distance);
        }
    }

    println!("{}", m);

    // The answer is stored in the bottom right cell
    // We know this won't panic, because the width and height of
    // the matrix obviously can't be out of its own bounds.
    *m.get(m.width() - 1, m.height() - 1).unwrap()
}
