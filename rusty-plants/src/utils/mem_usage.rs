use sysinfo::{System};
use crate::data_structures::transcriptome::Transcriptome;
use crate::data_structures::suffix_tree::SuffixTree;

pub fn get_safe_memory_limit() -> u64 {
    let sys = System::new_all();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let free_memory = total_memory - used_memory;

    // Use 80% of the available memory as a safe threshold
    let safe_memory_limit = (free_memory as f64 * 0.8) as u64;
    safe_memory_limit

}

pub fn process_chunk(transcriptome: &Transcriptome, st: &mut SuffixTree, start: usize, end: usize) {
    for (i, c) in transcriptome
        .get_bases()
        .chars()
        .map(|x| x as u8)
        .skip(start)
        .take(end - start)
        .enumerate()
    {
        if (start + i) % 1_000_000 == 0 {
            println!("Added up to transcriptome character {} to suffix tree", start + i);
        }
        st.extend(c);
    }

    println!("Finished processing chunk from {} to {}", start, end);
}