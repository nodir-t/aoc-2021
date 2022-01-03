use aoc2021::IterExt;

fn main() {
    let mut stdin = aoc2021::StdinParser::new();
    let sums = stdin.i32s()
        .with_prev()
        .with_prev()
        .map(|((a, b), (_b, c))| a + b + c);
    println!("{}", 
        IterExt::new(sums)
            .with_prev()
            .filter(|(s1, s2)| s2 > s1)
            .count());    
}
