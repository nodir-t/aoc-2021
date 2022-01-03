//! Problem: <https://adventofcode.com/2021/day/2#part2>
//!
//! A solution using iterators.

use std::io::{self, BufRead};
use itertools::Itertools;

fn main() {
    match run() {
        Err(err) => eprintln!("Error: {}", err),
        Ok(x) => println!("{}", x)
    }
}

fn run() -> aoc2021::Result<i32> {
    let stdin = io::stdin();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in stdin.lock().lines() {
        let line = line?;
        let (cmd, dist) = line
            .split(' ')
            .collect_tuple::<(&str, &str)>()
            .ok_or_else(|| format!("expected two words in {:?}", &line))?;
        let dist: i32 = dist.parse()?;
        match cmd {
            "down" => aim += dist,
            "up" => aim -= dist,
            "forward" => {
                x += dist;
                y += aim * dist;
            }
            _ => return Err(From::from(format!("unexpected command {:?}", cmd))),
        }
    }

    Ok(x * y)
}
