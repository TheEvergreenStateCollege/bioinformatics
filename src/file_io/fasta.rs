use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, read_to_string, File},
    io::prelude::*,
    path::Path,
};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
// Same as a read. I chose to use this name because Read is taken by io::Read
pub struct Fragment<'a> {
    uid: &'a str, // I don't actually know what this part is, so this is a guess
    runid: &'a str,
    sampleid: &'a str,
    read_number: &'a str,
    ch: &'a str,
    start_time: &'a str,
    model_version_id: &'a str,
    bases: &'a str,
}

impl Fragment<'_> {
    pub fn bases(&self) -> &str {
        self.bases
    }

    pub fn serialize(&self, name: &str) {
        let encoded = bincode::serialize(self).unwrap();

        let base_dirs = BaseDirs::new().unwrap();
        let mut file =
            File::create(base_dirs.data_dir().join(format!("{}.bincode", name))).unwrap();

        file.write_all(&encoded).unwrap();
    }
}

/// Adds the contents of every file in the given directory to a string, and returns it.
pub fn read_directory_to_string(path: &Path) -> Result<String, std::io::Error> {
    let mut all_contents = String::new();

    for dir_path in read_dir(path)? {
        let contents = read_to_string(dir_path?.path().to_str().unwrap())?;
        all_contents.push_str(&contents);
    }
    //Uracil to Thymine conversion so the reads can be matched to DNA
    let all_contents = all_contents.replace('U', "T");
    println!("Reads imported from files");
    Ok(all_contents)
}

pub fn parse_file(contents: &str) -> Result<Vec<Fragment>, std::io::Error> {
    let mut fragments: Vec<Fragment> = Vec::new();

    // Each read in prefaced with a carrot
    // This skips the empty slice which is at the start
    for read in contents.split('>').skip(1) {
        // This makes three slices: the first line (metadata), the second (bases), and an empty slice
        let lines: Vec<&str> = read.split('\n').collect();
        if lines.len() < 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid data. Not enough lines",
            ));
        }

        let tokens: Vec<&str> = lines[0].split(' ').collect();
        if tokens.len() != 7 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid tokens",
            ));
        }

        fragments.push(Fragment {
            uid: tokens[0],
            runid: tokens[1].split('=').nth(1).expect("Failed_to_parse"),
            sampleid: tokens[2].split('=').nth(1).expect("Failed_to_parse"),
            read_number: tokens[3].split('=').nth(1).expect("Failed_to_parse"),
            ch: tokens[4].split('=').nth(1).expect("Failed_to_parse"),
            start_time: tokens[5].split('=').nth(1).expect("Failed_to_parse"),
            model_version_id: tokens[6].split('=').nth(1).expect("Failed_to_parse"),
            bases: lines[1],
        });
    }
    println!("Reads parsed");
    Ok(fragments)
}

//example lines
//>4c6bc618-e920-44b4-92ab-642f2d535cf0 runid=9d742d72b6f5d334c2d0d388f2eb1da13decd9a6 sampleid=Plant_Memory_RNA_1 read=55292 ch=490 start_time=2023-05-19T10:33:23Z model_version_id=2020-09-07_rna_r9.4.1_minion_256_8f8fc47b
//GCUAUGAUGUCUAAAGUUUACGCUAGAUCCGUACGACUCCGUGGUAACCCAACCGUCGAAGUCGAAUUAACUACCGAAAAGGGUGUUUCAGAUCCAUUGUUCCAUCUGGUGCCUCACACCGGUGUCCACGAAGCUUUGGAAAUGAGAGAUGAAGACAAAUCCAAGUGGAUGGGUAAGGGUGUUAUGAACGCUGCUCAACAACGUCAACAACGUCAUUAUUG

pub fn parse_genome(fna: String) -> String {
    let genome: String = fna.split('\n').skip(1).collect::<String>();
    println!("Genome imported from file");
    genome
}
