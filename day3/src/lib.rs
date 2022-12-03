use std::collections::{HashMap, HashSet};

// Split each sack into it's two compartments, find their common values, and return the sum of
//      the priority of all found values.
pub fn process_sacks(sacks: &[&str]) -> Result<u32, String> {
    let mut sum: u32 = 0;
    sacks.iter().for_each(|sack| {
        let (first, second) = sack.split_at(sack.len() / 2);
        let first_set: HashSet<char> = HashSet::from_iter(first.chars());
        let second_set: HashSet<char> = HashSet::from_iter(second.chars());
        let common = find_common_item(first_set, second_set).unwrap();
        sum += get_value(common);
    });

    Ok(sum)
}

// Find every group badge in all of the sacks (one group is a set of 3 sacks) and return the sum
//      of the priority of all badges
pub fn find_badges(sacks: &[&str]) -> Result<u32, String> {
    let groups = sacks.chunks(3);
    let mut badge_sum = 0;
    groups.for_each(|group| {
        let mut badges: HashMap<char, u32> = HashMap::new();
        group.iter().for_each(|sack| {
            let sack_set: HashSet<char> = HashSet::from_iter(sack.chars());
            sack_set.iter().for_each(|item| {
                let entry = badges.entry(*item).or_insert(0);
                *entry += 1;
            });
        });
        let group_badge = badges.iter().find(|&b| *b.1 == 3).unwrap();
        badge_sum += get_value(*group_badge.0);
    });
    Ok(badge_sum)
}

// Find the item that is shared by two compartments in a sack
fn find_common_item(first: HashSet<char>, second: HashSet<char>) -> Result<char, String> {
    if let Some(common) = first.intersection(&second).next() {
        return Ok(*common)
    }
    Err(String::from("No common item found!"))
}

// Convert a char in to a priority value (a-z = 1-26; A-Z = 27-52)
fn get_value(item: char) -> u32 {
    if item.is_uppercase() {
        item as u32 - 38
    } else {
        item as u32 - 96
    }
}