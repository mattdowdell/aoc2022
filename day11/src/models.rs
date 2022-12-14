//! ...

use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;
use std::num::ParseIntError;

use lazy_static::lazy_static;
use regex::Regex;

///
#[derive(Clone, Debug)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    test_true: usize,
    test_false: usize,
    inspected: u64,
}

impl Monkey {
    ///
    pub fn turn(&mut self) -> Option<(u64, usize)> {
        if let Some(x) = self.items.pop_front() {
            self.inspected += 1;

            let x = self.operation.execute(x) / 3;
            if x % self.test == 0 {
                Some((x, self.test_true))
            } else {
                Some((x, self.test_false))
            }
        } else {
            None
        }
    }

    ///
    pub fn turn_lcm(&mut self, lcm: u64) -> Option<(u64, usize)> {
        if let Some(x) = self.items.pop_front() {
            self.inspected += 1;

            let x = self.operation.execute_mod(x, lcm); // / 3;
            if x % self.test == 0 {
                Some((x, self.test_true))
            } else {
                Some((x, self.test_false))
            }
        } else {
            None
        }
    }

    ///
    pub fn catch(&mut self, item: u64) {
        self.items.push_back(item);
    }

    ///
    pub fn inspected(&self) -> u64 {
        self.inspected
    }

    ///
    pub fn test(&self) -> u64 {
        self.test
    }
}

///
pub struct MonkeyBuilder {
    items: Option<VecDeque<u64>>,
    operation: Option<Operation>,
    test: Option<u64>,
    test_true: Option<usize>,
    test_false: Option<usize>,
}

impl MonkeyBuilder {
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    pub fn with_items(&mut self, items: VecDeque<u64>) {
        self.items = Some(items);
    }

    ///
    pub fn with_operation(&mut self, operation: Operation) {
        self.operation = Some(operation);
    }

    ///
    pub fn with_test(&mut self, test: u64) {
        self.test = Some(test);
    }

    ///
    pub fn with_test_true(&mut self, test_true: usize) {
        self.test_true = Some(test_true);
    }

    ///
    pub fn with_test_false(&mut self, test_false: usize) {
        self.test_false = Some(test_false);
    }
}

impl TryFrom<MonkeyBuilder> for Monkey {
    type Error = &'static str;

    fn try_from(builder: MonkeyBuilder) -> Result<Self, Self::Error> {
        Ok(Self {
            items: builder.items.ok_or("missing items")?,
            operation: builder.operation.ok_or("missing operation")?,
            test: builder.test.ok_or("missing test")?,
            test_true: builder.test_true.ok_or("missing test true")?,
            test_false: builder.test_false.ok_or("missing test false")?,
            inspected: 0,
        })
    }
}

impl Default for MonkeyBuilder {
    fn default() -> Self {
        Self {
            items: Option::default(),
            operation: Option::default(),
            test: Option::default(),
            test_true: Option::default(),
            test_false: Option::default(),
        }
    }
}

pub fn parse_line(line: &str, builder: &mut MonkeyBuilder) -> Result<(), Box<dyn Error>> {
    if line.strip_prefix("Monkey ").is_some() {
        return Ok(());
    }

    if let Some(items) = parse_items(line) {
        builder.with_items(items?);
        return Ok(());
    }

    if let Some(operation) = parse_operation(line) {
        builder.with_operation(operation?);
        return Ok(());
    }

    if let Some(test) = parse_test(line) {
        builder.with_test(test?);
        return Ok(());
    }

    if let Some(test_true) = parse_test_true(line) {
        builder.with_test_true(test_true?);
        return Ok(());
    }

    if let Some(test_false) = parse_test_false(line) {
        builder.with_test_false(test_false?);
        return Ok(());
    }

    Err(format!("failed to parse line: {}", line).into())
}

fn parse_items(value: &str) -> Option<Result<VecDeque<u64>, ParseIntError>> {
    value.strip_prefix("  Starting items: ").map(|x| {
        x.split(", ")
            .map(|x| x.parse::<u64>())
            .collect::<Result<VecDeque<u64>, _>>()
    })
}

fn parse_operation(value: &str) -> Option<Result<Operation, Box<dyn Error>>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"new = (\w+) ([*+]) (\w+)").expect("failed to parse regex");
    }

    value.strip_prefix("  Operation:").map(|x| {
        let captures = RE
            .captures_iter(x)
            .next()
            .ok_or("failed to match line")?;

        Ok(Operation::new(
            captures[1].parse()?,
            captures[2].parse()?,
            captures[3].parse()?,
        ))
    })
}

fn parse_test(value: &str) -> Option<Result<u64, ParseIntError>> {
    value.strip_prefix("  Test: divisible by ").map(|x| x.parse())
}

fn parse_test_true(value: &str) -> Option<Result<usize, ParseIntError>> {
    value.strip_prefix("    If true: throw to monkey ").map(|x| x.parse())
}

fn parse_test_false(value: &str) -> Option<Result<usize, ParseIntError>> {
    value.strip_prefix("    If false: throw to monkey ").map(|x| x.parse())
}

#[derive(Clone, Debug)]
pub struct Operation {
    left: Operand,
    operator: Operator,
    right: Operand,
}

impl Operation {
    ///
    pub fn new(left: Operand, operator: Operator, right: Operand) -> Self {
        Self { left, operator, right }
    }

    ///
    pub fn execute(&self, old: u64) -> u64 {
        match self.left {
            Operand::Old => {
                match self.right {
                    Operand::Old => self.operator.execute(old, old),
                    Operand::Int(x) => self.operator.execute(old, x),
                }
            }
            Operand::Int(x) => {
                match self.right {
                    Operand::Old => self.operator.execute(x, old),
                    Operand::Int(y) => self.operator.execute(y, x),
                }
            }
        }
    }

    ///
    pub fn execute_mod(&self, old: u64, m: u64) -> u64 {
        match self.left {
            Operand::Old => {
                match self.right {
                    Operand::Old => self.operator.execute_mod(old, old, m),
                    Operand::Int(x) => self.operator.execute_mod(old, x, m),
                }
            }
            Operand::Int(x) => {
                match self.right {
                    Operand::Old => self.operator.execute_mod(x, old, m),
                    Operand::Int(y) => self.operator.execute_mod(y, x, m),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Multiply,
}

impl Operator {
    ///
    pub fn execute(&self, left: u64, right: u64) -> u64 {
        match self {
            Self::Add => left + right,
            Self::Multiply => left * right,
        }
    }

    ///
    pub fn execute_mod(&self, left: u64, right: u64, m: u64) -> u64 {
        match self {
            Self::Add => left + right,
            Self::Multiply => ((left % m) * (right % m)) % m,
        }
    }


}

impl FromStr for Operator {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operand {
    Old,
    Int(u64),
}

impl FromStr for Operand {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "old" => Ok(Self::Old),
            _ => Ok(Self::Int(value.parse()?)),
        }
    }
}
