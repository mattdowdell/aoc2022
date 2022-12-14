//! ...

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::io::{self, BufRead, Lines};

///
#[derive(Clone, Debug)]
pub struct Graph {
    end: Coordinate,
    vertices: HashMap<Coordinate, Vertex>,
}

impl Graph {
    ///
    pub fn from<T>(lines: Lines<T>) -> Result<Self, io::Error>
    where
        T: BufRead,
    {
        let mut vertices = HashMap::new();

        for (y, line) in lines.enumerate() {
            let line = line?;

            if line.is_empty() {
                continue;
            }

            vertices.extend(
                line.chars()
                    .map(Vertex::from)
                    .enumerate()
                    .map(|(x, vertex)| (Coordinate::new(x, y), vertex)),
            );
        }

        let mut end = Coordinate::default();

        for (coord, vertex) in vertices.iter() {
            if vertex.is_end {
                end = *coord;
                break;
            }
        }

        Ok(Self { end, vertices })
    }

    ///
    pub fn reverse_flood_fill(&self, target_start: bool) -> Option<u32> {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back(Step {
            coordinate: self.end,
            ..Default::default()
        });

        while let Some(step) = queue.pop_front() {
            if seen.contains(&step.coordinate) {
                continue;
            }

            seen.insert(step.coordinate);

            let current = self.vertices.get(&step.coordinate).unwrap();

            if (target_start && current.is_start) || (!target_start && current.value == 1) {
                return Some(step.distance);
            }

            for adjacent in step.coordinate.adjacents().into_iter() {
                if let Some(next) = self.vertices.get(&adjacent) {
                    if current.accessible_from(next) {
                        queue.push_back(Step {
                            coordinate: adjacent,
                            distance: step.distance + 1,
                        })
                    }
                }
            }
        }

        None
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        let mut y = 0;

        loop {
            let mut x = 0;

            if self.vertices.get(&Coordinate::new(x, y)).is_none() {
                break;
            }

            while let Some(vertex) = self.vertices.get(&Coordinate::new(x, y)) {
                out.push(char::from(vertex));
                x += 1;
            }

            out.push('\n');
            y += 1;
        }

        write!(f, "{}", out)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub value: u32,
    pub is_start: bool,
    pub is_end: bool,
}

impl Vertex {
    ///
    pub fn accessible_from(&self, other: &Self) -> bool {
        self.value <= other.value + 1
    }
}

impl From<char> for Vertex {
    fn from(c: char) -> Self {
        let value = match c {
            'S' => 1,
            'E' => 26,
            _ => c as u32 - 'a' as u32 + 1,
        };

        Self {
            value,
            is_start: c == 'S',
            is_end: c == 'E',
        }
    }
}

impl From<&Vertex> for char {
    fn from(v: &Vertex) -> Self {
        if v.is_start {
            'S'
        } else if v.is_end {
            'E'
        } else {
            Self::from_u32(v.value - 1 + 'a' as u32).unwrap()
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
    pub fn adjacents(&self) -> Vec<Self> {
        let mut adjacents = Vec::new();

        adjacents.push(Self::new(self.x, self.y + 1));

        if self.y > 0 {
            adjacents.push(Self::new(self.x, self.y - 1));
        }

        adjacents.push(Self::new(self.x + 1, self.y));

        if self.x > 0 {
            adjacents.push(Self::new(self.x - 1, self.y));
        }

        adjacents
    }
}

#[derive(Debug, Default)]
pub struct Step {
    coordinate: Coordinate,
    distance: u32,
}
