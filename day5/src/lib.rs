use std::collections::VecDeque;
use eyre::Result;
use regex::Regex;

type Stacks = Vec<VecDeque<char>>;

pub struct Command {
    qty: usize,
    from: usize,
    to: usize,
}

pub fn parse_input(input: &str) -> Result<(Stacks, Vec<Command>)> {
    let (stacks, commands) = input.split_once("\r\n\r\n").unwrap();
    let stacks = parse_stacks(stacks)?;
    let commands = parse_commands(commands.trim());

    Ok((stacks, commands))
}

// Process CrateMover 9000 commands for pt 1
pub fn process_9000_commands(mut stacks: Stacks, commands: &[Command]) -> Stacks {
    commands.iter().for_each(|cmd| {
        let from_stack = stacks.get_mut(cmd.from).unwrap();
        let mut popped: VecDeque<char> = from_stack.split_off(from_stack.len() - cmd.qty);
        popped.make_contiguous().reverse();
        stacks[cmd.to].append(&mut popped);
    });

    stacks
}

// Process CrateMover 9001 commands for pt 2
pub fn process_9001_commands(mut stacks: Stacks, commands: &[Command]) -> Stacks {
    commands.iter().for_each(|cmd| {
        let from_stack = stacks.get_mut(cmd.from).unwrap();
        let mut popped: VecDeque<char> = from_stack.split_off(from_stack.len() - cmd.qty);
        stacks[cmd.to].append(&mut popped);
    });

    stacks
}


pub fn get_top_of_stacks(stacks: &Stacks) -> String {
    stacks.iter().map_while(VecDeque::back).collect()
}

fn parse_stacks(stacks_str: &str) -> Result<Stacks> {
    let mut stacks = Vec::new();
    let mut lines = stacks_str.lines();
    while let Some(line) = lines.next() {
        let chars: Vec<char> = line.trim_end().chars().collect();
        if chars[1] == '1' {
            break;
        }
        let n = (chars.len() + 1) / 4;
        for _ in stacks.len()..n {
            stacks.push(VecDeque::new());
        }

        for i in 0..n {
            let pos_char = i * 4 + 1;
            if chars[pos_char] != ' ' {
                stacks[i].push_front(chars[pos_char]);
            }
        }
    }
    Ok(stacks)
}

fn parse_commands(commands_str: &str) -> Vec<Command> {
    let r = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    commands_str.lines()
        .map(|line| {
            let cap = r.captures(line).unwrap();
            Command {
                qty: cap[1].parse().unwrap(),
                from: cap[2].parse::<usize>().unwrap() - 1,
                to: cap[3].parse::<usize>().unwrap() - 1,
            }
    })
    .collect()
}