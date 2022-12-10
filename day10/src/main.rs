use eyre::Result;
use std::{fs, time::Instant};

use aoc_day10 as AoC;

fn main() -> Result<()> {
    let now = Instant::now();
    {
        let input = fs::read_to_string("day10/sample.txt")?;
        let commands = AoC::parse_input(input)?;
        println!("Parsed commands: {:.2?}", now.elapsed());
        let signal_strengths = AoC::part_1(&commands);
        println!("Part 1 complete: {:.2?}", now.elapsed());
        println!("Sum of signal strengths: {}", signal_strengths);
        AoC::part_2(&commands)?;
        println!("Part 2 complete: {:.2?}", now.elapsed());
    }
    println!("Total elapsed: {:.2?}", now.elapsed());
    Ok(())
}
