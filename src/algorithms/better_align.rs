use crate::algorithms::edit_distance::string_compare_r;
use crate::{
    data_structures::{k_mer_array::KMerArray, transcriptome::Transcriptome},
    file_io::fasta::Fragment,
};

pub fn align_fragment(
    frag: &Fragment,
    array: &KMerArray,
    transcriptome: &Transcriptome,
) -> Option<(u32, u8)> {
    //index, error count of best alignment, or None if none of the sub_frags have any matches

    let trans_str: &str = transcriptome.get_bases();
    let sub_frag_size: u32 = array.get_size();
    let mut matches: Vec<(u32, u8)> = Vec::new();

    for i in 0..frag.bases().len() / sub_frag_size as usize{
        let sub_frag: &str = &frag.bases()[i * sub_frag_size as usize..(i + 1) * sub_frag_size as usize];
        if let Some(indicies) = array.indicies(sub_frag) {
            for index in indicies {
                let frag_index = index - (i as u32 * sub_frag_size);
                let error = string_compare_r(
                    frag.bases(),
                    &trans_str[frag_index as usize..frag_index as usize + frag.bases().len()],
                );
                matches.push((frag_index, error as u8));
            }
        }
    }
    //weird that min_by_key returns an option containing a reference, but whatever, I just copy it
    matches.iter().min_by_key(|(_, errors)| errors).copied()
}

pub fn align_fragments(
    frags: &Vec<Fragment>,
    array: &KMerArray,
    transcriptome: &Transcriptome,
) -> Vec<Option<(u32, u8)>> {
    let mut matches: Vec<Option<(u32, u8)>> = Vec::new();
    for frag in frags {
        println!("Aligning fragment...");

        let m = align_fragment(frag, array, transcriptome);
        
        println!("{:?}", m);

        matches.push(m);
    }
    matches
}
