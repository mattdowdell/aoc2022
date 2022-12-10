//! Solution for [Advent of Code 2022 - Day 9][1].
//!
//! [1]: https://adventofcode.com/2022/day/9

mod models;

use models::{Move, Rope};
use std::io::{self, BufRead, Lines};

struct Answer {
    part1: usize,
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
    let mut rope1 = Rope::new(2);
    let mut rope2 = Rope::new(10);

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let mov = Move::try_from(line.as_str())?;
        rope1.apply(mov);
        rope2.apply(mov);
    }

    let part1 = rope1.tail_locations();
    let part2 = rope2.tail_locations();

    Ok(Answer { part1, part2 })
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

        assert_eq!(answer.part1, 13);

        Ok(())
    }

    #[test]
    fn test_sample2() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("sample2.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part2, 36);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 6271);
        assert_eq!(answer.part2, 2458);

        Ok(())
    }
}
