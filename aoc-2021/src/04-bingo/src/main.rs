#![allow(unused_variables)]

use std::env;
use std::fs::File;
use std::io::Read;

mod bingo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut content = String::new();
    File::open(&args[args.len() - 1])
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let (num_str, board_str) = content.split_once("\n\n").unwrap();
    let nums: Vec<u32> = num_str
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let boards: Vec<&str> = board_str.split("\n\n").collect();
    bingo::to_boards(&boards);
}
