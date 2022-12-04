// Method for solving part 1
pub fn count_overlapping_assignments(pairs: &[&str]) -> i32 {
    pairs.iter()
        .map(is_overlapping)
        .sum()
}

pub fn count_partial_overlaps(pairs: &[&str]) -> i32 {
    pairs.iter()
        .map(is_partial_overlap)
        .sum()
}

fn is_overlapping(pair: &&str) -> i32 {
    let (first,second) = pair.split_once(',')
                                        .expect("Bad input, no ',' detected!");

    let (first_section_1, first_section_2) = parse_sections(first);
    let (second_section_1, second_section_2) = parse_sections(second);

    ((first_section_1 <= second_section_1 && first_section_2 >= second_section_2) ||
        (second_section_1 <= first_section_1 && second_section_2 >= first_section_2)).into()
}

fn is_partial_overlap(pair: &&str) -> i32 {
    let (first,second) = pair.split_once(',')
                                        .expect("Bad input, no ',' detected!");

    let (first_section_1, first_section_2) = parse_sections(first);
    let (second_section_1, second_section_2) = parse_sections(second);

    (first_section_1 <= second_section_2 && second_section_1 <= first_section_2)
        .into()
}

fn parse_sections(elf: &str) -> (i32, i32) {
    let (section1,section2) = elf.split_once('-')
                                            .expect("Bad input, no '-' detected!");
    let section1: i32 = section1.parse().expect("Bad input, not a number");
    let section2: i32 = section2.parse().expect("Bad input, not a number");

    (section1, section2)
}

