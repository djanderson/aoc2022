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
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Int(x), Int(y)) => x.cmp(y),
            (List(u), List(v)) => u.cmp(v),
            (Int(x), List(v)) => vec![Int(*x)].cmp(v),
            (List(u), Int(y)) => u.cmp(&vec![Int(*y)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Parse an integer
fn parse_integer(input: &str) -> IResult<&str, Packet> {
    map_res(digit1, |s: &str| s.parse::<i32>().map(Packet::Int))(input)
}

// Parse a list of Packets
fn parse_list(input: &str) -> IResult<&str, Packet> {
    let parser = separated_list0(char(','), parse_packet);

    delimited(char('['), parser, char(']'))(input)
        .map(|(remaining, pkts)| (remaining, Packet::List(pkts)))
}

// Parse a Packet (either an int or another list)
fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((parse_integer, parse_list))(input)
}

// Consume an entire line and convert into Packet
fn parse_line(input: &str) -> IResult<&str, Packet> {
    all_consuming(parse_list)(input)
}

pub fn main() {
    use Packet::*;

    let input = fs::read_to_string("input.txt").unwrap();

    let packets = input
        .split("\n\n")
        .map(|pkt| pkt.lines())
        .map(|mut lines| (lines.next().unwrap(), lines.next().unwrap()));

    let mut sum = 0;

    for (i, (left, right)) in packets.enumerate() {
        let index = i + 1;
        let (_, left) = parse_line(left).unwrap();
        let (_, right) = parse_line(right).unwrap();
        if left < right {
            sum += index;
        }
    }

    println!("Part 1: {}", sum);

    // Divider packet [[2]]
    let divider1 = List(vec![List(vec![Int(2)])]);
    // Divider packet [[6]]
    let divider2 = List(vec![List(vec![Int(6)])]);

    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|pkt| parse_line(pkt).unwrap().1)
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
