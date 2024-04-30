use crate::algorithms::edit_distance::string_compare_r;
use crate::{
    data_structures::{k_mer_array::FragmentArray, transcriptome::Transcriptome},
    file_io::fasta::Fragment,
};

pub fn align_fragment(
    frag: &Fragment,
    array: &FragmentArray,
    transcriptome: &Transcriptome,
) -> Option<(u32, u8)> {
    //index, error count of best alignment, or None if none of the sub_frags have any matches

    let trans_str: &str = transcriptome.get_bases();
    let k_mer_size: u32 = array.get_size();
    let mut matches: Vec<(u32, u8)> = Vec::new();

    for i in 0..frag.bases().len() - k_mer_size as usize {
        let k_mer: &str = &frag.bases()[i..i + k_mer_size as usize];

        //println!("{}", k_mer);

        if let Some(indicies) = array.indicies(k_mer) {
            println!("MATCH!");
            for index in indicies {
                let frag_index = index - i as u32;
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
    array: &FragmentArray,
    transcriptome: &Transcriptome,
) -> Vec<Option<(u32, u8)>> {
    let mut matches: Vec<Option<(u32, u8)>> = Vec::new();

    let mut count = 0;

    for frag in frags {

        // if count >= 1 {
        //     break;
        // }
        // count += 1;
        
        if frag.bases().len() < array.get_size() as usize {
            // println!("Fragment too short - skipping");
            continue;
        }

        let m = align_fragment(frag, array, transcriptome);
        
        // println!("{:?}", m);

        matches.push(m);
    }
    matches
}
