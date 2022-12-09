use core::{f32, fmt};
use std::collections::VecDeque;

use eyre::{eyre, Result};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Coord {
    x: i16,
    y: i16,
    last: Option<(i16, i16)>,
}

impl Coord {
    fn new(x: i16, y: i16) -> Coord {
        Coord { x, y, last: None }
    }

    fn calc_distance(a: &Coord, b: &Coord) -> f32 {
        let x_diff = a.x - b.x;
        let y_diff = a.y - b.y;
        let sum = x_diff.pow(2) + y_diff.pow(2);
        f32::from(sum).sqrt()
    }

    fn diff(a: &Coord, b: &Coord) -> (i16, i16) {
        (a.x - b.x, a.y - b.y)
    }

    fn is_cardinal(&self, other: &Coord) -> bool {
        Coord::same_column(self, other) || Coord::same_row(self, other)
    }

    fn same_row(a: &Coord, b: &Coord) -> bool {
        a.y == b.y
    }

    fn same_column(a: &Coord, b: &Coord) -> bool {
        a.x == b.x
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Move {
    dir: Direction,
    delta: i16,
}

impl Move {
    fn new(dir: Direction, delta: i16) -> Move {
        Move { dir, delta }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_input(input: String) -> Result<Vec<Move>> {
    let mut moves = Vec::new();
    for line in input.lines() {
        let (dir, delta) = line
            .split_once(" ")
            .ok_or_else(|| eyre!("Unable to split line."))?;
        let delta = delta.parse::<i16>()?;
        let mv = match dir {
            "U" => Move::new(Direction::Up, delta),
            "D" => Move::new(Direction::Down, delta),
            "L" => Move::new(Direction::Left, delta),
            "R" => Move::new(Direction::Right, delta),
            _ => panic!("Unrecognized input!"),
        };
        moves.push(mv);
    }
    Ok(moves)
}

// Solution for part 1
pub fn part_1(moves: &Vec<Move>) -> Vec<Coord> {
    let (mut head, mut tail) = (Coord::new(0, 0), Coord::new(0, 0));
    let mut tail_locations = Vec::new();
    for mv in moves {
        for _ in 0..mv.delta {
            move_head(&mut head, &mv.dir);
            move_tail(&mut tail, &head);
            tail_locations.push(Coord::new(tail.x, tail.y));
        }
    }
    println!("Final Locations:\nHead:\t{}\nTail:\t{}", head, tail);
    tail_locations.sort();
    tail_locations.dedup();
    tail_locations
}

// Solution for part 2
pub fn part_2(moves: &Vec<Move>, rope_len: usize) -> Result<Vec<Coord>> {
    let mut rope = generate_rope(rope_len);
    let mut tail_locations = Vec::new();
    for mv in moves {
        for _ in 0..mv.delta {
            {
                let mut head = rope.front_mut().ok_or_else(|| eyre!("Rope is empty!"))?;
                move_head(&mut head, &mv.dir);
            }
            for i in 1..rope_len {
                let prev = rope
                    .get(i - 1)
                    .ok_or_else(|| eyre!("Rope segments detached!"))?
                    .clone();
                let knot = rope
                    .get_mut(i)
                    .ok_or_else(|| eyre!("Unable to find segment {i}"))?;
                move_knot(knot, &prev);
            }
            let tail = rope.back().ok_or_else(|| eyre!("Rope is empty!"))?;
            tail_locations.push(Coord::new(tail.x, tail.y))
        }
    }
    println!(
        "Final locations:\n{}",
        rope.iter()
            .fold(String::new(), |acc, coord| acc + &coord.to_string() + "\n")
    );
    tail_locations.sort();
    tail_locations.dedup();
    Ok(tail_locations)
}

fn move_head(head: &mut Coord, dir: &Direction) {
    head.last = Some((head.x, head.y));
    match dir {
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
        Direction::Right => head.x += 1,
        Direction::Left => head.x -= 1,
    }
}

fn move_tail(tail: &mut Coord, head: &Coord) {
    let dist = Coord::calc_distance(head, tail);
    if dist < 2.0 {
        return;
    }
    if let Some((x, y)) = head.last {
        tail.last = Some((tail.x, tail.y));
        tail.x = x;
        tail.y = y;
    }
}

fn generate_rope(size: usize) -> VecDeque<Coord> {
    let mut rope = VecDeque::new();
    for _ in 0..size {
        rope.push_back(Coord::new(0, 0));
    }
    rope
}

fn move_knot(knot: &mut Coord, prev: &Coord) {
    let dist = Coord::calc_distance(prev, knot);
    let (diff_x, diff_y) = Coord::diff(prev, knot);
    if dist < 2.0 {
        return;
    }
    if Coord::is_cardinal(knot, prev) {
        // Cardinal movements
        if Coord::same_row(knot, prev) {
            match diff_x.is_positive() {
                true => knot.x += 1,
                false => knot.x -= 1,
            }
        }
        if Coord::same_column(knot, prev) {
            match diff_y.is_positive() {
                true => knot.y += 1,
                false => knot.y -= 1,
            }
        }
    } else {
        // Diagonal
        match (diff_x.is_positive(), diff_y.is_positive()) {
            (true, true) => {
                knot.x += 1;
                knot.y += 1;
            }
            (true, false) => {
                knot.x += 1;
                knot.y -= 1;
            }
            (false, true) => {
                knot.x -= 1;
                knot.y += 1;
            }
            (false, false) => {
                knot.x -= 1;
                knot.y -= 1;
            }
        }
    }
}
