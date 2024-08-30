#![allow(dead_code, unused_variables, unused_imports)]
use bincode::de::read;
use clap::{command, Parser};
use smarty_plants::{
    algorithms::read_align::{align_fragment, Match},
    data_structures::{
        suffix_tree::{self, SuffixTree},
        transcriptome::Transcriptome,
    },
    file_io::fasta::*,
};
use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};
use smarty_plants::utils::mem_usage::get_safe_memory_limit;


#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// The path to a directory full of .fasta fragments
    fragments_dir: PathBuf,
    /// The path to the genome file to align with
    genome_path: PathBuf,
}

fn process_chunk(transcriptome: &Transcriptome, st: &mut SuffixTree, start: usize, end: usize) {
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


fn main() {
    prisma_client_rust_cli::run();

    let safe_memory_limit = get_safe_memory_limit();
    println!("Safe memory limit: {} KB", safe_memory_limit);

    let read_dir = std::path::Path::new("../data/reads/");
    let files = read_directory_to_string(read_dir).expect("failed to read fragment files");
    let fragments = parse_file(&files).expect("failed to parse fragments");
    let genome = parse_genome(read_to_string("../data/ref_genome.fna").expect("failed to read genome file"));
    let transcriptome = Transcriptome::new(&genome);

    // Calculate max_chars and total_bases
    let max_chars = (safe_memory_limit * 1024) / (std::mem::size_of::<u8>() as u64) / 2; // Reduce chunk size
    let total_bases = transcriptome.get_bases().chars().count();
    let num_chunks = (total_bases as f64 / max_chars as f64).ceil() as usize;

    let mut st = SuffixTree::new();
    let mut sys = sysinfo::System::new_all();
    

    for chunk in 0..num_chunks {
        let start = chunk * max_chars as usize;
        let end = ((chunk + 1) * max_chars as usize).min(total_bases);

        println!("Processing chunk from {} to {}", start, end);
        helper::process_chunk(&transcriptome, &mut st, start, end);

        sys.refresh_memory();
        println!("Memory usage after processing chunk {}: {} KB", chunk, sys.used_memory());

        println!("Aligning fragments");
        for i in 0..fragments.len() {
            let matches = align_fragment(&fragments[i], &st, &transcriptome);
            let mut best: &Match = &matches[0];
            for m in matches.iter() {
                if m.errors < best.errors {
                    best = &m;
                }
            }
            println!("{:?}, read_length: {}", best, &fragments[i].bases().len());
        }

        drop(st);
        std::thread::sleep(std::time::Duration::from_secs(5));

        sys.refresh_memory();
        println!("Memory after dropping suffix tree: {} KB", sys.used_memory());
    }
}

