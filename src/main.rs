#![allow(dead_code, unused_variables, unused_imports)]
use clap::{command, Parser};
use rusty_plants::{
    algorithms::better_align::align_fragments,
    data_structures::{k_mer_array::FragmentArray, transcriptome::Transcriptome, suffix_trees::SuffixTree},
    file_io::fasta::*,
};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// The path to a directory full of .fasta fragments
    fragments_dir: PathBuf,
    /// The path to the genome file to align with
    genome_path: PathBuf,
}

fn main() {
    // let cli = Cli::parse();

    human_panic::setup_panic!();

    // let files =
    //     read_directory_to_string(&cli.fragments_dir).expect("failed to read fragment files");
    // let fragments = parse_file(&files).expect("failed to parse fragments");

    // let genome = parse_genome(read_to_string(cli.genome_path).expect("failed to read genome file"));
    // let transcriptome = Transcriptome::new(&genome);
    //let trans_array = FragmentArray::new(transcriptome.get_bases(), 50);
    //align_fragments(&fragments, &trans_array, &transcriptome);

    // xaccxaca
    let st = SuffixTree::new("xaccxaca");
    //println!("{}", &st);
    //println!("{:?}", st.find_substring("❤️"));
}

// How to run:
// cargo run --release  -- .\data\reads\ .\data\ref_genome.fna