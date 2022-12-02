//! Solution for [Advent of Code 2022 - Day 2][1].
//!
//! [1]: https://adventofcode.com/2022/day/2

mod models;

use std::io::{self, BufRead};
use models::{Shape, Outcome}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut total1 = 0;
    let mut total2 = 0;

    for line in stdin.lock().lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let (first, last) = line.split_at(1);
        let shape1 = Shape::try_from(first)?;
        let shape2 = Shape::try_from(last)?;
        let outcome = Outcome::try_from(last)?;

        total1 += shape2.outcome(shape1).score() + shape2.score();
        total2 += outcome.score() + shape1.match_outcome(outcome).score();
    }

    println!("Solution 1: {}", total1);
    println!("Solution 2: {}", total2);

    Ok(())
}
