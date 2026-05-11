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

const DEFAULT_MIN_FREE_MEMORY_MB: u64 = 512;
const BYTES_PER_KB: u64 = 1024;
const BYTES_PER_MB: u64 = 1024 * 1024;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// The path to a directory full of .fasta fragments
    fragments_dir: PathBuf,
    /// The path to the genome file to align with
    genome_path: PathBuf,
}

fn min_free_memory_bytes() -> u64 {
    std::env::var("SMARTY_PLANTS_MIN_FREE_MEM_MB")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_MIN_FREE_MEMORY_MB)
        * BYTES_PER_MB
}

fn available_memory_bytes() -> Option<u64> {
    let meminfo = read_to_string("/proc/meminfo").ok()?;
    meminfo.lines().find_map(|line| {
        let mut fields = line.split_whitespace();
        match (fields.next(), fields.next()) {
            (Some("MemAvailable:"), Some(kb)) => {
                kb.parse::<u64>().ok().map(|value| value * BYTES_PER_KB)
            }
            _ => None,
        }
    })
}

fn ensure_enough_memory(position: usize, min_free_bytes: u64) {
    if let Some(available_bytes) = available_memory_bytes() {
        if available_bytes < min_free_bytes {
            eprintln!(
                "Stopping suffix tree construction at transcriptome character {position}: available memory is {} MB, below the configured minimum of {} MB.",
                available_bytes / BYTES_PER_MB,
                min_free_bytes / BYTES_PER_MB,
            );
            std::process::exit(1);
        }
    }
}

fn main() {
    prisma_client_rust_cli::run();

    let read_dir = std::path::Path::new("../data/reads/");
    let files = read_directory_to_string(read_dir).expect("failed to read fragment files");
    let fragments = parse_file(&files).expect("failed to parse fragments");
    let genome =
        parse_genome(read_to_string("../data/ref_genome.fna").expect("failed to read genome file"));
    let transcriptome = Transcriptome::new(&genome);
    let mut st = suffix_tree::SuffixTree::new();
    let min_free_bytes = min_free_memory_bytes();
    for (i, c) in transcriptome
        .get_bases()
        .chars()
        .map(|x| x as u8)
        .take(80_000_000)
        .enumerate()
    {
        if i % 1_000_000 == 0 {
            ensure_enough_memory(i, min_free_bytes);
            println!("Added up to transcriptome character {} to suffix tree", i);
        }
        st.extend(c);
    }

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
}
