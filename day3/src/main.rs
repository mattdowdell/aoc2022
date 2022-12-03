//! Solution for [Advent of Code 2022 - Day 3][1].
//!
//! [1]: https://adventofcode.com/2022/day/3

use std::collections::HashSet;
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

fn run<T>(mut lines: Lines<T>) -> Result<Answer, Box<dyn std::error::Error>>
where
    T: BufRead,
{
    let mut total1 = 0;
    let mut total2 = 0;

    'outer: loop {
        let mut badges = HashSet::new();

        for _ in 0..3 {
            let line = match lines.next() {
                Some(x) => x?,
                None => break 'outer,
            };

            if line.is_empty() {
                continue;
            }

            // match chars across the groups of 3 lines
            if badges.is_empty() {
                badges = line.chars().collect();
            } else {
                let filter: HashSet<char> = line.chars().collect();
                badges.retain(|c| filter.contains(c));
            }

            // find common chars in the 2 halves of the line
            let (first, second) = line.split_at(line.len() / 2);

            let chars: HashSet<char> = first.chars().collect();
            let common = second
                .chars()
                .find(|c| chars.contains(c))
                .ok_or("no common char found")?;

            total1 += char_to_priority(common)?;
        }

        total2 += char_to_priority(*badges.iter().next().ok_or("no common badge found")?)?;
        badges.clear();
    }

    Ok(Answer {
        part1: total1,
        part2: total2,
    })
}

// Map an ASCII alphabetic char to a priority, where a-z are priorities 1-26, while A-Z are
// priorities 27-52. Chars that are not ASCII alphabetic cause an error to be returned.
fn char_to_priority(c: char) -> Result<u32, &'static str> {
    match c {
        'A'..='Z' => Ok((c as u32) - 38),
        'a'..='z' => Ok((c as u32) - 96),
        _ => Err("failed to map char to priority"),
    }
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

        assert_eq!(answer.part1, 157);
        assert_eq!(answer.part2, 70);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 7553);
        assert_eq!(answer.part2, 2758);

        Ok(())
    }
}
