//! Solution for [Advent of Code 2022 - Day 15][1].
//!
//! [1]: https://adventofcode.com/2022/day/15

// 5764145 too high

mod models;

use std::io::{self, BufRead, Lines};

use models::Map;

struct Answer {
    part1: i32,
    part2: i128,
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

    let map = Map::from(lines)?;

    let part1 = map.covers(2_000_000);
    let mut part2 = 0;

    for y in 0..=4_000_000 {
        if let Some(x) = map.find_hole(y) {
            part2 = x * 4_000_000 + y as i128;
            break;
        }
    }

    // for (key, item) in grid.iter() {
    //     if item == &Item::Covered {
    //         // println!("{:?} is covered", key);
    //         part1 += 1;
    //     }
    // }

    // should get 26
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
    #[ignore]
    fn test_sample() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("sample.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 0);
        // assert_eq!(answer.part2, 0);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 0);
        // assert_eq!(answer.part2, 0);

        Ok(())
    }
}
