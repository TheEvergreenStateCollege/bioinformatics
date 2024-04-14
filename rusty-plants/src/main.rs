
use rusty_plants::fasta::import;

fn main() {
    let test_file = import::read_file("../fastq_runid_9d742d72b6f5d334c2d0d388f2eb1da13decd9a6_0_0.fasta").expect("failed to read file");
    let fragments = import::parse_file(&test_file).expect("failed to parse fragments");
    println!("{:?}", fragments[0]);

}
