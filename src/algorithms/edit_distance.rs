use rayon::iter::ParallelBridge;

use crate::data_structures::matrix::Matrix;

/// Return the minimum amount of changes needed to transform `s1` into `s2`.
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    // An extra row and column are required for the base cases
    let mut m = Matrix::<usize>::new(s1.len() + 1, s2.len() + 1);

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

    // We start from 1 to skip over the base cases
    for y in 1..m.height() {
        for x in 1..m.width() {
            // We only add 1 to the replace cost if the characters are different, because identical
            // characters require no change.
            let replace_cost = *m.get(x - 1, y - 1).unwrap()
                + if s1.as_bytes().get(x - 1) == s2.as_bytes().get(y - 1) {
                    0
                } else {
                    1
                };
            let delete_cost = *m.get(x - 1, y).unwrap() + 1;
            let insert_cost = *m.get(x, y - 1).unwrap() + 1;

            let min_cost = *[replace_cost, delete_cost, insert_cost]
                .iter()
                .min()
                .expect("This array obviously isn't empty");

            // If characters are not equal we must perform an operation, which will add 1 to our
            // cost.
            m.set(x, y, min_cost);
        }
    }

    println!("{}", m);

    // The answer is stored in the bottom right cell
    // We know this won't panic, because the width and height of
    // the matrix obviously can't be out of its own bounds.
    *m.get(m.width() - 1, m.height() - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_strings() {
        assert_eq!(edit_distance("kitten", "kitten"), 0);
    }

    #[test]
    fn test_single_character_difference() {
        assert_eq!(edit_distance("kitten", "sitten"), 1);
    }

    #[test]
    fn test_single_character_insertion() {
        assert_eq!(edit_distance("kitten", "kitteny"), 1);
    }

    #[test]
    fn test_single_character_deletion() {
        assert_eq!(edit_distance("kitten", "kittn"), 1);
    }

    #[test]
    fn test_multiple_character_difference() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn test_empty_to_non_empty_string() {
        assert_eq!(edit_distance("", "kitten"), 6);
    }

    #[test]
    fn test_non_empty_to_empty_string() {
        assert_eq!(edit_distance("kitten", ""), 6);
    }

    #[test]
    fn test_completely_different_strings() {
        assert_eq!(edit_distance("flaw", "lawn"), 2);
    }

    #[test]
    fn test_same_characters_different_order() {
        assert_eq!(edit_distance("abc", "cba"), 2);
    }

    #[test]
    fn test_strings_with_repeated_characters() {
        assert_eq!(edit_distance("aaaa", "aaa"), 1);
    }

    #[test]
    fn test_case_sensitivity() {
        assert_eq!(edit_distance("Kitten", "kitten"), 1);
    }

    #[test]
    fn test_real_world_example() {
        assert_eq!(edit_distance("intention", "execution"), 5);
    }

    #[test]
    fn test_both_strings_empty() {
        assert_eq!(edit_distance("", ""), 0);
    }

    #[test]
    fn test_one_character_to_empty_string() {
        assert_eq!(edit_distance("a", ""), 1);
    }

    #[test]
    fn test_empty_string_to_one_character() {
        assert_eq!(edit_distance("", "a"), 1);
    }
}
