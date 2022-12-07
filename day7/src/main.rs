//! Solution for [Advent of Code 2022 - Day 7][1].
//!
//! [1]: https://adventofcode.com/2022/day/7

use std::collections::HashMap;
use std::io::{self, BufRead, Lines};

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

fn run<T>(mut lines: Lines<T>) -> Result<Answer, Box<dyn std::error::Error>>
where
    T: BufRead,
{
    let fs = enter_dir(&mut lines)?;
    let mut totals = Vec::new();

    dir_size(&fs, &mut totals);

    let mut part1 = 0;

    for total in totals.iter() {
        if *total < 100000 {
            part1 += *total;
        }
    }

    let size = *totals.last().unwrap();
    let unused = 70000000 - size;
    let to_free = 30000000 - unused;
    let mut part2 = size;

    for total in totals.iter() {
        if *total < part2 && *total > to_free {
            part2 = *total;
        }
    }

    println!("size: {}", size);
    println!("unused: {}", unused);
    println!("to free: {}", to_free);
    println!("to delete: {}", part2);

    Ok(Answer { part1, part2 })
}

#[derive(Debug)]
enum Entry {
    Dir(HashMap<String, Entry>),
    File(u64),
}

fn enter_dir<T>(lines: &mut Lines<T>) -> Result<HashMap<String, Entry>, Box<dyn std::error::Error>>
where
    T: BufRead,
{
    let mut cwd: HashMap<String, Entry> = HashMap::new();

    loop {
        let line = read_line(lines)?;
        if line.is_none() {
            break;
        }
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        if line == "$ ls" {
            continue;
        }

        if let Some(dir) = line.strip_prefix("$ cd ") {
            if dir == "/" {
                continue;
            }

            if dir == ".." {
                break;
            }

            cwd.insert(dir.to_string(), Entry::Dir(enter_dir(lines)?));
            continue;
        }

        if line.strip_prefix("dir ").is_some() {
            continue;
        }

        let (size, name) = line.split_once(' ').unwrap();
        cwd.insert(name.to_string(), Entry::File(size.parse()?));
    }

    Ok(cwd)
}

fn read_line<T>(lines: &mut Lines<T>) -> Result<Option<String>, Box<dyn std::error::Error>>
where
    T: BufRead,
{
    if let Some(line) = lines.next() {
        Ok(Some(line?))
    } else {
        Ok(None)
    }
}

fn dir_size(dir: &HashMap<String, Entry>, totals: &mut Vec<u64>) -> u64 {
    let mut total: u64 = 0;

    for (_, entry) in dir.iter() {
        total += match entry {
            Entry::Dir(dir) => dir_size(dir, totals),
            Entry::File(size) => *size,
        };
    }

    totals.push(total);
    total
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

        assert_eq!(answer.part1, 95437);
        assert_eq!(answer.part2, 24933642);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 1243729);
        assert_eq!(answer.part2, 4443914);

        Ok(())
    }
}
