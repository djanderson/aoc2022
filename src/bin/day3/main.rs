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
            calculate_priority(duplicate)
        })
        .sum();

    println!("Part 1: {}", priority_sum);

    let mut lines = input.lines();
    let mut priority_sum: i32 = 0;
    // NOTE: Iterator.array_chunks could make this more functional but it's currently in nightly
    while let Some(line1) = lines.next() {
        let line2 = lines.next().expect("Number of lines not a multiple of 3");
        let line3 = lines.next().expect("Number of lines not a multiple of 3");
        let elf1: HashSet<u8> = HashSet::from_iter(line1.bytes());
        let elf2: HashSet<u8> = HashSet::from_iter(line2.bytes());
        let elf3: HashSet<u8> = HashSet::from_iter(line3.bytes());
        let elf1_and_elf2: HashSet<u8> = elf1.intersection(&elf2).cloned().collect();
        let badge = elf1_and_elf2.intersection(&elf3).next().expect("Invalid input");
        priority_sum += calculate_priority(badge);
    }

    println!("Part 2: {}", priority_sum);
}

#[inline]
fn calculate_priority(c: &u8) -> i32 {
    match c {
        b'a'..=b'z' => (c - b'a' + 1).into(),
        b'A'..=b'Z' => (c - b'A' + 27).into(),
        _ => panic!("Invalid character {}", c),
    }
}
