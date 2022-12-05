use std::fs;

use aoc_day5 as AoC;
use eyre::{WrapErr, Result};

fn main() -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    {
        let input = fs::read_to_string("day5/day5.txt")
            .wrap_err("Failed to read input file!")?;
        let (stacks, commands) = AoC::parse_input(&input.as_str())?;
        println!("Parsed input. Elapsed: {:.2?}", now.elapsed());
        let stacks = AoC::process_9000_commands(stacks, &commands);
        println!("Processed commands. Elapsed: {:.2?}", now.elapsed());
        let stack_tops = AoC::get_top_of_stacks(&stacks);
        println!("Top of stacks: {:?}", stack_tops);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
