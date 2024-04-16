use rusty_plants::fasta::{
    align, analyze::histogram, parse_file, parse_genome, read_directory_to_string};
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let files = read_to_string(&args[1]).expect("failed to read fragment files");
    let fragments = parse_file(&files).expect("failed to parse fragments");
    let genome = parse_genome(read_to_string(&args[2]).expect("failed to read genome file"));

    let indicies: Vec<usize> = align::naive(genome, fragments);
    println!("{:?}", indicies);

    //println!("{:?}", fragments[0]);
    //histogram(&fragments, 100, 100);
}
