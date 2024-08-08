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
    fs::File,
};
use std::io::{BufWriter, Write};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// The path to a directory full of .fasta fragments
    fragments_dir: PathBuf,
    /// The path to the genome file to align with
    genome_path: PathBuf,
}

fn process_chunk(
    chunk: &str,
    fragments: &[Fragment],
    transcriptome: &Transcriptome,
) -> Vec<Match> {
    let mut st = SuffixTree::new();
    for c in chunk.chars().map(|x| x as u8) {
        st.extend(c);
    }

    let mut best_matches = Vec::new();

    for fragment in fragments {
        let matches = align_fragment(fragment, &st, &transcriptome);
        if let Some(best_match) = matches.iter().min_by_key(|m| m.errors) {
            best_matches.push(best_match.clone());
        }
    }

    best_matches
}

fn save_matches_to_file(matches: &[Match], filename: &str) {
    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for m in matches {
        writeln!(
            writer,
            "Index: {}, Errors: {}",
            m.index, m.errors
        ).expect("Unable to write data to file");
    }
}

fn main() {
    prisma_client_rust_cli::run();

    let read_dir = std::path::Path::new("../data/reads/");
    let files = read_directory_to_string(read_dir).expect("failed to read fragment files");
    let fragments = parse_file(&files).expect("failed to parse fragments");
    let genome = parse_genome(read_to_string("../data/ref_genome.fna").expect("failed to read genome file"));
    let transcriptome = Transcriptome::new(&genome);
    
    let chunk_size = 2_000_000;
    let genome_length = transcriptome.get_bases().len();
    let mut all_best_matches = Vec::new();

    for chunk_start in (0..std::cmp::min(80_000_000, genome_length)).step_by(chunk_size) {
        let chunk_end = std::cmp::min(chunk_start + chunk_size, genome_length);
        let chunk = &transcriptome.get_bases()[chunk_start..chunk_end];

        let best_matches = process_chunk(chunk, &fragments, &transcriptome);
        all_best_matches.extend(best_matches);

        // Save the current chunk results to a file
        let filename = format!("chunk_{}_{}.txt", chunk_start, chunk_end);
        save_matches_to_file(&all_best_matches, &filename);

        // Clear memory by dropping the current chunk's results
        all_best_matches.clear();

        println!("Processed and saved chunk {} - {}", chunk_start, chunk_end);
    }

    println!("All chunks processed.");
}