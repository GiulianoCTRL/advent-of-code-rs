use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn input() -> Vec<String> {
    //! Read filename (from args) into vector of strings
    let args: Vec<String> = env::args().collect();
    let lines: Vec<String> = BufReader::new(File::open(&args[args.len() - 1]).unwrap())
        .lines()
        .map(|x| String::from(x.unwrap().trim()))
        .collect();
    lines
}
