use super::Fragment;
use std::fmt::Write;

pub fn histogram(fragments: &Vec<Fragment>, bin_size: usize, max_width: usize) {
    let mut bins: Vec<usize> = Vec::new();

    for fragment in fragments {
        let bin_index: usize = fragment.bases.len() / bin_size;
        while bin_index >= bins.len() {
            bins.push(0);
        }
        bins[bin_index] += 1;
    }

    //Finds the maximum margin text length, so all of the margin will display as the same width
    let margin_width = (bins.len() * bin_size).to_string().len() * 2 + 3;

    //Finds max width of bar in order to adjust width to fit.
    let max_bin_size = bins.iter().max().unwrap();
    let mut squeeze: usize = 1;
    while max_bin_size / squeeze > max_width {
        squeeze += 1;
    }

    for i in 0..bins.len() {
        //Margin
        let mut margin: String = String::new();
        write!(&mut margin, "{}-{}: ", i * bin_size, (i + 1) * bin_size).unwrap();
        while margin.len() < margin_width {
            margin.push(' ');
        }
        print!("{}", margin);

        //Bars
        for j in 0..bins[i] {
            if j % squeeze == 0 {
                //This works because 1/n of all whole numbers are divisible by n (evenly distributed)
                print!("|");
            }
        }
        println!();
    }
}
