//! Solution for [Advent of Code 2022 - Day 4][1].
//!
//! [1]: https://adventofcode.com/2022/day/4

#![feature(test)]

extern crate test;

mod models;

use models::Range;
use std::io::{self, BufRead, Lines};

struct Answer {
    part1: u32,
    part2: u32,
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
    let mut total1 = 0;
    let mut total2 = 0;

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let (left, right) = line.split_once(',').ok_or("failed to split line")?;

        let first = Range::try_from(left)?;
        let second = Range::try_from(right)?;

        if first.contains(&second) || second.contains(&first) {
            total1 += 1;
        }

        if first.overlaps(&second) {
            total2 += 1;
        }
    }

    Ok(Answer {
        part1: total1,
        part2: total2,
    })
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::io::BufRead;
    use test::Bencher;

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

        assert_eq!(answer.part1, 2);
        assert_eq!(answer.part2, 4);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 509);
        assert_eq!(answer.part2, 870);

        Ok(())
    }

    #[bench]
    fn bench_sample(b: &mut Bencher) {
        b.iter(|| {
            let lines = lines!("sample.txt").unwrap();
            run(lines).unwrap();
        })
    }

    #[bench]
    fn bench_user_specific(b: &mut Bencher) {
        b.iter(|| {
            let lines = lines!("input.txt").unwrap();
            run(lines).unwrap();
        })
    }
}
