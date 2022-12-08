use std::fs;
use std::time::Instant;
use eyre::Result;

use aoc_day8 as AoC;

fn main() -> Result<()> {
    let now = Instant::now();
    {
        let input = fs::read_to_string("day8/day8.txt")?;
        println!("Read input from file: {:.2?}", now.elapsed());
        let grid = AoC::parse_grid(input.as_str())?;
        println!("Parsed grid from input: {:.2?}", now.elapsed());
        let visible = AoC::find_visible_trees(&grid);
        println!("Searched for visible trees: {:.2?}", now.elapsed());
        println!("Num Visible: {:?}", visible.len());
        let max_score = AoC::find_max_scenic_score(&grid)?;
        println!("Calculated max scenic score: {:.2?}", now.elapsed());
        println!("Max score: {:?}", max_score);
    }
    println!("Total Elapsed: {:.2?}", now.elapsed());
    Ok(())
}
