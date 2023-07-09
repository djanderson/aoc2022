/// Day 8: Treetop Tree House
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut rows: Vec<Vec<u32>> = vec![];
    let mut cols: Vec<Vec<u32>> = vec![];

    // Initialize an empty cols vector for each number in the first row.
    input
        .lines()
        .next()
        .unwrap()
        .bytes()
        .for_each(|_| cols.push(vec![]));

    // Initialize the rows and cols vectors.
    const BASE10: u32 = 10;
    for line in input.lines() {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(BASE10).unwrap()).collect();
        for (i, height) in row.iter().enumerate() {
            cols[i].push(*height);
        }
        rows.push(row);
    }

    let nvisible = visible_trees(&rows, &cols);
    println!("Part 1: {}", nvisible);

    // Part 2
    let score = best_scenic_score(&rows, &cols);
    println!("Part 2: {}", score);
}

// Part 2: Find the most scenic view.
fn best_scenic_score(rows: &Vec<Vec<u32>>, cols: &Vec<Vec<u32>>) -> usize {
    let mut scores: Vec<usize> = vec![];

    fn score_view<'a>(this: u32, others: impl Iterator<Item = &'a u32>) -> usize {
        let mut score = 0;
        for other in others {
            score += 1;
            if *other >= this {
                break;
            }
        }
        score
    }

    // Don't consider edge trees, since a score on one edge of 0 makes the
    // total scenic score 0.
    for (i, row) in rows.iter().enumerate().take(rows.len() - 1).skip(1) {
        for (j, col) in cols.iter().enumerate().take(cols.len() - 1).skip(1) {
            let this_tree = row[j];
            let score_left = score_view(this_tree, row[..j].iter().rev());
            let score_right = score_view(this_tree, row[j + 1..].iter());
            let score_up = score_view(this_tree, col[..i].iter().rev());
            let score_down = score_view(this_tree, col[i + 1..].iter());
            scores.push(score_left * score_right * score_up * score_down);
        }
    }

    scores.into_iter().max().unwrap()
}

// Part 1: Find the number of trees that are visible from outside.
fn visible_trees(rows: &Vec<Vec<u32>>, cols: &Vec<Vec<u32>>) -> usize {
    // All of the outside perimeter are visible.
    let mut nvisible = rows.len() * 2 + (cols.len() - 2) * 2;

    for (i, row) in rows.iter().enumerate().take(rows.len() - 1).skip(1) {
        for (j, col) in cols.iter().enumerate().take(cols.len() - 1).skip(1) {
            let height = row[j];
            // Check left.
            let visible_left = row[..j].iter().all(|&h| h < height);
            if visible_left {
                nvisible += 1;
                continue;
            }
            // Check right.
            let visible_right = row[j + 1..].iter().all(|&h| h < height);
            if visible_right {
                nvisible += 1;
                continue;
            }
            // Check up.
            let visible_up = col[..i].iter().all(|&h| h < height);
            if visible_up {
                nvisible += 1;
                continue;
            }
            // Check down.
            let visible_down = col[i + 1..].iter().all(|&h| h < height);
            if visible_down {
                nvisible += 1;
                continue;
            }
        }
    }

    nvisible
}
