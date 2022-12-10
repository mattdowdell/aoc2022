//! Models to model operations executed by a CPU.

use std::collections::VecDeque;
use std::str::FromStr;

/// A representation of a CPU with a single register.
pub struct Cpu {
    operations: VecDeque<Operation>,
}

impl Cpu {
    /// Create a new `Cpu` instance.
    pub fn new() -> Self {
        Self {
            operations: VecDeque::new(),
        }
    }

    /// Add an operation to be executed by the CPU.
    pub fn add(&mut self, op: Operation) {
        self.operations.push_back(op);
    }

    /// Execute all operations, returning the values of the register after each cycle.
    pub fn execute(&self) -> Vec<i32> {
        let mut x = 1;
        let mut results = Vec::new();

        for op in self.operations.iter() {
            for i in (0..op.cycles()).rev() {
                results.push(x);

                if i == 0 {
                    x = op.execute(x);
                }
            }
        }

        results
    }
}

/// A representation of an operation that a CPU can execute.
#[derive(Debug)]
pub enum Operation {
    Noop,
    Addx(i32),
}

impl Operation {
    /// The number of cycles the operation takes.
    pub fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }

    /// execute the operation on the given value.
    pub fn execute(&self, input: i32) -> i32 {
        match self {
            Self::Noop => input,
            Self::Addx(change) => input + change,
        }
    }
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Self::Noop);
        }

        if let Some(x) = s.strip_prefix("addx ") {
            let value = x.parse().map_err(|_| "invaid number for addx")?;
            return Ok(Self::Addx(value));
        }

        Err("unrecognised operation")
    }
}
