//! Solution for [Advent of Code 2022 - Day 12][1].
//!
//! [1]: https://adventofcode.com/2022/day/12

mod models;

use std::io::{self, BufRead, Lines};

use models::Graph;

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
    let graph = Graph::from(lines)?;

    Ok(Answer {
        part1: graph.reverse_flood_fill(true /*target_start*/).unwrap(),
        part2: graph.reverse_flood_fill(false /*target_start*/).unwrap(),
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

        assert_eq!(answer.part1, 31);
        assert_eq!(answer.part2, 29);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 440);
        assert_eq!(answer.part2, 439);

        Ok(())
    }
}
