use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut totals = BinaryHeap::new();
    let mut current = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            totals.push(current);
            current = 0;
        } else {
            let value: u32 = line.parse().unwrap();
            current += value;
        }
    }

    println!(
        "{}",
        totals.pop().unwrap() + totals.pop().unwrap() + totals.pop().unwrap()
    );
}
