use std::collections::HashSet;
/// Day 9: Rope Bridge
use std::fs;
use std::ops::{Add, Sub};
use std::str::FromStr;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut head = Point::default();
    let mut tail = Point::default();

    let mut tracker: HashSet<Point> = HashSet::new();
    tracker.insert(tail.clone());

    for line in input.lines() {
        let Some((d, c)) = line.split_once(" ") else {
            panic!("Failed to parse input line");
        };
        let head_offset = Point::from_str(d).expect("Direction should be in R, L, U, D");
        let count = i32::from_str(c).expect("Count should be an integer");
        for _ in 0..count {
            head = &head + &head_offset;
            tail = &tail + &tail_offset(&head, &tail);
            tracker.insert(tail.clone());
        }
    }

    println!("Part 1: {}", tracker.len());
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Point { x: 1, y: 0 }),
            "L" => Ok(Point { x: -1, y: 0 }),
            "U" => Ok(Point { x: 0, y: 1 }),
            "D" => Ok(Point { x: 0, y: -1 }),
            _ => Err(ParsePointError),
        }
    }
}

/// Return an offset that tail can apply to stay less than one space away
/// from head
fn tail_offset(head: &Point, tail: &Point) -> Point {
    match head - tail {
        Point { x, y } if x.abs() <= 1 && y.abs() <= 1 => Point::default(),
        Point { x, y } => Point {
            x: x.max(-1).min(1),
            y: y.max(-1).min(1),
        },
    }
}
