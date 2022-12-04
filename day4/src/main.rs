use std::error::Error;
use std::fs;

use aoc_day4 as AoC;

fn main() -> Result<(), Box<dyn Error>> {
    use std::time::Instant;
    let now = Instant::now();
    {
        let input = fs::read_to_string("day4/day4.txt")?;
        let pairs: Vec<&str> = input.lines().collect();
        let overlap_count = AoC::count_overlapping_assignments(&pairs);
        println!("There are {} overlapping assignments", overlap_count);
        let partial_overlap_count = AoC::count_partial_overlaps(&pairs);
        println!("There are {} partially overlapping assignments", partial_overlap_count)
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
