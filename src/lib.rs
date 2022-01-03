use std::io::{self, BufRead};

pub struct StdinParser {
    stdin: io::Stdin,
}

impl StdinParser {
    pub fn new() -> StdinParser {
        StdinParser{stdin: std::io::stdin()}
    }
}

impl StdinParser {
    pub fn i32s<'a>(&'a mut self) -> impl Iterator<Item = i32> + 'a {
        self.stdin.lock().lines()
            .map(|s| s.unwrap().trim().parse::<i32>().expect("failed to parse item"))
    }
}
