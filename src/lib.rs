use std::io::{self, BufRead};

pub struct IterExt<I: Iterator> {
    iter: I
}

impl<T, I> IterExt<I> 
where
    T: Copy,
    I: Iterator<Item = T>,
{
    pub fn new(iter: I) -> IterExt<I> {
        IterExt{iter: iter}
    }

    pub fn with_prev(self) -> IterExt<impl Iterator<Item=(T, T)>> {
        let mut prev: Option<T> = None;
        IterExt::new(self.iter.filter_map(move |x| {
            let result = prev.map(|prev| (prev, x));
            prev = Some(x);
            result
        }))
    }
}

impl<T, I> Iterator for IterExt<I> 
where
    T: Copy,
    I: Iterator<Item = T>
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
}


pub struct StdinParser {
    stdin: io::Stdin,
}

impl StdinParser {
    pub fn new() -> StdinParser {
        StdinParser{stdin: std::io::stdin()}
    }
}

impl StdinParser {
    pub fn i32s<'a>(&'a mut self) -> IterExt<impl Iterator<Item=i32> + 'a> {
        IterExt::new(
            self.stdin.lock().lines()
            .map(|s| s.unwrap().trim().parse::<i32>().expect("failed to parse item"))
        )
    }
}


