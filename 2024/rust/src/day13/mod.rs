use crate::util::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 13 ==");
    let input = "src/day13/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> u32 {
    let lines = File::open(input)
        .map(|f| BufReader::new(f).lines().flatten().collect::<Vec<String>>())
        .unwrap();
    let mut machines = Vec::new();
    let mut block = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            machines.push(Machine::new(block));
            block = Vec::new();
            continue;
        }
        block.push(line);
    }
    if !block.is_empty() {
        machines.push(Machine::new(block));
    }
    machines
        .iter()
        .map(Machine::cheapest_win)
        .map(|min| min.unwrap_or(0))
        .sum()
}

fn part_b(input: &str) -> i32 {
    0
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32),
}
impl Machine {
    fn parse(line: &str, delim: char) -> (u32, u32) {
        line.split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .map(|(x, y)| {
                (
                    x.split_once(delim).unwrap().1.parse().unwrap(),
                    y.split_once(delim).unwrap().1.parse().unwrap(),
                )
            })
            .unwrap()
    }
    fn new(block: Vec<&String>) -> Self {
        let a = Machine::parse(block[0], '+');
        let b = Machine::parse(block[1], '+');
        let prize = Machine::parse(block[2], '=');
        Self { a, b, prize }
    }

    fn cheapest_win(&self) -> Option<u32> {
        let mut min = None;
        for press_a in 0..=100 {
            for press_b in 0..=100 {
                let x = press_a * self.a.0 + press_b * self.b.0;
                let y = press_a * self.a.1 + press_b * self.b.1;
                let cost = press_a * 3 + press_b;
                if x == self.prize.0 && y == self.prize.1 {
                    if min.is_none() || min.unwrap() > cost {
                        min = Some(cost);
                    }
                }
            }
        }
        // println!("{:?} ::: {:?}", self,min);
        min
    }
    fn cheapest_win_13_0(&self) -> Some(usize) {
        let prize = (
            self.prize.0 as usize + 10_000_000_000_000,
            self.prize.1 as usize + 10_000_000_000_000,
        );
        let press_a = (prize.0 * self.b.1 as usize - prize.1 * self.b.0 as usize)
            / (self.a.0 as usize * self.b.1 as usize - self.a.1 as usize * self.b.0 as usize);
        // let press_b = prize.0 - self.a.0  * press_a  as usize self.b.o
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn run_day() {
        solve();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day13/input.txt";
        assert_eq!(36250, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day13/input.txt";
        assert_eq!(0, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day13/test-input.txt";
        let result = part_a(input);
        assert_eq!(480, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day13/test-input.txt";
        let result = part_b(input);
        assert_eq!(0, result);
    }
}
