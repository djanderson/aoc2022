/// Day 10: Cathode-Ray Tube
use std::fs;
use std::str::FromStr;

pub fn main() -> Result<(), ParseInstructionError> {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cpu = Cpu::new();
    let mut clock: u64 = 1;
    let mut insn_mem = input.lines().map(|s| Instruction::from_str(s).unwrap());
    let mut total_signal_strength = 0;

    cpu.pc = insn_mem.next();

    loop {
        // Tick.
        clock += 1;

        match &mut cpu {
            Cpu {
                pc: Some(Instruction { op, cycle: 1 }),
                ..
            } => {
                match op {
                    OpCode::Addx(addend) => cpu.x += *addend,
                    OpCode::Noop => {}
                }
                cpu.pc = insn_mem.next();
            }
            Cpu {
                pc: Some(Instruction { cycle, .. }),
                ..
            } => *cycle -= 1,
            Cpu { pc: None, .. } => break,
        }

        match clock {
            20 | 60 | 100 | 140 | 180 | 220 => total_signal_strength += clock as i32 * cpu.x,
            _ => {}
        }
    }

    println!("Part 1: {}", total_signal_strength);

    Ok(())
}

#[derive(Debug)]
struct Cpu {
    x: i32, // Register
    pc: Option<Instruction>,
}

impl Cpu {
    fn new() -> Self {
        Cpu { x: 1, pc: None }
    }
}

#[derive(Clone, Copy, Debug)]
enum OpCode {
    Noop,
    Addx(i32),
}

// Instructions with cycle count
#[derive(Clone, Debug)]
struct Instruction {
    op: OpCode,
    cycle: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Instruction {
                op: OpCode::Noop,
                cycle: 1,
            }),
            insn if insn.starts_with("addx") => {
                let (_, addend) = insn.split_once(' ').expect("addx should have addend");
                Ok(Instruction {
                    op: OpCode::Addx(addend.parse().expect("addx addend should be an i32")),
                    cycle: 2,
                })
            }
            _ => Err(ParseInstructionError),
        }
    }
}
