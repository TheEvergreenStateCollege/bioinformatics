use rusty_plants::file_io::fasta::*;
use std::fs::read_to_string;
use rusty_plants::data_structures::k_mer_array::KMerArray;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {panic!("Usage is fragments_dir genome_path");}

    let files = read_directory_to_string(&args[1]).expect("failed to read fragment files");
    let fragments = parse_file(&files).expect("failed to parse fragments");
    
    let genome = parse_genome(read_to_string(&args[2]).expect("failed to read genome file"));
    let genome_k_mers = KMerArray::new(&genome, 10);

    if let Some(indicies) = genome_k_mers.indicies(&fragments[0].bases()[0..10]) {
        println!("{:?}", indicies);
    }

    // let indicies: Vec<usize> = align::naive(genome, fragments);
    // println!("{:?}", indicies);

    //println!("{:?}", fragments[0]);
    //histogram(&fragments, 100, 100);
}
