use eyre::Result;
use std::{fs, time::Instant};

use aoc_day9 as AoC;

fn main() -> Result<()> {
    let now = Instant::now();
    {
        let input = fs::read_to_string("day9/sample2.txt")?;
        let moves = AoC::parse_input(input)?;
        println!("Parsed moves: {:.2?}", now.elapsed());
        let tail_locations = AoC::part_1(&moves);
        println!("Calculated tail locations: {:.2?}", now.elapsed());
        println!("Tail visited {} locations", tail_locations.len());
        let tail_locations = AoC::part_2(&moves, 10)?;
        println!("Calculated tail locations: {:.2?}", now.elapsed());
        println!("Tail visited {} locations", tail_locations.len());
    }
    println!("Total elapsed: {:.2?}", now.elapsed());
    Ok(())
}
