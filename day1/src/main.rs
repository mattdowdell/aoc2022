//! Solution for [Advent of Code 2022 - Day 1][1].
//!
//! [1]: https://adventofcode.com/2022/day/1

use std::cmp::Reverse;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut totals = Vec::new();
    let mut current = 0;

    for line in stdin.lock().lines() {
        let line = line?;

        if line.is_empty() {
            totals.push(current);
            current = 0;
        } else {
            let value: u32 = line.parse()?;
            current += value;
        }
    }

    totals.sort_by_key(|x| Reverse(*x));

    println!(
        "Solution 1: {}",
        totals.first().ok_or("Missing first value")?
    );
    println!("Solution 2: {}", totals.iter().take(3).sum::<u32>());

    Ok(())
}
