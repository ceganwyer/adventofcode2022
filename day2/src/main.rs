use std::{fs};
use std::error::Error;

use aoc_day2 as AoC;

fn main() -> Result<(), Box<dyn Error>> {
    use std::time::Instant;
    let now = Instant::now();
    {
        let input = fs::read_to_string("day2/day2.txt")?;

        let rounds: Vec<&str> = input.lines().collect();

        let rounds = AoC::parse_tournament(&rounds)?;

        let (opp, user) = AoC::eval_tournament(rounds);

        println!("Final score: {} - {}", opp, user);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    
    Ok(())
}
