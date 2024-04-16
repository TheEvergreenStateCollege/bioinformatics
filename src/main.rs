use rusty_plants::fasta::{
    align, parse_file, parse_genome, read_directory_to_string};
use std::fs::read_to_string;
use rusty_plants::suffix::SuffixArray;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // if args.len() < 2 {panic!("pants shidded - usage is fragments_dir genome_path");}

    // let files = read_directory_to_string(&args[1]).expect("failed to read fragment files");

    // let fragments = parse_file(&files).expect("failed to parse fragments");
    let genome = parse_genome(read_to_string(&args[1]).expect("failed to read genome file"));
    SuffixArray::new(&genome, 10);

    // let indicies: Vec<usize> = align::naive(genome, fragments);
    // println!("{:?}", indicies);

    //println!("{:?}", fragments[0]);
    //histogram(&fragments, 100, 100);
}
