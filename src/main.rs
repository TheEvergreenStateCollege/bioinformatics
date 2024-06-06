#![allow(dead_code, unused_variables, unused_imports)]
use bincode::de::read;
use clap::{command, Parser};
use smarty_plants::{
    // algorithms::better_align::align_fragments,
    data_structures::{transcriptome::Transcriptome, suffix_tree::SuffixTree},
    file_io::fasta::*,
};
use std::{fs::{self, read_to_string}, path::PathBuf};

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
    // human_panic::setup_panic!();

    // let read_dir = std::path::Path::new("data/reads/");
    // let files = read_directory_to_string(read_dir).expect("failed to read fragment files");
    // let fragments = parse_file(&files).expect("failed to parse fragments");
    // let genome = parse_genome(read_to_string("data/ref_genome.fna").expect("failed to read genome file"));
    // let transcriptome = Transcriptome::new(&genome);
    //let trans_array = FragmentArray::new(transcriptome.get_bases(), 50);
    //align_fragments(&fragments, &trans_array, &transcriptome);

}
