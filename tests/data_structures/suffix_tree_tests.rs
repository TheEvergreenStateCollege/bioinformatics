#[cfg(test)]
mod suffix_tree_tests {

    use rusty_plants::data_structures::suffix_trees::SuffixTree;

    #[test]
    fn create_suffix_trees_1() {
        let st = SuffixTree::new("x");
        let expected_st =
"Suffix tree for: x
1   |            | Root   | Root   | No SL  | [2]
2   | x          | 0      | End    | No SL  | []
";
        assert_eq!(expected_st, format!("{}", st));
    }

    #[test]
    fn create_suffix_trees_2() {
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