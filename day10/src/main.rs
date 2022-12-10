//! Solution for [Advent of Code 2022 - Day 10][1].
//!
//! [1]: https://adventofcode.com/2022/day/10

mod models;

use std::io::{self, BufRead, Lines};
use std::str::FromStr;

use models::{Cpu, Operation};

struct Answer {
    part1: i32,
    part2: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let answer = run(io::stdin().lock().lines())?;

    println!("Solution 1: {}", answer.part1);
    println!("Solution 2:\n{}", answer.part2);

    Ok(())
}

fn run<T>(lines: Lines<T>) -> Result<Answer, Box<dyn std::error::Error>>
where
    T: BufRead,
{
    let mut cpu = Cpu::new();

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let op = Operation::from_str(line.as_str())?;
        cpu.add(op);
    }

    let mut part1 = 0;
    let mut part2 = String::new();

    for (i, x) in cpu.execute().iter().enumerate() {
        let sprite_over_cursor = (x - ((i as i32) % 40)).abs() <= 1;
        part2.push(if sprite_over_cursor { '#' } else { '.' });

        if (i as i32 + 1) % 40 == 0 {
            part2.push('\n');
        }

        if ((i as i32) + 1 - 20) % 40 == 0 {
            part1 += (i as i32 + 1) * x;
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

        assert_eq!(answer.part1, 13140);
        assert_eq!(
            answer.part2.trim(),
            [
                "##..##..##..##..##..##..##..##..##..##..",
                "###...###...###...###...###...###...###.",
                "####....####....####....####....####....",
                "#####.....#####.....#####.....#####.....",
                "######......######......######......####",
                "#######.......#######.......#######.....",
            ]
            .join("\n")
        );

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 16480);
        assert_eq!(
            answer.part2.trim(),
            [
                "###..#....####.####.#..#.#....###..###..",
                "#..#.#....#....#....#..#.#....#..#.#..#.",
                "#..#.#....###..###..#..#.#....#..#.###..",
                "###..#....#....#....#..#.#....###..#..#.",
                "#....#....#....#....#..#.#....#....#..#.",
                "#....####.####.#.....##..####.#....###..",
            ]
            .join("\n")
        );

        Ok(())
    }
}
