#![allow(dead_code)]

use std::collections::HashMap;

pub struct SuffixArray<'a> {
    map: HashMap<&'a str, Vec<u32>>,
}

impl<'a> SuffixArray<'a> {
    pub fn new(genome: &'a str, k_mer_size: usize) -> Self {
        let mut map: HashMap<&str, Vec<u32>> = HashMap::new();

        for index in 0..genome.len() - k_mer_size {
            let k_mer: &str = &genome[index..index + k_mer_size];
            if let Some(indicies) = map.get_mut(k_mer) {
                indicies.push(index as u32);
            } else {
                map.insert(k_mer, vec![index as u32]);
            };
        }
        Self { map }
    }
}
