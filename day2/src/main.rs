use std::io::{self, BufRead};

#[derive(Clone, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_outcome(&self, outcome: Outcome) -> Self {
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

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value.trim() {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => todo!(),
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
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value.trim() {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
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
        let shape = Shape::from(first);
        let outcome = Outcome::from(last);

        let score = outcome.score() + shape.from_outcome(outcome).score();
        total += score;
    }

    println!("{}", total);
}
