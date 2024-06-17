//The transcriptome is all the coding DNA (genes) in the genome. The reference genome uses capitalization to encode this,
//with lower case bases being non-coding. I copy the bases to a new string because it will make it easier to use. The size is not so bad since only ~10% of the genome is coding.
//The excerpts represent all of the non-coding regions, including introns and the regions between genes.
//Each excerpt is stored as a tuple, which represents the index in the transcriptome after the excerpt (so an excerpt at the start is 0), and the length of the excerpt.
//This should allow me to match a read onto the transcriptome, and then map it back onto the genome with awareness of where the introns are in it.

pub struct Transcriptome {
    bases: String,
    excerpt_list: Vec<(usize, usize)>,
}

impl Transcriptome {
    pub fn new(genome: &str) -> Self {
        let mut bases = String::new();
        let mut excerpt_list: Vec<(usize, usize)> = Vec::new();
        let mut trans_index: usize = 0;
        let mut in_excerpt: bool = false;

        for ch in genome.chars() {
            if ch.is_lowercase() {
                if in_excerpt {
                    //this unwrap should be unable to fail, because in_exerpt starts false and for it to be true, an element must have been pushed to the list
                    excerpt_list.last_mut().unwrap().1 += 1;
                } else {
                    in_excerpt = true;
                    excerpt_list.push((trans_index, 1));
                }
            } else {
                trans_index += 1;
                bases.push(ch);
                in_excerpt = false;
            }
        }
        println!("Reference transcriptome constructed from reference genome");
        Transcriptome {
            bases,
            excerpt_list,
        }
    }

    pub fn get_bases(&self) -> &str {
        &self.bases
    }

    pub fn genome_index(&self, trans_index: &usize) -> usize {
        self.excerpt_list
            .iter()
            .filter_map(|(index, size)| {
                if index <= trans_index {
                    Some(size)
                } else {
                    None
                }
            })
            .sum()
    }
}
