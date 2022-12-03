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

            // match chars across the groups of 3 lines
            if badges.is_empty() {
                badges = line.chars().collect();
            } else {
                let filter: HashSet<char> = line.chars().collect();
                badges.retain(|c| filter.contains(&c));
            }

            // find common chars in the 2 halves of the line
            let (first, second) = line.split_at(line.len() / 2);

            let chars: HashSet<char> = first.chars().collect();
            let common = second.chars().find(|c| chars.contains(&c)).ok_or("no common char found")?;

            total1 += char_to_priority(common)?;
        }

        total2 += char_to_priority(*badges.iter().next().ok_or("no common badge found")?)?;
        badges.clear();
    }

    println!("Solution 1: {}", total1);
    println!("Solution 2: {}", total2);

    Ok(())
}

// Map an ASCII alphabetic char to a priority, where a-z are priorities 1-26, while A-Z are
// priorities 27-52. Chars that are not ASCII alphabetic cause an error to be returned.
fn char_to_priority(c: char) -> Result<u32, &'static str> {
    match c {
        'A'..='Z' => Ok((c as u32) - 38),
        'a'..='z' => Ok((c as u32) - 96),
        _ => Err("failed to map char to priority"),
    }
}
