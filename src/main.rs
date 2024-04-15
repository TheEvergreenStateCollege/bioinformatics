use rusty_plants::fasta::{analyze::histogram, parse_file, read_directory_to_string};
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let files = read_to_string(&args[1]).expect("failed to read file");
    let fragments = parse_file(&files).expect("failed to parse fragments");
    //println!("{:?}", fragments[0]);

    histogram(&fragments, 100, 100);
}
