//! Solution for [Advent of Code 2022 - Day 2][1].
//!
//! [1]: https://adventofcode.com/2022/day/2

mod models;

use models::{Outcome, Shape};
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

        let (first, last) = line.split_at(1);
        let shape1 = Shape::try_from(first)?;
        let shape2 = Shape::try_from(last)?;
        let outcome = Outcome::try_from(last)?;

        total1 += shape2.outcome(shape1).score() + shape2.score();
        total2 += outcome.score() + shape1.match_outcome(outcome).score();
    }

    Ok(Answer {
        part1: total1,
        part2: total2,
    })
}

#[cfg(test)]
mod test {
    use super::run;
    use std::io::BufRead;

    macro_rules! lines {
        ($file:literal) => {
            std::fs::File::open(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join($file))
                .map(|f| std::io::BufReader::new(f).lines())
        };
    }

    #[test]
    fn test_sample() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("sample.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 15);
        assert_eq!(answer.part2, 12);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 15632);
        assert_eq!(answer.part2, 14416);

        Ok(())
    }
}
