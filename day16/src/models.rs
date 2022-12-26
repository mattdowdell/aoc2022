//! ...

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::{BufRead, Lines};

///
#[derive(Debug)]
pub struct Volcano {
    current: String,
    // TODO, use `(char, char)` instead of String
    valves: HashMap<String, Valve>,
}

///
impl Volcano {
    ///
    pub fn from<T: BufRead>(lines: Lines<T>) -> Result<Self, Box<dyn Error>> {
        let mut valves = HashMap::new();

        for line in lines {
            let line = line?;

            if line.is_empty() {
                continue;
            }

            let (name, valve) = Valve::parse(line.as_str())?;
            valves.insert(name, valve);
        }

        Ok(Self {
            current: "AA".to_string(),
            valves,
        })
    }

    ///
    pub fn run(&mut self, remainder: u32) -> u32 {
        let pressure = self.valves.values().map(|v| v.pressure()).sum();

        let next = self.next_move(&self.current, remainder);

        if next != self.current {
            println!("{}: Move from {} to {}", 30 - remainder, self.current, next);
            self.current = next;
        } else {
            let valve = self.valves.get_mut(&next).unwrap();

            if !valve.on {
                println!("{}: Open {}", 30 - remainder, next);
                valve.on = true;
            }
        }

        println!("{}: Pressure: {}", 30 - remainder, pressure);
        pressure
    }

    ///
    pub fn next_move(&self, from: &str, remainder: u32) -> String {
        let mut queue = VecDeque::new();
        let mut seen = HashMap::new();

        queue.push_back((from, (0, vec![])));

        while let Some((name, (distance, path))) = queue.pop_front() {
            if seen.get(name).is_some() {
                continue;
            }

            let valve = self.valves.get(name).unwrap();
            seen.insert(name, (valve.cost(distance, remainder), path.clone()));

            for next in valve.tunnels.iter() {
                let mut path = path.clone();
                path.push(name.clone());

                queue.push_back((next, (distance + 1, path)));
            }
        }

        let mut next = String::new();
        let mut next_cost = 0;
        let mut next_steps = 0;

        for (name, (cost, steps)) in seen.iter() {
            if next.is_empty() {
                next = name.to_string();
                next_cost = *cost;
                next_steps = steps.len();
                continue;
            }

            if next_cost < *cost {
                next = name.to_string();
                next_cost = *cost;
                next_steps = steps.len();
                continue;
            }

            if next_cost == *cost && next_steps > steps.len() {
                next = name.to_string();
                next_cost = *cost;
                next_steps = steps.len();
                continue;
            }
        }

        let (_, steps) = seen.get(next.as_str()).unwrap();
        return steps.get(1).map(|s| s.to_string()).unwrap_or(next);
    }
}

///
#[derive(Debug)]
pub struct Valve {
    pub flow_rate: u32,
    pub tunnels: Vec<String>,
    pub on: bool,
}

impl Valve {
    ///
    pub fn parse(input: &str) -> Result<(String, Self), Box<dyn Error>> {
        let input = input
            .strip_prefix("Valve ")
            .ok_or("missing 'Valve ' prefix")?;
        let (name, tail) = input
            .split_once(' ')
            .ok_or("failed to identify valve name")?;

        let input = tail
            .strip_prefix("has flow rate=")
            .ok_or("missing 'has flow rate='")?;
        let (flow_rate, tail) = input
            .split_once(';')
            .ok_or("failed to identify flow rate")?;

        let input = if let Some(input) = tail.strip_prefix(" tunnel leads to valve ") {
            input
        } else {
            tail.strip_prefix(" tunnels lead to valves")
                .ok_or("failed to extract valves")?
        };

        let tunnels = input.split(", ").map(|s| s.trim().to_string()).collect();

        Ok((
            name.trim().to_string(),
            Self {
                flow_rate: flow_rate.parse()?,
                tunnels,
                on: false,
            },
        ))
    }

    ///
    pub fn cost(&self, distance: u32, remainder: u32) -> u32 {
        if self.flow_rate == 0 || self.on {
            return 0;
        }

        // TODO: this needs to be better - travelling salesman problem?
        if self.flow_rate > distance {
            self.flow_rate - distance
        } else {
            0
        }
    }

    ///
    pub fn pressure(&self) -> u32 {
        if self.on {
            self.flow_rate
        } else {
            0
        }
    }
}
