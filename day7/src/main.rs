use eyre::Result;
use std::fs;

use aoc_day7 as AoC;

fn main() -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    {
        let input = fs::read_to_string("day7/day7.txt")?;
        let tree = AoC::parse_tree(input)?;
        let mut sums = AoC::sum_directories(tree);
        sums.retain(|_, v| v <= &mut 100000);
        let sum = sums.values().fold(0, |a, b| a + b);
        println!("{:?}", sum);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
