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

