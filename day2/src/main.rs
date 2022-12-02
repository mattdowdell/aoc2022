use std::io::{self, BufRead};

const OUTCOME_WIN: u32 = 6;
const OUTCOME_DRAW: u32 = 3;
const OUTCOME_LOSE: u32 = 0;

#[derive(Clone, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn outcome(&self, other: Self) -> u32 {
        match (self, other) {
            (Self::Rock, Self::Paper) => OUTCOME_LOSE,
            (Self::Rock, Self::Scissors) => OUTCOME_WIN,
            (Self::Rock, Self::Rock) => OUTCOME_DRAW,
            (Self::Paper, Self::Paper) => OUTCOME_DRAW,
            (Self::Paper, Self::Scissors) => OUTCOME_LOSE,
            (Self::Paper, Self::Rock) => OUTCOME_WIN,
            (Self::Scissors, Self::Paper) => OUTCOME_WIN,
            (Self::Scissors, Self::Scissors) => OUTCOME_DRAW,
            (Self::Scissors, Self::Rock) => OUTCOME_LOSE,
        }
    }
}

impl From<Shape> for u32 {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl TryFrom<&str> for Shape {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => todo!(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut total = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue
        }

        let (first, last) = line.split_at(1);

        let (opponent, yours) = (Shape::try_from(first).unwrap(), Shape::try_from(last).unwrap());
        let score =  yours.outcome(opponent) + u32::from(yours);

        total += score;
    }

    println!("{}", total);
}
