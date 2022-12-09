//! Solution for [Advent of Code 2022 - Day 7][1].
//!
//! [1]: https://adventofcode.com/2022/day/7

use std::collections::HashSet;
use std::io::{self, BufRead, Lines};
use std::fmt;

struct Answer {
    part1: usize,
    part2: usize,
}

struct Grid {
    knots: Vec<(i32, i32)>,
    path: HashSet<(i32, i32)>,
}

impl Grid {
    ///
    pub fn new(count: usize) -> Self {
        let mut path = HashSet::new();
        path.insert((0, 0));

        let mut knots = Vec::new();

        for _ in 0..count {
            knots.push((0, 0));
        }

        Self {
            knots,
            path,
        }
    }

    ///
    pub fn apply(&mut self, mov: Move) {
        match mov {
            Move::Right(count) => {
                for _ in 0..count {
                    let (mut x, y) = self.knots[0];
                    x += 1;
                    self.knots[0] = (x, y);

                    for i in 1..self.knots.len() {
                        self.move_tail(i);
                    }
                }
            },
            Move::Left(count) => {
                for _ in 0..count {
                    let (mut x, y) = self.knots[0];
                    x -= 1;
                    self.knots[0] = (x, y);

                    for i in 1..self.knots.len() {
                        self.move_tail(i);
                    }
                }
            },
            Move::Up(count) => {
                for _ in 0..count {
                    let (x, mut y) = self.knots[0];
                    y += 1;
                    self.knots[0] = (x, y);

                    for i in 1..self.knots.len() {
                        self.move_tail(i);
                    }
                }
            },
            Move::Down(count) => {
                for _ in 0..count {
                    let (x, mut y) = self.knots[0];
                    y -= 1;
                    self.knots[0] = (x, y);

                    for i in 1..self.knots.len() {
                        self.move_tail(i);
                    }
                }
            },
        }
    }

    ///
    fn move_tail(&mut self, index: usize) {
        let (mut tail_x, mut tail_y) = self.knots[index];
        let (head_x, head_y) = self.knots[index - 1];

        if (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
            tail_x += (head_x - tail_x).signum();
            tail_y += (head_y - tail_y).signum();
        }

        self.knots[index] = (tail_x, tail_y);
        self.path.insert((tail_x, tail_y));
    }

    ///
    pub fn tail_locations(&self) -> usize {
        let mut locations = self.path.iter().collect::<Vec<_>>();
        locations.sort();
        self.path.len()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut locations = self.path.iter().collect::<Vec<_>>();
        locations.sort();

        let (min_x, min_y) = locations.first().unwrap();
        let (max_x, max_y) = locations.last().unwrap();

        let mut data = String::new();

        for y in (*min_y..*max_y + 2).rev() {
            for x in *min_x..*max_x + 2 {
                if self.path.contains(&(x, y)) {
                    data.push('#');
                } else {
                    data.push('.');
                }
            }

            data.push('\n');
        }

        write!(f, "{}", data)
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Right(u32),
    Left(u32),
    Up(u32),
    Down(u32),
}

impl TryFrom<&str> for Move {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, count) = value.split_once(' ').unwrap();

        match direction {
            "R" => Ok(Self::Right(count.parse().unwrap())),
            "L" => Ok(Self::Left(count.parse().unwrap())),
            "U" => Ok(Self::Up(count.parse().unwrap())),
            "D" => Ok(Self::Down(count.parse().unwrap())),
            _ => todo!(),
        }
    }
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
    let mut grid1 = Grid::new(2);
    let mut grid2 = Grid::new(9);

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let mov = Move::try_from(line.as_str())?;
        grid1.apply(mov);
        grid2.apply(mov);
    }

    println!("{}", grid2);

    let part1 = grid1.tail_locations();
    // let part2 = grid2.tail_locations(); // TODO: fixme

    Ok(Answer{part1, part2: 0})
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
        // assert_eq!(answer.part2, 8);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_sample2() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("sample2.txt")?;
        let _answer = run(lines)?;

        // assert_eq!(answer.part1, 21);
        // assert_eq!(answer.part2, 8);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 6271);
        // assert_eq!(answer.part2, 199272);

        Ok(())
    }
}
