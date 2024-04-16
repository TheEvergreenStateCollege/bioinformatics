use super::Fragment;

pub fn naive(genome: String, fragments: Vec<Fragment>) -> Vec<usize> {

    let mut indicies: Vec<usize> = Vec::new();

    for frag in fragments {
        print!("#");
        if let Some(index) = genome.find(&frag.bases().replace("U", "T")) {
            indicies.push(index);
            println!("\n---Found a match!---");
        }
    }
    indicies
}