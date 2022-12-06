use std::fs;
use eyre::{Result, WrapErr};

use aoc_day6 as AoC;

fn main() -> Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    {
        let input = fs::read_to_string("day6/day6.txt")
            .wrap_err("Failed to read input file!")?;
        let signals: Vec<&str> = input.lines().collect();
        let packet_markers = AoC::find_packet_markers(&signals);
        println!("Found packet markers: {:?}", packet_markers);
        let message_markers = AoC::find_message_markers(&signals);
        println!("Found message markers: {:?}", message_markers);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
