/// Day 3: Rucksack Reorganization
use std::collections::HashSet;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let priority_sum: i32 = input
        .lines()
        .map(|line| {
            let boundary = line.len() / 2;
            let compartment_1: HashSet<u8> = HashSet::from_iter(line[..boundary].bytes());
            let compartment_2: HashSet<u8> = HashSet::from_iter(line[boundary..].bytes());
            let duplicate: &u8 = compartment_1
                .intersection(&compartment_2)
                .next()
                .expect("Invalid line contains no duplicates");
            let priority = match duplicate {
                b'a'..=b'z' => duplicate - b'a' + 1,
                b'A'..=b'Z' => duplicate - b'A' + 27,
                _ => panic!("Invalid character {}", duplicate),
            };
            priority as i32
        })
        .sum();

    println!("Part 1: {}", priority_sum);
}
