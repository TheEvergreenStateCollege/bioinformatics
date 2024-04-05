use std::io::{stdin,BufRead};
use regex_lite::Regex;

fn main() {
    let tokenizer = Regex::new(r"([.,\s]+)").unwrap();

    let mut tokens: Vec<&str>;

    let input stdin().lock();
    let lines = input.lines();

    for line in lines {

    }
}
