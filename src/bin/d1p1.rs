//! Problem: <https://adventofcode.com/2021/day/1#part1>
//!
//! A solution using iterators.

fn main() {
    let mut stdin = aoc2021::StdinParser::new();
    println!("{}",     
        stdin.i32s()
            .with_prev()
            .filter(|(x, y)| y > x)
            .count()
    );
}
