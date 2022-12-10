//! Models to represent rope movement.

use std::collections::HashSet;
use std::fmt;

/// A representation with a number of knots.
pub struct Rope {
    knots: Vec<(i32, i32)>,
    path: HashSet<(i32, i32)>,
}

impl Rope {
    /// Create a new instance of `Rope` with the given number of knots.
    pub fn new(count: usize) -> Self {
        let mut path = HashSet::new();
        path.insert((0, 0));

        let mut knots = Vec::new();

        for _ in 0..count {
            knots.push((0, 0));
        }

        Self { knots, path }
    }

    /// Apply a move to a rope.
    pub fn apply(&mut self, mov: Move) {
        let (count, (x_diff, y_diff)) = match mov {
            Move::Right(count) => (count, (1, 0)),
            Move::Left(count) => (count, (-1, 0)),
            Move::Up(count) => (count, (0, 1)),
            Move::Down(count) => (count, (0, -1)),
        };

        for _ in 0..count {
            let (mut x, mut y) = self.knots[0];

            x += x_diff;
            y += y_diff;

            self.knots[0] = (x, y);

            for i in 1..self.knots.len() {
                self.move_tail(i);
            }

            if let Some(tail) = self.knots.last() {
                self.path.insert(*tail);
            }
        }
    }

    /// Move a knot to be sufficiently close to its predecessor.
    fn move_tail(&mut self, index: usize) {
        let (mut tail_x, mut tail_y) = self.knots[index];
        let (head_x, head_y) = self.knots[index - 1];

        if (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
            tail_x += (head_x - tail_x).signum();
            tail_y += (head_y - tail_y).signum();
        }

        self.knots[index] = (tail_x, tail_y);
    }

    /// Get the number of locations the last knot in the rope has been in.
    pub fn tail_locations(&self) -> usize {
        self.path.len()
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut locations = self.path.iter().collect::<Vec<_>>();
        locations.sort();

        let min_x = locations
            .iter()
            .map(|(x, _)| x)
            .min()
            .expect("no locations present");
        let max_x = locations
            .iter()
            .map(|(x, _)| x)
            .max()
            .expect("no locations present");
        let min_y = locations
            .iter()
            .map(|(_, y)| y)
            .min()
            .expect("no locations present");
        let max_y = locations
            .iter()
            .map(|(_, y)| y)
            .max()
            .expect("no locations present");

        let mut data = String::new();

        for y in (*min_y - 2..*max_y + 2).rev() {
            for x in *min_x - 2..*max_x + 2 {
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

/// The direction and number of steps a rope can be moved in.
#[derive(Copy, Clone, Debug)]
pub enum Move {
    Right(u32),
    Left(u32),
    Up(u32),
    Down(u32),
}

impl TryFrom<&str> for Move {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, count) = value.split_once(' ').ok_or("unexpected format for move")?;

        match direction {
            "R" => Ok(Self::Right(
                count.parse().map_err(|_| "failed to parse count")?,
            )),
            "L" => Ok(Self::Left(
                count.parse().map_err(|_| "failed to parse count")?,
            )),
            "U" => Ok(Self::Up(
                count.parse().map_err(|_| "failed to parse count")?,
            )),
            "D" => Ok(Self::Down(
                count.parse().map_err(|_| "failed to parse count")?,
            )),
            _ => todo!(),
        }
    }
}
