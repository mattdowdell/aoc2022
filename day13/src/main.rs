//! Solution for [Advent of Code 2022 - Day 13][1].
//!
//! [1]: https://adventofcode.com/2022/day/13

use std::cmp::Ordering;
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
    let mut left = None;
    let mut right = None;
    let mut pairs = Vec::new();
    let mut packets = Vec::new();

    let (divider1, _) = parse_line("[[2]]")?;
    let (divider2, _) = parse_line("[[6]]")?;

    packets.push(divider1.clone());
    packets.push(divider2.clone());

    for line in lines {
        let line = line?;

        if line.is_empty() {
            let (a, b) = left.zip(right).ok_or("pair not completed")?;
            pairs.push(Pair { left: a, right: b });

            left = None;
            right = None;

            continue;
        }

        if left.is_none() {
            let (parsed, _) = parse_line(line.as_str())?;
            packets.push(parsed.clone());
            left = Some(parsed);
            continue;
        }

        if right.is_none() {
            let (parsed, _) = parse_line(line.as_str())?;
            packets.push(parsed.clone());
            right = Some(parsed);
            continue;
        }

        // shouldn't get here
        unimplemented!();
    }

    if let Some((a, b)) = left.zip(right) {
        pairs.push(Pair { left: a, right: b });
    }

    let part1 = pairs
        .iter()
        .enumerate()
        .map(|(i, pair)| if pair.is_ordered() { i + 1 } else { 0 })
        .sum();

    packets.sort();

    let part2 = packets
        .into_iter()
        .enumerate()
        .filter(|(_, item)| *item == divider1 || *item == divider2)
        .map(|(i, _)| i + 1)
        .product();

    Ok(Answer { part1, part2 })
}

#[derive(Debug)]
struct Pair {
    pub left: Item,
    pub right: Item,
}

impl Pair {
    ///
    pub fn is_ordered(&self) -> bool {
        self.left.cmp(&self.right).is_lt()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    Value(u32),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => todo!(),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Value(x), Self::Value(y)) => x.partial_cmp(y),
            (Self::Value(x), Self::List(y)) => vec![Self::Value(*x)].partial_cmp(y),
            (Self::List(x), Self::Value(y)) => x.partial_cmp(&vec![Self::Value(*y)]),
            (Self::List(x), Self::List(y)) => {
                for (a, b) in x.iter().zip(y) {
                    match a.partial_cmp(b) {
                        Some(Ordering::Equal) => {}
                        None => {}
                        ordering => return ordering,
                    }
                }

                x.len().partial_cmp(&y.len())
            }
        }
    }
}

fn parse_line(line: &str) -> Result<(Item, &str), Box<dyn std::error::Error>> {
    let mut line = line.strip_prefix('[').unwrap_or(line);
    let mut items = Vec::new();

    if line.is_empty() {
        return Ok((Item::List(items), line));
    }

    loop {
        if line.strip_prefix('[').is_some() {
            let (head, tail) = parse_line(line)?;
            items.push(head);
            line = tail.strip_prefix(',').unwrap_or(tail);
            continue;
        }

        if let Some((head, tail)) = line.split_once(',') {
            if !head.contains('[') && !head.contains(']') {
                items.push(Item::Value(
                    head.parse().map_err(|e| format!("{} {:?}", e, head))?,
                ));
                line = tail;
                continue;
            }
        }

        let (head, tail) = line.split_once(']').ok_or("missing closing ]")?;
        if !head.is_empty() {
            items.push(Item::Value(
                head.parse().map_err(|e| format!("{} ({})", e, head))?,
            ));
        }

        line = tail;
        break;
    }

    Ok((Item::List(items), line))
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
        assert_eq!(answer.part2, 140);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 5252);
        assert_eq!(answer.part2, 20592);

        Ok(())
    }
}
