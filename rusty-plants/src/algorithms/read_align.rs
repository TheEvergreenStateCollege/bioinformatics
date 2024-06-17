use crate::{
    algorithms::edit_distance::edit_distance,
    data_structures::{suffix_tree::SuffixTree, transcriptome::Transcriptome},
    file_io::fasta::Fragment,
};

#[derive(Debug)]
pub struct Match {
    pub index: usize,
    pub errors: usize,
}

pub fn align_fragment(read: &Fragment, st: &SuffixTree, transcriptome: &Transcriptome) -> Vec<Match> {
    let trans_bases = transcriptome.get_bases();
    let read_bases = read.bases();

    let mut matches: Vec<Match> = Vec::new();

    for i in 0..read_bases.len() {
        let substring = &read_bases[i..];
        let (start, _) = st.find_substring(&substring);
        // dbg!(start);
        if let Some(matching_transcriptome) =
            &trans_bases.get(start as usize - i..start as usize - i + read_bases.len()) {//unsure of correctness

            // println!("{}  -  {}", read_bases, matching_transcriptome);
            let errors = edit_distance(read_bases, matching_transcriptome);
            let new_match = Match {
                index: start as usize - i,
                errors: errors,
            };
            matches.push(new_match);
        }
    }
    matches
}

// pub fn align_fragments(
//     frags: &Vec<Fragment>,
//     array: &FragmentArray,
//     transcriptome: &Transcriptome,
// ) -> Vec<Option<(u32, u8)>> {
//     let mut matches: Vec<Option<(u32, u8)>> = Vec::new();

//     let mut _count = 0;

//     for frag in frags {

//         // if count >= 1 {
//         //     break;
//         // }
//         // count += 1;

//         if frag.bases().len() < array.get_size() as usize {
//             // println!("Fragment too short - skipping");
//             continue;
//         }

//         let m = align_fragment(frag, array, transcriptome);

//         // println!("{:?}", m);

//         matches.push(m);
//     }
//     matches
// }
