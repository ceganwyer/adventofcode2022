#![allow(dead_code, unused)]
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use eyre::{eyre, Result};

#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<Item>,
    pub op: Operation,
    pub test: Test,
}

impl Monkey {
    fn new(id: usize, items: Vec<Item>, op: Operation, test: Test) -> Self {
        Monkey {
            id,
            items,
            op,
            test,
        }
    }

    fn inspect_items(&mut self, op: &Operation) {
        for mut item in self.items.iter_mut() {
            Monkey::inspect_item(&mut item, &op)
        }
    }

    fn inspect_item(mut item: &mut Item, op: &Operation) {
        match op {
            Operation::Add(operator) => item.add(operator),
            Operation::Multiply(operator) => item.mult(operator),
        }
    }

    fn throw_items(&mut self, monkeys: &mut Monkeys) {
        while let Some(item) = self.items.pop() {
            if item.worry_level % self.test.divisible_by == 0 {
                if let Some(monkey) = monkeys.get_mut(self.test.true_monkey) {
                    monkey.items.push(item);
                }
            } else {
                if let Some(monkey) = monkeys.get_mut(self.test.false_monkey) {
                    monkey.items.push(item);
                }
            }
        }
    }

    fn get_items(&self) -> &Vec<Item> {
        self.items.as_ref()
    }

    fn get_items_mut(&mut self) -> &mut Vec<Item> {
        self.items.as_mut()
    }
}

impl AsRef<Monkey> for Monkey {
    fn as_ref(&self) -> &Monkey {
        self
    }
}

impl AsMut<Monkey> for Monkey {
    fn as_mut(&mut self) -> &mut Monkey {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Item {
    worry_level: i32,
}

impl Item {
    fn add(&mut self, operator: &String) {
        if operator.contains("old") {
            self.worry_level = self.worry_level * 2;
            self.worry_level = self.worry_level / 3;
        } else {
            let operator = operator.parse::<i32>().expect("Expected a number");
            self.worry_level += operator;
            self.worry_level = self.worry_level / 3;
        }
    }

    fn mult(&mut self, operator: &String) {
        if operator.contains("old") {
            self.worry_level = self.worry_level.pow(2);
            self.worry_level = self.worry_level / 3;
        } else {
            let operator = operator.parse::<i32>().expect("Expected a number");
            self.worry_level *= operator;
            self.worry_level = self.worry_level / 3;
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Test {
    divisible_by: i32,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add(String),
    Multiply(String),
}

#[derive(Debug)]
struct Monkeys {
    monkeys: HashMap<usize, Monkey>,
}

impl Monkeys {
    fn new() -> Self {
        Monkeys {
            monkeys: HashMap::new(),
        }
    }

    fn iter(&self) -> Vec<&Monkey> {
        self.monkeys.values().collect()
    }

    fn add_monkey(&mut self, monkey: Monkey) -> usize {
        let index = self.monkeys.len();
        self.monkeys.insert(monkey.id, monkey);
        index
    }

    fn get(&self, index: usize) -> Option<&Monkey> {
        return if let Some(monkey) = self.monkeys.get(&index) {
            Some(monkey.as_ref())
        } else {
            None
        };
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Monkey> {
        return if let Some(monkey) = self.monkeys.get_mut(&index) {
            Some(monkey.as_mut())
        } else {
            None
        };
    }
}

pub fn parse_input(input: String) -> Result<Monkeys> {
    let mut monkeys = Monkeys::new();
    let mut id = 0;
    let mut items: Vec<Item> = vec![];
    let mut op = Operation::Add(String::default());
    let mut test = Test::default();
    for line in input.lines() {
        if line.is_empty() {
            // Push monkey with obtained values then reset values
            monkeys.add_monkey(Monkey::new(id, items, op, test));
            id += 1;
            items = vec![];
            op = Operation::Add(String::default());
            test = Test::default();
            continue;
        }
        match line {
            line if line.contains("items") => {
                items = parse_items(line);
            }
            line if line.contains("Operation") => {
                op = parse_operation(line);
            }
            line if line.contains("Test") => {
                test.divisible_by = parse_single_number::<i32>(line);
            }
            line if line.contains("true") => {
                test.true_monkey = parse_single_number::<usize>(line);
            }
            line if line.contains("false") => {
                test.false_monkey = parse_single_number::<usize>(line);
            }
            _ => {}
        }
    }
    Ok(monkeys)
}

pub fn take_turns(monkeys: &mut Monkeys) {
    let mut hold_counts: HashMap<usize, i32> = HashMap::new();
    let iter = monkeys.iter();
    for _ in 0..20 {
        for monke in iter {
            let mut monkey = monkeys.get_mut(monke.id).unwrap();
            if monkey.items.is_empty() {
                continue;
            }
            let hold_entry = hold_counts.entry(monkey.id).or_insert(0);
            *hold_entry += monkey.items.len() as i32;
            monkey.inspect_items(&monkey.op);
            monkey.throw_items(monkeys);
        }
    }
}

fn parse_items(items_line: &str) -> Vec<Item> {
    items_line
        .replace(",", "")
        .split_whitespace()
        .map(|a| {
            if a.chars().next().unwrap().is_digit(10) {
                let val = a.parse::<i32>().unwrap();
                Some(Item { worry_level: val })
            } else {
                None
            }
        })
        .flat_map(|a| a)
        .collect()
}

fn parse_operation(op_line: &str) -> Operation {
    let mut parts = op_line.split_whitespace().rev();
    let num = parts.next().unwrap();
    let op = parts.next().unwrap();
    match op {
        "*" => Operation::Multiply(num.to_string()),
        "+" => Operation::Add(num.to_string()),
        _ => panic!("Unexpected input!"),
    }
}

fn parse_single_number<T: FromStr>(line: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    let num = line
        .chars()
        .skip_while(|e| !e.is_digit(10))
        .take_while(|e| e.is_digit(10))
        .fold(String::new(), |acc, ch| acc + ch.to_string().as_str());
    num.parse::<T>().expect("To parse number")
}
