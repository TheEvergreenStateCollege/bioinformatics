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

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// The path to a directory full of .fasta fragments
    fragments_dir: PathBuf,
    /// The path to the genome file to align with
    genome_path: PathBuf,
}

fn main() {

    let read_dir = std::path::Path::new("../data/reads/");
    let files = read_directory_to_string(read_dir).expect("failed to read fragment files");
    let fragments = parse_file(&files).expect("failed to parse fragments");
    let genome =
        parse_genome(read_to_string("../data/ref_genome.fna").expect("failed to read genome file"));
    let transcriptome = Transcriptome::new(&genome);
    let mut st = suffix_tree::SuffixTree::new();
    for (i, c) in transcriptome
        .get_bases()
        .chars()
        .map(|x| x as u8)
        .take(80_000_000)
        .enumerate()
    {
        if i % 1_000_000 == 0 {
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
