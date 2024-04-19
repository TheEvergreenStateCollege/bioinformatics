
use std::collections::HashMap;
use rayon::prelude::*;

pub struct KMerMultiMap<'a> {
    maps: Vec<HashMap<&'a str, Vec<usize>>>,
}

impl<'a> KMerMultiMap<'a> {
    pub fn new(genome: &'a str, k_mer_size: usize) -> Self {
        
        let maps: Vec<HashMap<&str, Vec<usize>>> = 
        (0..genome.len() - k_mer_size)
        .into_par_iter()
        .map(|index| (index, &genome[index..index+k_mer_size]))
        .fold(
            || HashMap::new(),
            |mut acc, (index, k_mer)| {
                acc.entry(k_mer).or_insert(vec![]).push(index);
                if index % 1_000_000 == 0 {
                     println!("Constructing Array: index {} of {}", index, genome.len());
                }
                acc
            },
        )
        .collect();
        Self { maps }
    }

    pub fn indicies(&self, k_mer: &str) -> Vec<usize> {
        self.maps.iter().fold(Vec::new(), |mut acc, map| {
                if let Some(indicies) = map.get(k_mer) {
                    acc.extend(indicies);
                }
                acc
            }
        )
    }
}