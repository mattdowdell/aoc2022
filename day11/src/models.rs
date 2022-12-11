//! ...

use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

///
#[derive(Clone, Debug)]
pub struct Monkey {
    starting_items: VecDeque<u128>,
    operation: Operation,
    test: u128,
    test_true: usize,
    test_false: usize,
    inspected: u128,
}

impl Monkey {
    ///
    pub fn turn(&mut self) -> Option<(u128, usize)> {
        if let Some(x) = self.starting_items.pop_front() {
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
    pub fn turn_lcm(&mut self, lcm: u128) -> Option<(u128, usize)> {
        if let Some(x) = self.starting_items.pop_front() {
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
    pub fn catch(&mut self, item: u128) {
        self.starting_items.push_back(item);
    }

    ///
    pub fn inspected(&self) -> u128 {
        self.inspected
    }

    ///
    pub fn test(&self) -> u128 {
        self.test
    }
}

///
pub struct MonkeyBuilder {
    starting_items: Option<VecDeque<u128>>,
    operation: Option<Operation>,
    test: Option<u128>,
    test_true: Option<usize>,
    test_false: Option<usize>,
}

impl MonkeyBuilder {
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    pub fn with_starting_items(&mut self, starting_items: VecDeque<u128>) {
        self.starting_items = Some(starting_items);
    }

    ///
    pub fn with_operation(&mut self, operation: Operation) {
        self.operation = Some(operation);
    }

    ///
    pub fn with_test(&mut self, test: u128) {
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
            starting_items: builder.starting_items.unwrap(),
            operation: builder.operation.unwrap(),
            test: builder.test.unwrap(),
            test_true: builder.test_true.unwrap(),
            test_false: builder.test_false.unwrap(),
            inspected: 0,
        })
    }
}

impl Default for MonkeyBuilder {
    fn default() -> Self {
        Self {
            starting_items: Option::default(),
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

    if let Some(starting_items) = parse_starting_items(line) {
        builder.with_starting_items(starting_items);
        return Ok(());
    }

    if let Some(operation) = parse_operation(line) {
        builder.with_operation(operation);
        return Ok(());
    }

    if let Some(test) = parse_test(line) {
        builder.with_test(test);
        return Ok(());
    }

    if let Some(test_true) = parse_test_true(line) {
        builder.with_test_true(test_true);
        return Ok(());
    }

    if let Some(test_false) = parse_test_false(line) {
        builder.with_test_false(test_false);
        return Ok(());
    }

    Err(format!("failed to parse line: {}", line).into())
}

fn parse_starting_items(value: &str) -> Option<VecDeque<u128>> {
    value.strip_prefix("  Starting items: ").map(|x| {
        x.split(", ")
            .map(|x| x.parse::<u128>())
            .collect::<Result<VecDeque<u128>, _>>()
            .unwrap()
    })
}

fn parse_operation(value: &str) -> Option<Operation> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"new = (\w+) ([*+]) (\w+)").unwrap();
    }

    value.strip_prefix("  Operation:").map(|x| {
        let captures = RE
            .captures_iter(x)
            .next()
            .unwrap();

        Operation::new(
            captures[1].parse().unwrap(),
            captures[2].parse().unwrap(),
            captures[3].parse().unwrap(),
        )
    })
}

fn parse_test(value: &str) -> Option<u128> {
    value.strip_prefix("  Test: divisible by ").map(|x| x.parse().unwrap())
}

fn parse_test_true(value: &str) -> Option<usize> {
    value.strip_prefix("    If true: throw to monkey ").map(|x| x.parse().unwrap())
}

fn parse_test_false(value: &str) -> Option<usize> {
    value.strip_prefix("    If false: throw to monkey ").map(|x| x.parse().unwrap())
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
    pub fn execute(&self, old: u128) -> u128 {
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
    pub fn execute_mod(&self, old: u128, m: u128) -> u128 {
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
    pub fn execute(&self, left: u128, right: u128) -> u128 {
        match self {
            Self::Add => left + right,
            Self::Multiply => left * right,
        }
    }

    ///
    pub fn execute_mod(&self, left: u128, right: u128, m: u128) -> u128 {
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
    Int(u128),
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "old" => Ok(Self::Old),
            _ => Ok(Self::Int(value.parse().unwrap())),
        }
    }
}
