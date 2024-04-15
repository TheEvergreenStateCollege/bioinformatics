
use rusty_plants::fasta::{analyze::histogram, import};

fn main() {
    let files = import::read_directory("../Sample1RNAControl1").expect("failed to read file");
    let fragments = import::parse_file(&files).expect("failed to parse fragments");
    //println!("{:?}", fragments[0]);

    histogram(&fragments, 100, 100);

}
