/// Day 5: Supply Stacks
use std::fs;
use std::io::{Error, ErrorKind};
use std::str::{FromStr, Lines};

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Count labels to determine number of stacks
    let n_stacks = input
        .lines()
        .find(|line| line.starts_with(" 1"))
        .expect("input should contain a line of stack numbers")
        .split_ascii_whitespace()
        .count();

    // Part 1: A move operation moves 1 crate at a time

    let mut lines = input.lines();
    let mut stacks = read_stacks(&mut lines, n_stacks);
    let moves = read_moves(lines);

    // Do the moves
    for mv in moves {
        for _ in 0..mv.n_items {
            let from = &mut stacks[mv.from - 1];
            let top_crate = from.pop().expect("'from' stack should not be empty");
            let to = &mut stacks[mv.to - 1];
            to.push(top_crate);
        }
    }

    let top_crates = format_answer(stacks);
    println!("Part 1: {}", top_crates);

    // Part 2: A move operation moves all crates in the move together

    let mut lines = input.lines();
    let mut stacks = read_stacks(&mut lines, n_stacks);
    let moves = read_moves(lines);

    // Do the moves
    for mv in moves {
        let from = &mut stacks[mv.from - 1];
        let crates: Vec<Crate> = from.drain(from.len() - mv.n_items..).collect();
        let to = &mut stacks[mv.to - 1];
        to.extend(crates);
    }

    let top_crates = format_answer(stacks);
    println!("Part 2: {}", top_crates);
}

/// Read crates from the provided input lines onto the stacks
fn read_stacks(lines: &mut Lines, n_stacks: usize) -> Vec<Vec<Crate>> {
    let mut stacks: Vec<Vec<Crate>> = vec![];
    for _ in 0..n_stacks {
        stacks.push(vec![]);
    }
    for line in lines {
        if line.trim().is_empty() {
            break; // end of stack section
        }
        for (i, stack) in stacks.iter_mut().enumerate() {
            if let Some(c) = line.chars().nth(1 + i * 4) {
                if c.is_ascii_uppercase() {
                    stack.push(Crate::new(c));
                }
            }
        }
    }
    // Reverse the stacks since we built them upside-down
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    stacks
}

/// Read moves from the provided input lines into a vector
fn read_moves(lines: Lines) -> Vec<Move> {
    lines.filter_map(|line| line.parse().ok()).collect()
}

/// Format the answer string from the stack of crates
fn format_answer(stacks: Vec<Vec<Crate>>) -> String {
    stacks
        .into_iter()
        .filter_map(|stack| stack.last().map(|top_crate| top_crate.id))
        .collect()
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    n_items: usize,
}

impl FromStr for Move {
    type Err = Error;

    /// Parse a line of the form
    ///     "move 7 from 3 to 9"
    /// to
    ///     Move { n_items: 7, from: 3, to: 9 }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<usize> = s
            .split_ascii_whitespace()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .filter_map(|(_, n)| n.parse().ok())
            .collect();

        match numbers[..] {
            [n_items, from, to] => Ok(Move { from, to, n_items }),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid move line")),
        }
    }
}

#[derive(Debug)]
struct Crate {
    id: char,
}

impl Crate {
    fn new(id: char) -> Self {
        match id {
            'A'..='Z' => Self { id },
            _ => panic!("Invalid crate id, must be in 'A'-'Z'"),
        }
    }
}
