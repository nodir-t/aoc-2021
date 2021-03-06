use std::io::{self, BufRead};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct IterExt<I: Iterator>(I);

impl<T, I> IterExt<I> 
where
    T: Copy,
    I: Iterator<Item = T>,
{
    pub fn new(iter: I) -> IterExt<I> {
        IterExt(iter)
    }

    pub fn with_prev(self) -> IterExt<impl Iterator<Item=(T, T)>> {
        let mut prev: Option<T> = None;
        IterExt::new(self.0.filter_map(move |x| {
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
        self.0.next()
    }
}


pub struct StdinParser {
    stdin: io::Stdin,
}

impl StdinParser {
    pub fn new() -> StdinParser {
        StdinParser{stdin: std::io::stdin()}
    }
    
    pub fn i32s<'a>(&'a mut self) -> IterExt<impl Iterator<Item=i32> + 'a> {
        self.seq::<i32>()
    }

    pub fn seq<'a, T>(&'a mut self) -> IterExt<impl Iterator<Item=T> + 'a>
    where
        T: std::str::FromStr + Copy,
        T::Err: std::fmt::Debug,
    {
        IterExt::new(
            self.stdin.lock().lines()
            .map(|s| s.unwrap().trim().parse::<T>().expect("failed to parse item"))
        )
    }
}
