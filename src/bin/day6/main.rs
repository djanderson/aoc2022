/// Day 6: Tuning Trouble
use std::collections::HashSet;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let marker_loc = find_marker(&input, 4).expect("input should contain start-of-packet marker");

    println!("Part 1: {}", marker_loc);

    let marker_loc = find_marker(&input, 14).expect("input should contain start-of-packet marker");

    println!("Part 2: {}", marker_loc);
}

fn find_marker(input: &str, window_size: usize) -> Option<usize> {
    let mut set = HashSet::with_capacity(window_size);

    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find_map(|(i, window)| match is_unique(window, &mut set) {
            true => Some(i + window_size),
            false => None,
        })
}

fn is_unique(arr: &[u8], set: &mut HashSet<u8>) -> bool {
    set.clear();
    let mut unique = true;
    for b in arr {
        if !set.insert(*b) {
            unique = false;
            break;
        }
    }
    unique
}
