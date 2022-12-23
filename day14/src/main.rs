//! Solution for [Advent of Code 2022 - Day 14][1].
//!
//! [1]: https://adventofcode.com/2022/day/14

use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
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

fn run<T>(lines: Lines<T>) -> Result<Answer, Box<dyn std::error::Error>>
where
    T: BufRead,
{
    let mut map = Map::new(Coordinate::new(500, 0));

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        map.parse_line(line.as_str())?;
    }

    let part1 = map.clone().fill(false /*with_floor*/);
    let part2 = map.fill(true /*with_floor*/);

    Ok(Answer { part1, part2 })
}

#[derive(Clone, Debug)]
pub struct Map {
    start: Coordinate,
    map: HashMap<Coordinate, Item>,
}

impl Map {
    ///
    pub fn new(start: Coordinate) -> Self {
        Self {
            start,
            map: HashMap::new(),
        }
    }

    ///
    pub fn parse_line(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut edges = Vec::new();

        for coord in line.split(" -> ") {
            let (x, y) = coord.split_once(',').ok_or("failed to split coordinate")?;
            edges.push(Coordinate::new(x.parse()?, y.parse()?));
        }

        for pair in edges.windows(2) {
            let bridge: HashMap<_, _> = pair[0]
                .bridge(&pair[1])
                .into_iter()
                .map(|c| (c, Item::Block))
                .collect();
            self.map.extend(&bridge);
        }

        Ok(())
    }

    //
    fn floor(&self) -> usize {
        let mut floor = 0;

        for coord in self.map.keys() {
            if coord.y > floor {
                floor = coord.y
            }
        }

        floor + 2
    }

    ///
    pub fn fill(&mut self, with_floor: bool) -> u32 {
        let floor = self.floor();
        let mut units = 0;

        'outer: loop {
            let mut sand = self.start;

            loop {
                if sand.y == floor - 1 {
                    if with_floor {
                        self.map.insert(sand, Item::Sand);
                        units += 1;
                        break;
                    }

                    break 'outer;
                }

                if self.map.get(&sand.look_down()).is_none() {
                    sand.move_down();
                    continue;
                }

                if self.map.get(&sand.look_left_down()).is_none() {
                    sand.move_left_down();
                    continue;
                }

                if self.map.get(&sand.look_right_down()).is_none() {
                    sand.move_right_down();
                    continue;
                }

                if sand == self.start {
                    self.map.insert(sand, Item::Sand);
                    units += 1;
                    break 'outer;
                }

                self.map.insert(sand, Item::Sand);
                units += 1;
                break;
            }
        }

        units
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (mut min_x, mut min_y, mut max_x, mut max_y) =
            (usize::MAX, usize::MAX, usize::MIN, usize::MIN);
        let mut out = String::new();

        for coord in self.map.keys() {
            if coord.x < min_x {
                min_x = coord.x
            }

            if coord.y < min_y {
                min_y = coord.y
            }

            if coord.x > max_x {
                max_x = coord.x
            }

            if coord.y > max_y {
                max_y = coord.y
            }
        }

        for y in min_y..=max_y {
            for x in min_x - 10..=max_x + 10 {
                if let Some(item) = self.map.get(&Coordinate::new(x, y)) {
                    out.push_str(&format!("{}", item));
                } else {
                    out.push(' ');
                }
            }

            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Item {
    Block,
    Sand,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block => write!(f, "#"),
            Self::Sand => write!(f, "o"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    ///
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    ///
    pub fn bridge(&self, other: &Self) -> Vec<Self> {
        if self.x != other.x && self.y != other.y {
            unimplemented!();
        }

        let mut bridge = Vec::new();

        if self.x == other.x {
            for y in min(self.y, other.y)..=max(self.y, other.y) {
                bridge.push(Self::new(self.x, y));
            }
        } else {
            for x in min(self.x, other.x)..=max(self.x, other.x) {
                bridge.push(Self::new(x, self.y));
            }
        }

        bridge
    }

    ///
    pub fn look_down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    ///
    pub fn move_down(&mut self) {
        self.y += 1;
    }

    ///
    pub fn look_left_down(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }

    ///
    pub fn move_left_down(&mut self) {
        self.x -= 1;
        self.y += 1;
    }

    ///
    pub fn look_right_down(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }

    ///
    pub fn move_right_down(&mut self) {
        self.x += 1;
        self.y += 1;
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
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

        assert_eq!(answer.part1, 24);
        assert_eq!(answer.part2, 93);

        Ok(())
    }

    #[test]
    fn test_user_specific() -> Result<(), Box<dyn std::error::Error>> {
        let lines = lines!("input.txt")?;
        let answer = run(lines)?;

        assert_eq!(answer.part1, 885);
        assert_eq!(answer.part2, 28691);

        Ok(())
    }
}
