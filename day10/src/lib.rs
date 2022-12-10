#![allow(dead_code, unused)]
use std::{collections::VecDeque, fmt};

use eyre::{eyre, Result};

#[derive(Debug)]
pub enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    fn get_cycles(&self) -> u8 {
        match self {
            Instruction::Noop => 0,
            Instruction::AddX(_) => 1,
        }
    }

    fn get_value(&self) -> i32 {
        match self {
            Instruction::Noop => 0,
            Instruction::AddX(val) => *val,
        }
    }
}

pub fn parse_input(input: String) -> Result<Vec<Instruction>> {
    let mut commands = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let command = parts.next().ok_or_else(|| eyre!("Unrecognized input!"))?;
        match command {
            "addx" => {
                let val = parts.next().ok_or_else(|| eyre!("Addx missing value!"))?;
                let val = val.parse::<i32>()?;
                commands.push(Instruction::AddX(val));
            }
            "noop" => commands.push(Instruction::Noop),
            _ => panic!("Unrecognized command"),
        }
    }
    Ok(commands)
}

// Solution for part 1
pub fn part_1(commands: &[Instruction]) -> i32 {
    let mut register = 1;
    let mut cycle_count = 1;
    let mut cmds = commands.iter();
    let mut current_command = cmds.next();
    let mut command_cycles = current_command.unwrap().get_cycles();
    let mut signal_strengths = 0;
    while cmds.len() > 0 || !current_command.is_none() {
        if (cycle_count == 20 || f64::from(cycle_count - 20) / 40.0 % 1.0 == 0.0)
            && cycle_count <= 220
        {
            println!("Cycle {}: {}", cycle_count, register);
            signal_strengths += register * cycle_count;
        }
        if command_cycles > 0 {
            command_cycles -= 1;
            cycle_count += 1;
            continue;
        }
        register += current_command.unwrap().get_value();
        if let Some(cmd) = cmds.next() {
            current_command = Some(cmd);
            command_cycles = current_command.unwrap().get_cycles();
        } else {
            current_command = None;
        }
        cycle_count += 1;
    }
    signal_strengths
}

// Solution for part 2
pub fn part_2(commands: &[Instruction]) -> Result<()> {
    let mut register = 1;
    let mut cycle_count = 1;
    let mut cmds = commands.iter();
    let mut current_command = cmds.next();
    let mut command_cycles = current_command.unwrap().get_cycles();
    let mut current_line: Vec<char> = vec![];
    while cmds.len() > 0 || !current_command.is_none() {
        if is_sprite_visible(register, cycle_count % 40 - 1) {
            current_line.push('#');
        } else {
            current_line.push('.');
        }
        if current_line.len() == 40 {
            println!(
                "{}",
                current_line.iter().fold(String::new(), |mut accum, pixel| {
                    accum.push(*pixel);
                    accum
                })
            );
            current_line = vec![];
        }
        if command_cycles > 0 {
            command_cycles -= 1;
            cycle_count += 1;
            continue;
        }
        register += current_command.unwrap().get_value();
        if let Some(cmd) = cmds.next() {
            current_command = Some(cmd);
            command_cycles = current_command.unwrap().get_cycles();
        } else {
            current_command = None;
        }
        cycle_count += 1;
    }
    Ok(())
}

fn is_sprite_visible(loc: i32, current_pixel: i32) -> bool {
    loc >= current_pixel - 1 && loc <= current_pixel + 1
}
