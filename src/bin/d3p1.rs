//! Problem: <https://adventofcode.com/2021/day/3#part1>
//!
//! A solution using iterators.

use std::io::{self, BufRead};

fn main() {
    match run() {
        Err(err) => eprintln!("Error: {}", err),
        Ok(x) => println!("{}", x)
    }
}

fn run() -> aoc2021::Result<u32> {
    let stdin = io::stdin();

    const MAX_BITS: usize = 12;
    let mut total = 0;
    let mut ones = [0; MAX_BITS];
    for num in stdin.lock().lines() {
        total += 1;

        let num = i32::from_str_radix(&num?, 2)?;
        for bit in 0..MAX_BITS {
            if num & (1<<bit) != 0 {
                ones[bit] += 1;
            }
        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for bit in 0..MAX_BITS {
        let mask = 1 << bit;
        let ones = ones[bit];
        let zeros = total - ones;
        if ones > zeros {
            gamma |= mask;
        } else {
            epsilon |= mask;
        }
    }
    Ok(gamma * epsilon)
}
