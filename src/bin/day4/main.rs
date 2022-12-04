/// Day 4: Camp Cleanup
use std::cmp::{max, min};
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let n_contained = input
        .lines()
        .map(|line| {
            let (r1, r2) = line
                .split_once(',')
                .expect("ranges should separated by ','");
            let s1: SectionRange = r1.parse().unwrap();
            let s2: SectionRange = r2.parse().unwrap();
            (s1, s2)
        })
        .filter(|(s1, s2)| s1.contains(&s2) || s2.contains(&s1))
        .count();

    println!("Part 1: {}", n_contained);

    let n_overlapped = input
        .lines()
        .map(|line| {
            let (r1, r2) = line
                .split_once(',')
                .expect("ranges should separated by ','");
            let s1: SectionRange = r1.parse().unwrap();
            let s2: SectionRange = r2.parse().unwrap();
            (s1, s2)
        })
        .filter(|(s1, s2)| s1.overlaps(&s2))
        .count();

    println!("Part 2: {}", n_overlapped);
}

struct SectionRange {
    start: i32,
    end: i32,
}

impl SectionRange {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        max(self.start, other.start) <= min(self.end, other.end)
    }
}

impl FromStr for SectionRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s
            .split_once('-')
            .expect("section range should contain dash");
        let start = start_str.parse().expect("value should parse as an int");
        let end = end_str.parse().expect("value should parse as an int");
        Ok(Self { start, end })
    }
}
