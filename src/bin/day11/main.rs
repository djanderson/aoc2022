/// Day 11: Monkey in the Middle
use std::cell::RefCell;
use std::cmp::Reverse;
use std::fs;
use std::str::FromStr;

// A "round" is:
// 1. Monkey inspects each item: worry = operation(worry).
// 2. I am relieved that the item is not damaged: worry = int(worry / 3).
// 3. The monkeys test my worry level on each item they hold in order and throw it accordingly.
// 4. An item thrown to a monkey is appended to the _end_ of its list.
pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| Monkey::from_str(s).unwrap())
        .collect();

    let mut n_inspections = vec![0usize; monkeys.len()];
    let lcm = monkeys.iter().map(|m| m.divisor).reduce(lcm).unwrap();

    for _round in 0..10000 {
        for (from_monkey_idx, monkey) in monkeys.iter().enumerate() {
            n_inspections[from_monkey_idx] += monkey.items.borrow().len();

            for worry in monkey.items.borrow_mut().iter_mut() {
                *worry = (monkey.test)(*worry) % lcm;
            }

            while let Some(worry) = monkey.items.borrow_mut().pop() {
                let to_partner_idx = (worry % monkey.divisor == 0) as usize;
                let to_monkey_idx = monkey.partners[to_partner_idx] as usize;
                monkeys[to_monkey_idx].items.borrow_mut().push(worry);
            }
        }
    }

    n_inspections.sort_unstable_by_key(|n| Reverse(*n));
    println!("Part 1: {}", n_inspections[0] * n_inspections[1]);
}

struct Monkey {
    divisor: u64,
    test: Box<dyn Fn(u64) -> u64>,
    items: RefCell<Vec<u64>>,
    partners: [u64; 2], // index 0 -> false, index 1 -> true
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMonkeyError;

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);
        let mut line = lines.next().unwrap();
        let items: RefCell<Vec<_>> =
            RefCell::new(line[18..].split(", ").map(|s| s.parse().unwrap()).collect());
        line = lines.next().unwrap();
        let test: Box<dyn Fn(u64) -> u64> = {
            let op = match line.chars().nth(23).unwrap() {
                '+' => u64::checked_add,
                '*' => u64::checked_mul,
                unknown => panic!("Unknown operation: '{}'", unknown),
            };
            match &line[25..] {
                "old" => Box::new(move |x: u64| op(x, x).unwrap()),
                rhs => {
                    let n = rhs.parse().unwrap();
                    Box::new(move |x: u64| op(x, n).unwrap())
                }
            }
        };
        line = lines.next().unwrap();
        let divisor: u64 = line[21..].parse().unwrap();
        line = lines.next().unwrap();
        let mut partners = [0u64, 2];
        partners[1] = line[29..].parse().unwrap();
        line = lines.next().unwrap();
        partners[0] = line[30..].parse().unwrap();
        Ok(Monkey {
            divisor,
            test,
            items,
            partners,
        })
    }
}

#[inline]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline]
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
