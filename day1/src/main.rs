//! Solution for [Advent of Code 2022 - Day 1][1].
//!
//! [1]: https://adventofcode.com/2022/day/1

use std::cmp::Reverse;
use std::io::{self, BufRead, Lines};

struct Answer {
    pub part1: u32,
    pub part2: u32,
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
    let mut totals = Vec::new();
    let mut current = 0;

    for line in lines {
        let line = line?;

        if line.is_empty() {
            totals.push(current);
            current = 0;
        } else {
            let value: u32 = line.parse()?;
            current += value;
        }
    }

    if current > 0 {
        totals.push(current);
    }

    totals.sort_by_key(|x| Reverse(*x));

    Ok(Answer {
        part1: *totals.first().ok_or("Missing first value")?,
        part2: totals.iter().take(3).sum::<u32>(),
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

        assert_eq!(answer.part1, 24000);
        assert_eq!(answer.part2, 45000);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 67633);
        assert_eq!(answer.part2, 199628);

        Ok(())
    }
}
