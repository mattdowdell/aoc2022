//! Solution for [Advent of Code 2022 - Day 11][1].
//!
//! [1]: https://adventofcode.com/2022/day/11

mod models;

use std::io::{self, BufRead, Lines};

use models::{parse_line, Monkey, MonkeyBuilder};

struct Answer {
    part1: u64,
    part2: u64,
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
    let mut monkeys = Vec::new();
    let mut builder = MonkeyBuilder::new();

    for line in lines {
        let line = line?;

        if line.is_empty() {
            monkeys.push(Monkey::try_from(builder)?);
            builder = MonkeyBuilder::new();
            continue;
        }

        parse_line(line.as_str(), &mut builder)?;
    }

    if let Ok(monkey) = Monkey::try_from(builder) {
        monkeys.push(monkey);
    }

    let mut monkeys2 = monkeys.clone();

    // get the lowest common multiple across the `test` value of all monkeys
    let lcm = monkeys.iter().map(|m| m.test()).product();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            loop {
                let monkey = monkeys.get_mut(i).unwrap();

                if let Some((item, to)) = monkey.turn() {
                    monkeys.get_mut(to).unwrap().catch(item);
                } else {
                    break;
                }
            }
        }
    }

    let mut inspected: Vec<_> = monkeys.iter().map(|m| m.inspected()).collect();
    inspected.sort();

    let part1 = inspected.pop().unwrap() * inspected.pop().unwrap();

    for _ in 0..10_000 {
        for i in 0..monkeys2.len() {
            loop {
                let monkey = monkeys2.get_mut(i).unwrap();

                if let Some((item, to)) = monkey.turn_lcm(lcm) {
                    monkeys2.get_mut(to).unwrap().catch(item);
                } else {
                    break;
                }
            }
        }
    }

    let mut inspected: Vec<_> = monkeys2.iter().map(|m| m.inspected()).collect();
    inspected.sort();

    let part2 = inspected.pop().unwrap() * inspected.pop().unwrap();

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

        assert_eq!(answer.part1, 10605);
        assert_eq!(answer.part2, 2713310158);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 110888);
        assert_eq!(answer.part2, 25590400731);

        Ok(())
    }
}

