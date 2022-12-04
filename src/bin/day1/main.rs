/// Day 1: Calorie Counting
use std::fs;
use std::iter::Iterator;

use itertools::Itertools;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let max_calories: i32 = calories_per_elf(&input).max().unwrap();

    println!("Part 1: {}", max_calories);

    let max_3_calories: i32 = calories_per_elf(&input).sorted().rev().take(3).sum();

    println!("Part 2: {}", max_3_calories);
}

fn calories_per_elf(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.trim().split("\n\n").map(|calories| {
        calories
            .lines()
            .map(|s| {
                s.parse::<i32>()
                    .expect("line should contain a single integer")
            })
            .sum()
    })
}
