/// Day 6: Tuning Trouble
use std::collections::HashSet;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let marker_idx = find_marker(&input, 4);
    let marker_loc = 1 + marker_idx.expect("input should contain start-of-packet marker");

    println!("Part 1: {}", marker_loc);

    let marker_idx = find_marker(&input, 14);
    let marker_loc = 1 + marker_idx.expect("input should contain start-of-packet marker");

    println!("Part 2: {}", marker_loc);
}

fn find_marker(input: &str, window_size: usize) -> Option<usize> {
    let mut window: Vec<u8> = vec![0; window_size];

    let mut marker_idx: Option<usize> = None;
    for (i, b) in input.bytes().enumerate() {
        window[i % window_size] = b;
        if i < 3 {
            continue;
        }
        if is_unique(&window) {
            marker_idx = Some(i);
            break;
        }
    }

    marker_idx
}

fn is_unique(arr: &[u8]) -> bool {
    let mut set = HashSet::with_capacity(arr.len());
    let mut unique = true;
    for b in arr {
        if !set.insert(*b) {
            unique = false;
            break;
        }
    }
    unique
}
