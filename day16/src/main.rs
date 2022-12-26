//! Solution for [Advent of Code 2022 - Day 15][1].
//!
//! [1]: https://adventofcode.com/2022/day/15

mod models;

use std::io::{self, BufRead, Lines};

use models::Volcano;

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
    let mut volcano = Volcano::from(lines)?;
    let mut pressure = 0;

    for i in 1..=30 {
        pressure += volcano.run(30 - i);
    }

    Ok(Answer {
        part1: pressure,
        part2: 0,
    })
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

        assert_eq!(answer.part1, 1651);
        // assert_eq!(answer.part2, 0);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 0);
        // assert_eq!(answer.part2, 13639962836448);

        Ok(())
    }
}
