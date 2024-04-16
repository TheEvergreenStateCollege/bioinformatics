#![allow(dead_code)]

use std::collections::HashMap;

struct SuffixArray<'a> {
    map: HashMap<&'a str, Vec<u8>>,
}

impl SuffixArray<'_> {
    pub fn new(genome: &str, k_mer_size: i32) -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
