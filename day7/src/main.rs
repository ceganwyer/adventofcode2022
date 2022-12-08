use eyre::Result;
use std::fs;

use aoc_day7 as AoC;

fn main() -> Result<()> {
    let input = fs::read_to_string("day7/day7.txt")?;
    let tree = AoC::parse_tree(input)?;
    let mut sums = AoC::sum_directories(tree);
    println!("{:?}", sums);
    sums.retain(|_, v| v < &mut 100000);
    println!("{:?}", sums);
    let sum = sums.values().fold(0, |a, b| a + b);
    println!("{:?}", sum);
    Ok(())
}
