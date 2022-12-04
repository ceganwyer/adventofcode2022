use std::error::Error;
use std::fs;

use aoc_day3 as AoC;

fn main() -> Result<(), Box<dyn Error>> {
        use std::time::Instant;
        let now = Instant::now();
        {
                let input = fs::read_to_string("day3/day3.txt")?;
                let lines: Vec<&str> = input.lines().collect();
                let common_sum = AoC::process_sacks(&lines)?;
                println!("Sum of common values: {}", common_sum);
                let badge_sum = AoC::find_badges(&lines)?;
                println!("Sum of found badges: {}", badge_sum);
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
