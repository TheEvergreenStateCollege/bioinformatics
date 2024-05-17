pub mod file_io;
pub mod data_structures;
pub mod algorithms;

#[cfg(test)]
mod edit_distance_tests {
    use crate::algorithms::edit_distance::string_compare_r;
    use crate::data_structures::suffix_trees::SuffixTree;

    #[test]
    fn edit_distance_1() {
        let dist = string_compare_r( "SALAMANDER", "SALAZAR");
        // From back to front,
        // Match the 'R'
        // Delete the 'NDE'
        // Match the 'A'
        // Substitute the 'Z' for 'M'
        assert_eq!(dist, 4);
    }

    #[test]
    fn edit_distance_2() {
        let dist = string_compare_r( "STRAP", "SCARAB");
        // From back to front,
        // Change 'B' to 'P'  (+1)
        // Match the 'RA'
        // Delete the 'A' (+1)
        // Change 'T' to 'C' (+1)
        // Match the 'S'
        assert_eq!(dist, 3);
    }

    #[test]
    fn edit_distance_3() {
        let dist = string_compare_r( "CAT", "CAR");
        assert_eq!(dist, 1);
    }

    #[test]
    fn try_suffix_tree(){
        let _st: SuffixTree = SuffixTree::new("xacxad");
    }

}

#[cfg(test)]
mod suffix_tree_tests {

    use super::data_structures::suffix_trees::SuffixTree;
    //use crate::data_structures::suffix_trees::SuffixTree;

    #[test]
    fn create_suffix_trees() {
        let st = SuffixTree::new("xaccxaca");
        let expected_st =
"Suffix tree for: xaccxaca
1   |            | Root   | Root   | No SL  | [7, 9, 11]
2   | cxaca      | 3      | End    | No SL  | []
3   | cxaca      | 3      | End    | No SL  | []
4   | cxaca      | 3      | End    | No SL  | []
5   | c          | 2      | 3      | 1      | [6, 4]
6   | xaca       | 4      | End    | No SL  | []
7   | xac        | 0      | 2      | 9      | [8, 2]
8   | a          | 7      | End    | No SL  | []
9   | ac         | 1      | 2      | 11     | [10, 3]
10  | a          | 7      | End    | No SL  | []
11  | c          | 2      | 2      | No SL  | [12, 5]
12  | a          | 7      | End    | No SL  | []
";
        assert_eq!(expected_st, format!("{}", st));
    }
}