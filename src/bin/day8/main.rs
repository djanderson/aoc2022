/// Day 8: Treetop Tree House
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut rows: Vec<Vec<u8>> = vec![];
    let mut cols: Vec<Vec<u8>> = vec![];

    // Initialize an empty cols vector for each number in the first row.
    input
        .lines()
        .next()
        .unwrap()
        .bytes()
        .for_each(|_| cols.push(vec![]));

    // Initialize the rows and cols vectors.
    for line in input.lines() {
        let row: Vec<u8> = line.bytes().map(|b| u8::from(b)).collect();
        for (i, height) in row.iter().enumerate() {
            cols[i].push(*height);
        }
        rows.push(row);
    }

    // All of the outside perimeter are visible.
    let mut nvisible = rows.len() * 2 + (cols.len() - 2) * 2;

    for i in 1..rows.len() - 1 {
        for j in 1..cols.len() - 1 {
            let height = rows[i][j];
            // Check left.
            let visible_left = rows[i][..j].iter().all(|&h| h < height);
            if visible_left {
                nvisible += 1;
                continue;
            }
            // Check right.
            let visible_right = rows[i][j + 1..].iter().all(|&h| h < height);
            if visible_right {
                nvisible += 1;
                continue;
            }
            // Check up.
            let visible_up = cols[j][..i].iter().all(|&h| h < height);
            if visible_up {
                nvisible += 1;
                continue;
            }
            // Check down.
            let visible_down = cols[j][i + 1..].iter().all(|&h| h < height);
            if visible_down {
                nvisible += 1;
                continue;
            }
        }
    }

    println!("{}", nvisible);
}
