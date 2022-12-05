//! Solution for [Advent of Code 2022 - Day 4][1].
//!
//! [1]: https://adventofcode.com/2022/day/4

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::io::{self, BufRead, Lines};

lazy_static! {
    static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

struct Answer {
    part1: String,
    part2: String,
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
    let mut data1: Vec<VecDeque<char>> = Vec::new();
    let mut data2: Vec<VecDeque<char>> = Vec::new();
    let mut parsing_data = true;

    for line in lines {
        let line = line?;

        if line.is_empty() {
            parsing_data = false;
            data2 = data1.clone();
            continue;
        }

        if parsing_data {
            parse_data(&line, &mut data1);
        } else {
            let mov = Move::try_from(line.as_str())?;

            mov.apply(&mut data1)?;
            mov.batch_apply(&mut data2);
        }
    }

    Ok(Answer {
        part1: data1.iter().map(|x| x.front().unwrap_or(&' ')).collect(),
        part2: data2.iter().map(|x| x.front().unwrap_or(&' ')).collect(),
    })
}

fn parse_data(input: &str, data: &mut Vec<VecDeque<char>>) {
    let mut input = input;
    let mut i = 0;

    loop {
        if input.trim().is_empty() {
            break;
        }

        let (left, right) = if input.len() > 3 {
            input.split_at(4)
        } else {
            (input, "")
        };

        if let Some(c) = parse_item(left) {
            while data.len() < (i + 1) {
                data.push(VecDeque::new())
            }

            data[i].push_back(c);
        }

        i += 1;
        input = right;
    }
}

fn parse_item(input: &str) -> Option<char> {
    input.trim().strip_prefix('[')?.chars().next()
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    ///
    pub fn apply(
        &self,
        data: &mut Vec<VecDeque<char>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while data.len() < (self.to + 1) {
            data.push(VecDeque::new());
        }

        for _ in 0..self.count {
            let c = data[self.from].pop_front().ok_or("missing data in stack")?;
            data[self.to].push_front(c);
        }

        Ok(())
    }

    ///
    pub fn batch_apply(&self, data: &mut Vec<VecDeque<char>>) {
        while data.len() < (self.to + 1) {
            data.push(VecDeque::new());
        }

        let mut moved: Vec<char> = data[self.from].drain(0..self.count).collect();

        while let Some(c) = moved.pop() {
            data[self.to].push_front(c);
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let captures = RE
            .captures_iter(input)
            .next()
            .ok_or("failed to parse move")?;

        Ok(Move {
            count: captures[1].parse().map_err(|_| "failed to parse count")?,
            from: captures[2]
                .parse::<usize>()
                .map_err(|_| "failed to parse from")?
                - 1,
            to: captures[3]
                .parse::<usize>()
                .map_err(|_| "failed to parse to")?
                - 1,
        })
    }
}
