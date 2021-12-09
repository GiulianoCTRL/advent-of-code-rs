use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn input() -> io::Result<Vec<String>> {
    //! Read filename (from args) into vector of strings
    let args: Vec<String> = env::args().collect();
    let lines: Vec<String> = BufReader::new(File::open(&args[args.len() - 1])?)
        .lines()
        .map(|x| String::from(x.unwrap().trim()))
        .collect();
    Ok(lines)
}
