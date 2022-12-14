use std::{fs};
use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    use std::time::Instant;
    let now = Instant::now();
    {
        let input = fs::read_to_string("day1/day1.txt")?;

        let elves = parse_into_elves(input)?;

        let mut elves_vec: Vec<(&u32, &u32)> = elves.iter().collect();
        elves_vec.sort_by(|a, b| b.1.cmp(a.1));

        println!("The top three elves are: {:?}, {:?}, {:?}",
                 elves_vec.get(0).unwrap(),
                 elves_vec.get(1).unwrap(),
                 elves_vec.get(2).unwrap());
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

fn parse_into_elves(input: String) -> Result<HashMap<u32, u32>, ParseIntError> {
    let mut elf_count = 1;
    let mut elves = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            elf_count += 1;
            continue;
        }
        let calories: u32 = line.parse()?;
        let elf_calories = elves.entry(elf_count).or_insert(0);
        *elf_calories += calories;
    }

    Ok(elves)
}