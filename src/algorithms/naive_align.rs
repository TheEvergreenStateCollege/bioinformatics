use crate::file_io::fasta::Fragment;

pub fn naive(genome: &str, fragments: Vec<Fragment>) -> Vec<usize> {

    let mut indicies: Vec<usize> = Vec::new();

    for frag in fragments {
        println!("#");
        if let Some(index) = genome.find(&frag.bases()) {
            indicies.push(index);
            println!("\n---Found a match!---");
        }
    }
    indicies
}