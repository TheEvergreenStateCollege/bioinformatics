use std::collections::HashMap;

pub struct FragmentArray<'a> {
    map: HashMap<&'a str, Vec<u32>>,
    frag_size: u32,
}

impl<'a> FragmentArray<'a> {
    pub fn new(bases: &'a str, frag_size: u32) -> Self {
        let mut map: HashMap<&str, Vec<u32>> = HashMap::new();
        for i in 0..bases.len() / frag_size as usize{
            let frag: &str = &bases[i * frag_size as usize..(i + 1) * frag_size as usize];
            map.entry(frag).or_insert(vec![i as u32]).push(i as u32);
        };
        Self { map, frag_size }
    }
    pub fn indicies(&self, k_mer: &str) -> Option<&Vec<u32>> {
        self.map.get(k_mer)
    }
    pub fn get_size(&self) -> u32 {
        self.frag_size
    }
}
