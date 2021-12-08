use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let (ans_i, ans_ii) = get_final_pos(&file_to_vec().expect("Could not read input."));
    println!("Part I: {}, part II: {}", ans_i, ans_ii);
}

fn file_to_vec() -> io::Result<Vec<String>> {
    //! Read filename (from args) into vector of strings
    let args: Vec<String> = env::args().collect();
    let lines: Vec<String> = BufReader::new(File::open(&args[args.len() - 1])?)
        .lines()
        .map(|x| String::from(x.unwrap().trim()))
        .collect();
    Ok(lines)
}

fn get_final_pos(instructions: &[String]) -> (i32, i32) {
    //! Get final x and y position of submarine and return their product
    //! Get aim, x and y and return product of x and d as second tuple item

    // Part I: x, y; Part II: a, x, y
    let (mut x, mut y, mut d) = (0i32, 0i32, 0i32);
    for i in instructions.iter() {
        let cmds: Vec<&str> = i.split(' ').collect();
        let (cmd, value): (&str, i32) = (cmds[0], cmds[1].parse::<i32>().unwrap());
        if cmd == "down" {
            y += value;
        } else if cmd == "up" {
            y -= value;
        } else if cmd == "forward" {
            x += value;
            // Part II
            if y != 0 {
                d += value * y;
            }
        }
    }
    // (Part I, Part II)
    (x * y, x * d)
}
