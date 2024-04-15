use rusty_plants::fasta::{analyze::histogram, parse_file, read_directory};

fn main() {
    let files = read_directory("../Sample1RNAControl1").expect("failed to read file");
    let fragments = parse_file(&files).expect("failed to parse fragments");
    //println!("{:?}", fragments[0]);

    histogram(&fragments, 100, 100);
}
