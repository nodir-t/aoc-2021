//! Problem: <https://adventofcode.com/2021/day/1#part1>
//!
//! A solution using iterators.


fn main() {
    let mut stdin = aoc2021::StdinParser::new();
    let mut count = 0;
    stdin.i32s().reduce(|x, y| {
        if y > x {
            count += 1
        }
        y
    });
    println!("{}", count);
}
