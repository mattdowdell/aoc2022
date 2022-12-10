//! Solution for [Advent of Code 2022 - Day 10][1].
//!
//! [1]: https://adventofcode.com/2022/day/10

use std::io::{self, BufRead, Lines};
use std::str::FromStr;
use std::collections::VecDeque;

struct Answer {
    part1: i32,
    part2: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let answer = run(io::stdin().lock().lines())?;

    println!("Solution 1: {}", answer.part1);
    println!("Solution 2: {}", answer.part2);

    Ok(())
}

fn run<T>(lines: Lines<T>) -> Result<Answer, Box<dyn std::error::Error>>
where
    T: BufRead,
{
    let mut operations = VecDeque::new();

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let op = Op::from_str(line.as_str())?;
        operations.push_back(op);
    }

    let mut cpu = CPU::new(operations);
    let mut part1 = 0;

    for cycle in vec![20, 60, 100, 140, 180, 220] {
        part1 += cycle as i32 * cpu.x_at(cycle);
    }

    Ok(Answer{part1, part2: 0})
}

struct CPU {
    x: i32,
    cycle: usize,
    operations: VecDeque<Op>,
}

impl CPU {
    pub fn new(operations: VecDeque<Op>) -> Self {
        Self {
            x: 1,
            cycle: 0,
            operations,
        }
    }

    pub fn x_at(&mut self, cycle: usize) -> i32 {
        while self.cycle < cycle - 1 {
            self.process();
        }

        self.x
    }

    fn process(&mut self) {
        if let Some(op) = self.operations.pop_front() {
            // println!("{:?} ({})", op, self.x);
            match op {
                Op::Noop => {},
                Op::AddX { value, started: false } => {
                    self.operations.push_front(Op::AddX{ value, started: true })
                },
                Op::AddX { value, started: true } => {
                    self.x += value;
                }
            }
        }

        self.cycle += 1;
    }
}

#[derive(Debug)]
enum Op {
    Noop,
    AddX{ value: i32, started: bool },
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Self::Noop);
        }

        if let Some(x) = s.strip_prefix("addx ") {
            return Ok(Self::AddX{ value: x.parse().unwrap(), started: false });
        }

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::io::BufRead;

    macro_rules! file {
        ($file:literal) => {
            std::fs::File::open(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join($file))
        };
    }

    macro_rules! lines {
        ($file:literal) => {
            file!($file).map(|f| std::io::BufReader::new(f).lines())
        };
    }

    #[test]
    fn test_sample() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("sample.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 13140);
        // assert_eq!(answer.part2, 8);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 16480);
        // assert_eq!(answer.part2, 199272);

        Ok(())
    }
}

