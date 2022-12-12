use eyre::Result;
use std::{fs, time::Instant};

use aoc_day11 as AoC;

fn main() -> Result<()> {
    let now = Instant::now();
    {
        let input = fs::read_to_string("day11/sample.txt")?;
        let monkeys = AoC::parse_input(input)?;
        println!("Parsed input: {:.2?}", now.elapsed());
        println!("Monkeys: {:?}", monkeys);
    }
    println!("Total elapsed: {:.2?}", now.elapsed());
    Ok(())
}
