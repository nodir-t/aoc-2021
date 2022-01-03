//! Problem: <https://adventofcode.com/2021/day/1#part1>
//!
//! A solution using iterator folding.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines()
        .map(|s| s.unwrap().trim().parse().expect("not a number"));
    let first: i32 = iter.next().expect("Expected at least one number");
    let (count, _) = iter.fold((0, first), |(count, prev), x| {
        (count + (if x > prev {1} else {0}), x)
    });
    println!("{}", count);
}
