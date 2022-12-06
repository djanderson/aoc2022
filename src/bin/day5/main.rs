/// Day 5: Supply Stacks
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Read state of current stacks
    let n_stacks = input
        .lines()
        .find(|line| line.starts_with(" 1"))
        .expect("input should contain a line of stack numbers")
        .split_ascii_whitespace()
        .count();

    let mut stacks: Vec<Vec<Crate>> = vec![];
    for _ in 0..n_stacks {
        stacks.push(vec![]);
    }

    let mut lines = input.lines();

    for line in &mut lines {
        if line.trim().is_empty() {
            break; // end of section
        }
        for (i, stack) in stacks.iter_mut().enumerate() {
            if let Some(c) = line.chars().nth(1 + i * 4) {
                if c.is_ascii_uppercase() {
                    stack.push(Crate::new(c));
                }
            }
        }
    }
    // Reverse the stack since we built it upside-down
    for stack in &mut stacks {
        stack.reverse();
    }

    let mut moves: Vec<Move> = vec![];

    // Read list of moves
    for line in &mut lines {
        let numbers: Vec<usize> = line
            .split_ascii_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        match numbers[..] {
            [n_times, from, to] => moves.push(Move { from, to, n_times }),
            _ => panic!("Invalid number of numbers in move line"),
        }
    }

    // Do the moves
    for m in moves {
        for _ in 0..m.n_times {
            let c = stacks[m.from - 1]
                .pop()
                .expect("from stack should contain crates");
            stacks[m.to - 1].push(c);
        }
    }

    // Pop end of stacks and display answer
    let mut top_crates = String::new();
    for stack in stacks {
        if let Some(c) = stack.last() {
            top_crates.push(c.id);
        }
    }

    println!("Part 1: {}", top_crates);
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    n_times: usize,
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
