/// Day 2: Rock Paper Scissors
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let score: i32 = input
        .lines()
        .map(|line| match line {
            "A X" => 4, // 1 + 3
            "A Y" => 8, // 2 + 6
            "A Z" => 3, // 3 + 0
            "B X" => 1, // 1 + 0
            "B Y" => 5, // 2 + 3
            "B Z" => 9, // 3 + 6
            "C X" => 7, // 1 + 6
            "C Y" => 2, // 2 + 0
            "C Z" => 6, // 3 + 3
            &_ => panic!("Invalid input"),
        })
        .sum();

    println!("Part 1: {}", score);

    let score: i32 = input
        .lines()
        .map(|line| match line {
            "A X" => 3, // 3 + 0
            "A Y" => 4, // 1 + 3
            "A Z" => 8, // 2 + 6
            "B X" => 1, // 1 + 0
            "B Y" => 5, // 2 + 3
            "B Z" => 9, // 3 + 6
            "C X" => 2, // 2 + 0
            "C Y" => 6, // 3 + 3
            "C Z" => 7, // 1 + 6
            &_ => panic!("Invalid input"),
        })
        .sum();

    println!("Part 2: {}", score);
}
