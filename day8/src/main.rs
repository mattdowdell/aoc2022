//! Solution for [Advent of Code 2022 - Day 8][1].
//!
//! [1]: https://adventofcode.com/2022/day/8

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
    let mut data = Vec::new();

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        data.push(line.chars().map(|c| c as u32 - 48).collect::<Vec<u32>>());
    }

    let height = data.len();
    let width = data[0].len();

    let mut part1 = 0;
    let mut part2 = 0;

    for row in 0..height {
        for column in 0..width {
            let mut visibility = 4;

            // view to left
            let mut left = column;
            for view_column in (0..column).rev() {
                if data[row][column] <= data[row][view_column] {
                    visibility -= 1;
                    left = column - view_column;
                    break;
                }
            }

            // view to right
            let mut right = width - 1 - column;
            for view_column in (column + 1)..width {
                if data[row][column] <= data[row][view_column] {
                    visibility -= 1;
                    right = view_column - column;
                    break;
                }
            }

            // view up
            let mut up = row;
            for view_row in (0..row).rev() {
                if data[row][column] <= data[view_row][column] {
                    visibility -= 1;
                    up = row - view_row;
                    break;
                }
            }

            // view down
            let mut down = height - 1 - row;
            for view_row in (row + 1)..height {
                if data[row][column] <= data[view_row][column] {
                    visibility -= 1;
                    down = view_row - row;
                    break;
                }
            }

            if visibility > 0 {
                part1 += 1;
            }

            let scenic_score = left * right * up * down;
            if scenic_score > part2 {
                part2 = scenic_score;
            }
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

        assert_eq!(answer.part1, 21);
        assert_eq!(answer.part2, 8);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 1794);
        assert_eq!(answer.part2, 199272);

        Ok(())
    }
}
