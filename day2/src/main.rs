use std::{fs};
use std::error::Error;

use aoc_day2 as AoC;
use aoc_day2::eval_tournament;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day2/day2.txt")?;

    let rounds: Vec<&str> = input.lines().collect();

    let rounds = AoC::parse_tournament(&rounds)?;

    let (opp, user) = eval_tournament(rounds);

    println!("Final score: {} - {}", opp, user);

    Ok(())
}
