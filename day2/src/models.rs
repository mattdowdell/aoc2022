//! Models to encapsulate the logic of the Rock/Paper/Scissor rules.

/// The shape used by a player in the game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    /// Get the outcome for the given shape.
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

    /// Provide a shape to match the given outcome.
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

    /// Get the score associated with the shape.
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

/// The outcome of a game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    /// The score associated with the outcome.
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
