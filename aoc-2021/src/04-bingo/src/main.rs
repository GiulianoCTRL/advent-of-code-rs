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
    bingo::announce_winners(&content);
}
