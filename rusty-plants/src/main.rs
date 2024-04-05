use std::io::{stdin,BufRead};
use regex_lite::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = std::fs::read_to_string(&args[1]).unwrap();
    let tokenizer = Regex::new(r"([.,\s]+)").unwrap();

    let mut tokens: Vec<&str>;

    let lines = contents.lines();

    for line in lines {
        println!("{:?}",line);
    }
}
