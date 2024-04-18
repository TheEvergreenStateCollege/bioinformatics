use std::collections::HashMap;
// use rayon::prelude::*;

pub struct KMerArray<'a> {
    map: HashMap<&'a str, Vec<usize>>,
}

impl<'a> KMerArray<'a> {
    pub fn new(genome: &'a str, k_mer_size: usize) -> Self {

        // The parallel algorithm actually seems to be slower, because it loses more time
        // combining the hashmaps created by each thread into one hashmap than it gains
        // from making them in parallel. It also uses a monstrous amount of RAM.

        // let map: HashMap<&str, Vec<usize>> = (0..genome.len() - k_mer_size)
        //     .into_par_iter()
        //     .map(|index| (index, &genome[index..index+k_mer_size]))
        //     .fold(
        //         || HashMap::new(),
        //         |mut acc, (index, k_mer)| {
        //             acc.entry(k_mer).or_insert(vec![]).push(index);
        //             if index % 1_000_000 == 0 {
        //                  println!("Constructing Array: index {} of {}", index, genome.len());
        //             }
        //             acc
        //         },
        //     )
        //     .reduce(
        //         || HashMap::new(),
        //         |m1, m2| {
        //             m2.iter().fold(m1, |mut acc, (k, vs)| {
        //                 acc.entry(k).or_insert(vec![]).extend(vs);
        //                 acc})
        //             },
        //         );
        

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
