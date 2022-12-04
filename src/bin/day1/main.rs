/// Day 1: Calorie Counting
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let max_calories: i32 = input
        .trim()
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|s| s.parse::<i32>().expect("Invalid integer"))
                .sum()
        })
        .max()
        .unwrap();

    println!("Part 1: {}", max_calories);

    let mut calories: Vec<i32> = input
        .trim()
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|s| s.parse::<i32>().expect("Invalid integer"))
                .sum()
        })
        .collect();

    calories.sort();

    let top_3: i32 = calories.iter().rev().take(3).sum();

    println!("Part 2: {}", top_3);
}
