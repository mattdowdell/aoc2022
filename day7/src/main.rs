//! Solution for [Advent of Code 2022 - Day 7][1].
//!
//! [1]: https://adventofcode.com/2022/day/7

use std::collections::HashMap;
use std::io::{self, BufRead, Lines};
use std::path::PathBuf;

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
    let mut dir_sizes = calc_dir_sizes(&mut lines)?;
    dir_sizes.sort();

    let part1 = dir_sizes.iter().filter(|&x| *x < 100_000).sum();

    let root_size = *dir_sizes.last().unwrap();
    let to_free = 30_000_000 - (70_000_000 - root_size);

    let part2 = *dir_sizes.iter().find(|&x| *x > to_free).unwrap();

    Ok(Answer { part1, part2 })
}

// Calculate the sizes of the directories as given by the input.
fn calc_dir_sizes<T>(lines: &mut Lines<T>) -> Result<Vec<u64>, Box<dyn std::error::Error>>
where
    T: BufRead,
{
    let mut fs: HashMap<PathBuf, Vec<u64>> = HashMap::new();
    let mut path = PathBuf::new();

    for line in lines {
        let line = line?;

        if line.is_empty() {
            break;
        }

        // ignore ls commands, no useful info in them
        if line == "$ ls" {
            continue;
        }

        // change directory to get some other sizes
        if let Some(dir) = line.strip_prefix("$ cd ") {
            if dir == ".." {
                // store the size of the child dir in the parent
                let size = if let Some(sizes) = fs.get(&path) {
                    sizes.iter().sum()
                } else {
                    0
                };

                path = path.parent().unwrap().to_path_buf();
                fs.get_mut(&path).unwrap().push(size);
                continue;
            }

            path.push(dir);

            // make sure we have some to store file sizes in for this dir
            if fs.get(&path).is_none() {
                fs.insert(path.clone(), Vec::new());
            }

            continue;
        }

        // ignore dir listings, no useful info in them
        if line.starts_with("dir ") {
            continue;
        }

        let (size, _) = line.split_once(' ').unwrap();
        fs.get_mut(&path).unwrap().push(size.parse()?);
    }

    // replicate any missing `$ cd ..` to make sure the root dir has all child dir sizes
    loop {
        if let Some(parent) = path.parent() {
            let size = fs.get(&path).map(|x| x.iter().sum()).unwrap_or(0);

            path = parent.to_path_buf();
            fs.get_mut(&path).unwrap().push(size);

            continue;
        }

        break;
    }

    // sum all the child dir sizes to make later calculations simpler
    Ok(fs.values().map(|v| v.iter().sum()).collect())
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
