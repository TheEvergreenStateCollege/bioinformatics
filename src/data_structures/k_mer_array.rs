use std::collections::HashMap;

pub struct KMerArray<'a> {
    map: HashMap<&'a str, Vec<u32>>,
    k_mer_size: u32,
}

impl<'a> KMerArray<'a> {
    pub fn new(genome: &'a str, k_mer_size: u32) -> Self {

        let mut map: HashMap<&str, Vec<u32>> = HashMap::new();
        let mut progress: u32 = 0;
        let mut bucket: u32 = 0;

        for index in 0..genome.len() - k_mer_size as usize {
            let k_mer: &str = &genome[index..index + k_mer_size as usize];
            if let Some(indicies) = map.get_mut(k_mer) {
                indicies.push(index as u32);
            } else {
                map.insert(k_mer, vec![index as u32]);
            };

            bucket += 1;
            if bucket > genome.len() as u32 / 100 {
                progress += 1;
                bucket = 0;
                println!("Constructing K-mer array: {}% complete", progress);
            }
        };
        Self { map, k_mer_size }
    }

    pub fn indicies(&self, k_mer: &str) -> Option<&Vec<u32>> {
        self.map.get(k_mer)
    }

    pub fn get_size(&self) -> u32 {
        self.k_mer_size
    }
}
