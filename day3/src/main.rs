//! Solution for [Advent of Code 2022 - Day 3][1].
//!
//! [1]: https://adventofcode.com/2022/day/3

use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = io::stdin().lock().lines();
    let mut total1 = 0;
    let mut total2 = 0;

    'outer: loop {
        let mut badges = HashSet::new();

        for _ in 0..3 {
            let line = match lines.next() {
                Some(x) => x?,
                None => break 'outer,
            };

            if line.is_empty() {
                continue;
            }

            if badges.is_empty() {
                badges = line.chars().collect();
            } else {
                let filter: HashSet<char> = line.chars().collect();
                badges.retain(|c| filter.contains(&c));
            }

            let (first, second) = line.split_at(line.len() / 2);

            let chars: HashSet<char> = first.chars().collect();
            let common = second.chars().find(|c| chars.contains(&c)).ok_or("no common char found")?;

            total1 += char_to_priority(common);
        }

        total2 += char_to_priority(*badges.iter().next().unwrap());
        badges.clear();
    }

    println!("Solution 1: {}", total1);
    println!("Solution 2: {}", total2);

    Ok(())
}

fn char_to_priority(c: char) -> u32 {
    match c {
        'A'..='Z' => (c as u32) - 38,
        'a'..='z' => (c as u32) - 96,
        _ => todo!(),
    }
}
