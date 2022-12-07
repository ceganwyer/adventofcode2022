use std::fs;
use eyre::Result;

use aoc_day7 as AoC;

fn main() -> Result<()>{
    let input = fs::read_to_string("day7/sample.txt")?;
    let tree = AoC::parse_tree(input)?;
    println!("{:?}", tree);
    let sums = AoC::sum_directories(tree);
    println!("{:?}", sums);
    Ok(())
}
