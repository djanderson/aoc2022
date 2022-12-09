/// Day 6: Tuning Trouble
use std::fs;
use std::collections::HashSet;

const WINDOW_SIZE: usize = 14;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut window = [0u8; WINDOW_SIZE];

    let mut marker_idx: Option<usize> = None;
    for (i, b) in input.bytes().enumerate() {
        window[i % WINDOW_SIZE] = b;
        if i < 3 {
            continue;
        }
        if is_unique(&window) {
            marker_idx = Some(i);
            break;
        }
    }
    let marker_loc = 1 + marker_idx.expect("input should contain start-of-packet marker");

    println!("Part 1: {}", marker_loc);
}

fn is_unique(arr: &[u8; WINDOW_SIZE]) -> bool {
    let mut set = HashSet::with_capacity(WINDOW_SIZE);
    let mut unique = true;
    for b in arr {
        if !set.insert(*b) {
            unique = false;
            break;
        }
    }
    unique
}
