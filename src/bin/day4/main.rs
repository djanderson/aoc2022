/// Day 4: Camp Cleanup
use std::fs;
use std::ops::RangeInclusive;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let n_contained: i32 = input
        .lines()
        .map(|line| {
            let (r1, r2) = line_to_ranges(line);
            let r1_contains_r2 = r1.contains(r2.start()) && r1.contains(r2.end());
            let r2_contains_r1 = r2.contains(r1.start()) && r2.contains(r1.end());
            let is_contained = r1_contains_r2 || r2_contains_r1;
            is_contained as i32
        })
        .sum();

    println!("Part 1: {}", n_contained);

    let n_overlapped: i32 = input
        .lines()
        .map(|line| {
            let (r1, r2) = line_to_ranges(line);
            let r1_is_overlapped = r1.contains(r2.start()) || r1.contains(r2.end());
            // Need to check this as well to catch case where r1 is fully contained in r2
            let r2_is_overlapped = r2.contains(r1.start()) || r2.contains(r1.end());
            let is_overlapped = r1_is_overlapped || r2_is_overlapped;
            is_overlapped as i32
        })
        .sum();

    println!("Part 2: {}", n_overlapped);
}

// Map each line to a pair of Ranges
fn line_to_ranges(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let bounds: Vec<i32> = line.split([',', '-']).map(|d| d.parse().unwrap()).collect();
    match bounds[..] {
        [r1_start, r1_end, r2_start, r2_end] => {
            let r1 = RangeInclusive::new(r1_start, r1_end);
            let r2 = RangeInclusive::new(r2_start, r2_end);
            (r1, r2)
        }
        _ => panic!("Invalid input"),
    }
}
