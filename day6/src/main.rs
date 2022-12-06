//! Solution for [Advent of Code 2022 - Day 6][1].
//!
//! [1]: https://adventofcode.com/2022/day/6

use std::collections::HashSet;
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

fn run<T>(mut lines: Lines<T>) -> Result<Answer, Box<dyn std::error::Error>>
where
    T: BufRead,
{
    let line = lines
        .next()
        .ok_or("missing input line")?
        .map_err(|_| "failed to read input line")?;

    let chars: Vec<_> = line.chars().collect();
    let mut part1 = 0;
    let mut part2 = 0;

    for (i, window) in chars.windows(4).enumerate() {
        let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if set.len() == 4 {
            part1 = i + 4;
            break;
        }
    }

    for (i, window) in chars.windows(14).enumerate() {
        let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if set.len() == 14 {
            part2 = i + 14;
            break;
        }
    }

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

        assert_eq!(answer.part1, 7);
        assert_eq!(answer.part2, 19);

        Ok(())
    }

    #[test]
    fn test_extra_sample_1() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("extra-sample1.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 5);
        assert_eq!(answer.part2, 23);

        Ok(())
    }

    #[test]
    fn test_extra_sample_2() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("extra-sample2.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 6);
        assert_eq!(answer.part2, 23);

        Ok(())
    }

    #[test]
    fn test_extra_sample_3() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("extra-sample3.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 10);
        assert_eq!(answer.part2, 29);

        Ok(())
    }

    #[test]
    fn test_extra_sample_4() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("extra-sample4.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 11);
        assert_eq!(answer.part2, 26);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 1623);
        assert_eq!(answer.part2, 3774);

        Ok(())
    }
}
