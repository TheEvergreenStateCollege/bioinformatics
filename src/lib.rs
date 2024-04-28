pub mod file_io;
pub mod data_structures;
pub mod algorithms;

use crate::algorithms::edit_distance::string_compare_r;
//use crate::Graph;

#[cfg(test)]
mod tests {
    use rayon::string;

    use crate::algorithms::edit_distance::string_compare_r;

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

}


