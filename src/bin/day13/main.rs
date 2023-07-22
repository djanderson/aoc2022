/// Day 13: Distress Signal
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{all_consuming, map_res},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Int(i32),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        use Item::*;
        match (self, other) {
            (Int(x), Int(y)) => x.cmp(y),
            (List(u), List(v)) => u.cmp(v),
            (Int(x), List(v)) => vec![Int(*x)].cmp(v),
            (List(u), Int(y)) => u.cmp(&vec![Int(*y)]),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Parse an integer
fn parse_integer(input: &str) -> IResult<&str, Item> {
    map_res(digit1, |digit_str: &str| {
        digit_str.parse::<i32>().map(Item::Int)
    })(input)
}

// Parse a list of Items
fn parse_list(input: &str) -> IResult<&str, Item> {
    let parser = separated_list0(char(','), parse_item);

    delimited(char('['), parser, char(']'))(input)
        .map(|(remaining, items)| (remaining, Item::List(items)))
}

// This will parse an Item
fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((parse_integer, parse_list))(input)
}

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let packets = input
        .split("\n\n")
        .map(|pkt| pkt.lines())
        .map(|mut lines| (lines.next().unwrap(), lines.next().unwrap()));

    let mut sum = 0;

    for (i, (left, right)) in packets.enumerate() {
        let index = i + 1;
        let (_, left) = all_consuming(parse_list)(left).unwrap();
        let (_, right) = all_consuming(parse_list)(right).unwrap();
        if left < right {
            sum += index;
        }
    }

    println!("Part 1: {}", sum);
}