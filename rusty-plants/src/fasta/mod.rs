
#[allow(dead_code)]
#[derive(Debug)]
pub struct Fragment<'a> { //Same as a read. I chose to use this name because Read is taken by io::Read
    hash: &'a str, //I don't actually know what this part is, so this is a guess
    runid: &'a str,
    sampleid: &'a str,
    read_number: &'a str,
    ch: &'a str,
    start_time: &'a str,
    model_version_id: &'a str,
    pub bases: &'a str,
}

pub mod import {
    use std::io::{self};
    use std::io::{Error, ErrorKind};
    use std::fs::{read_to_string,read_dir};
    use crate::fasta::Fragment;

    pub fn read_file(path: &str) -> Result<String, io::Error> {
        read_to_string(path)
    }

    pub fn read_directory(path: &str) -> Result<String, io::Error> {
        let dir_paths = read_dir(path)?;
        let mut all_contents: String = String::new();

        for dir_path in dir_paths {
            let contents = read_file(dir_path.unwrap().path().to_str().unwrap())?;
            all_contents.push_str(&contents);
        }
        Ok(all_contents)
    }

    pub fn parse_file(contents: &str) -> Result<Vec<Fragment>, io::Error> {
        
        let mut fragments: Vec<Fragment> = Vec::new();

        for read in contents.split('>') { //Each read in prefaced with a carrot
            if read.is_empty() { continue } //This skips the empty slice which is at the start

            let lines: Vec<&str> = read.split('\n').collect(); //This makes three slices: the first line (metadata), the second (bases), and an empty slice
            if lines.len() < 2 { return Err(Error::new(ErrorKind::InvalidData, "invalid lines")); }

            let tokens: Vec<&str> = lines[0].split(' ').collect();
            if tokens.len() != 7 { return Err(Error::new(ErrorKind::InvalidData, "invalid tokens")); }

            fragments.push(Fragment {
                hash: tokens[0],
                runid: tokens[1].split('=').nth(1).unwrap_or("failed_to_parse"),
                sampleid: tokens[2].split('=').nth(1).unwrap_or("failed_to_parse"),
                read_number: tokens[3].split('=').nth(1).unwrap_or("failed_to_parse"),
                ch: tokens[4].split('=').nth(1).unwrap_or("failed_to_parse"),
                start_time: tokens[5].split('=').nth(1).unwrap_or("failed_to_parse"),
                model_version_id: tokens[6].split('=').nth(1).unwrap_or("failed_to_parse"),
                bases: lines[1],
            });

        }
        Ok(fragments)
    }
}

//example lines
//>4c6bc618-e920-44b4-92ab-642f2d535cf0 runid=9d742d72b6f5d334c2d0d388f2eb1da13decd9a6 sampleid=Plant_Memory_RNA_1 read=55292 ch=490 start_time=2023-05-19T10:33:23Z model_version_id=2020-09-07_rna_r9.4.1_minion_256_8f8fc47b
//GCUAUGAUGUCUAAAGUUUACGCUAGAUCCGUACGACUCCGUGGUAACCCAACCGUCGAAGUCGAAUUAACUACCGAAAAGGGUGUUUCAGAUCCAUUGUUCCAUCUGGUGCCUCACACCGGUGUCCACGAAGCUUUGGAAAUGAGAGAUGAAGACAAAUCCAAGUGGAUGGGUAAGGGUGUUAUGAACGCUGCUCAACAACGUCAACAACGUCAUUAUUG

pub mod analyze {
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
            write!(&mut margin, "{}-{}: ", i * bin_size, (i+1) * bin_size).unwrap();
            while margin.len() < margin_width {
                margin.push(' ');
            }
            print!("{}", margin);

            //Bars
            for j in 0..bins[i] {
                if j % squeeze == 0 { //This works because 1/n of all whole numbers are divisible by n (evenly distributed)
                    print!("|");
                }
            }
            println!("");
        }
    }
}