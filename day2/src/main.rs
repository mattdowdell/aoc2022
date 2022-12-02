use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn outcome(&self, other: Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Paper) => Outcome::Lose,
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Rock, Self::Rock) => Outcome::Draw,
            (Self::Paper, Self::Paper) => Outcome::Draw,
            (Self::Paper, Self::Scissors) => Outcome::Lose,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Scissors, Self::Scissors) => Outcome::Draw,
            (Self::Scissors, Self::Rock) => Outcome::Lose,
        }
    }

    pub fn match_outcome(&self, outcome: Outcome) -> Self {
        match (self, outcome) {
            (Self::Rock, Outcome::Win) => Self::Paper,
            (Self::Rock, Outcome::Lose) => Self::Scissors,
            (Self::Rock, Outcome::Draw) => Self::Rock,
            (Self::Paper, Outcome::Win) => Self::Scissors,
            (Self::Paper, Outcome::Lose) => Self::Rock,
            (Self::Paper, Outcome::Draw) => Self::Paper,
            (Self::Scissors, Outcome::Win) => Self::Rock,
            (Self::Scissors, Outcome::Lose) => Self::Paper,
            (Self::Scissors, Outcome::Draw) => Self::Scissors,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
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
            _ => Err("Unsupported value"),
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

impl TryFrom<&str> for Outcome {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Unsupported value"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut total1 = 0;
    let mut total2 = 0;

    for line in stdin.lock().lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let (first, last) = line.split_at(1);
        let shape1 = Shape::try_from(first)?;
        let shape2 = Shape::try_from(last)?;
        let outcome = Outcome::try_from(last)?;

        total1 += shape2.outcome(shape1).score() + shape2.score();
        total2 += outcome.score() + shape1.match_outcome(outcome).score();
    }

    println!("Solution 1: {}", total1);
    println!("Solution 2: {}", total2);

    Ok(())
}
