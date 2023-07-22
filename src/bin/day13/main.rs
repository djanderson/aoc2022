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

#[derive(Clone, Debug, PartialEq, Eq)]
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
    map_res(digit1, |s: &str| s.parse::<i32>().map(Item::Int))(input)
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
    use Item::*;

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

    // Divider packet [[2]]
    let divider1 = List(vec![List(vec![Int(2)])]);
    // Divider packet [[6]]
    let divider2 = List(vec![List(vec![Int(6)])]);

    let mut packets: Vec<Item> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|pkt| all_consuming(parse_list)(pkt).unwrap().1)
        .collect();

    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort();

    let part2: usize = packets
        .into_iter()
        .enumerate()
        .filter_map(|(i, pkt)| {
            if pkt == divider1 || pkt == divider2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();

    println!("Part 2: {}", part2);
}
