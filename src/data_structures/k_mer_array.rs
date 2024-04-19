use std::collections::HashMap;
use rayon::prelude::*;

pub struct KMerArray<'a> {
    map: HashMap<&'a str, Vec<usize>>,
}

impl<'a> KMerArray<'a> {
    pub fn new(genome: &'a str, k_mer_size: usize) -> Self {

        let mut map: HashMap<&str, Vec<usize>> = HashMap::new();
        for index in 0..genome.len() - k_mer_size {
            let k_mer: &str = &genome[index..index + k_mer_size];
            if let Some(indicies) = map.get_mut(k_mer) {
                indicies.push(index);
            } else {
                map.insert(k_mer, vec![index]);
            };
            // if index % 1_000_000 == 0 {
            //     println!("Constructing Array: index {} of {}", index, genome.len());
            // }
        };
        Self { map }
    }

    pub fn indicies(&self, k_mer: &str) -> Option<&Vec<usize>> {
        self.map.get(k_mer)
    }
}
