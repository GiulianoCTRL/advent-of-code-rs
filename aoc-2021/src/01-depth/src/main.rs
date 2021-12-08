use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let nums: Vec<i32> = BufReader::new(File::open(&args[args.len() - 1])?)
        .lines()
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect();

    // part I
    let count = part_i(&nums);
    println!("Count part I: {}", count);

    // part II
    let count = part_ii(&nums);
    
    println!("Count part II: {}", count);
    Ok(())
}


fn part_i(nums: &[i32]) -> i32 {
    //! Iterate through slice of integers and add to count if the previous
    //! integer was higher than the current one
    let mut count = 0;
    for (i, _) in nums.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if nums[i] > nums[i - 1] {
            count += 1;
        }
    }
    count
}


fn part_ii(nums: &[i32]) -> i32 {
    //! Combine nums in pairs of three and iterate through them and
    //! add to count if the previous integer was higher than the current one
    let mut sums: Vec<i32> = Vec::new();
    let mut count = 0;
    for (i, _) in nums.iter().enumerate() {
        if i < 2 {
            continue;
        }
        sums.push(nums[i -2] + nums[i - 1]+ nums[i]);
    }
    for (i, _) in sums.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if sums[i] > sums[i - 1] {
            count += 1;
        }
    }
    count
}