pub mod file_io;
pub mod data_structures;
pub mod algorithms;

#[cfg(test)]
mod tests {
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


    #[test]
    pub fn st_check_for_property1(){ //Root should have an alphabet size number of children.
       
        let _st: SuffixTree = SuffixTree::new("abcdeaccbe");
        let _st_buffer: &Vec<crate::data_structures::suffix_trees::Node> = &_st.nodes;
        let _st_alphabetlen = _st.alphabet.len();

        if let Some(_st_root) = _st_buffer.get(1) { //Check if root exists.
            
            let _st_root_children_list = &_st_root.children;
            let _st_root_children_length = &_st_root.children.len();
            
            println!("Children of root: {:?}", _st_root_children_list);
            println!("Number of children root has: {:?}", _st_root_children_length);

            if (_st_root_children_length == &_st_alphabetlen) { //The actual check.
                println!("Property 1 Test: Root has an alphabet sized number of children and therefore passes property 1..");
            } else {
                println!("Error for property 1: Number of children for root != to size of alphabet... ")
            }

        } else {
            println!("Failed no root found in tree");
        }
    }
       
 }


