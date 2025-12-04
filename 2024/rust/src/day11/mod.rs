use crate::util::time;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 11 ==");
    let input = "src/day11/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let stones = parse_stones(input);
    blink(&stones, 25)
}
fn part_b(input: &str) -> usize {
    let stones = parse_stones(input);
    let mut pluto = Pluto {
        cache: HashMap::new(),
    };
    stones.iter().map(|stone| pluto.blink2(*stone, 75)).sum()
}

fn parse_stones(input: &str) -> Vec<usize> {
    File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .last()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .unwrap()
}

fn blink(initial_stones: &Vec<usize>, times: i32) -> usize {
    let mut stones = initial_stones.clone();
    // println!("Initial stones: {:?}", initial_stones);
    let mut blinked_stones = Vec::new();
    for _ in 0..times {
        for stone in stones.iter() {
            let string = stone.to_string();
            if *stone == 0 {
                blinked_stones.push(1);
            } else if string.len() % 2 == 0 {
                let x = string.split_at(string.len() / 2);
                blinked_stones.push(x.0.parse::<usize>().unwrap());
                blinked_stones.push(x.1.parse::<usize>().unwrap());
            } else {
                blinked_stones.push(stone * 2024)
            }
        }
        stones = blinked_stones.clone();
        blinked_stones.clear();
        // println!("{:?}", stones);
    }
    stones.len()
}

struct Pluto {
    cache: HashMap<(usize, i32), usize>,
}
impl Pluto {
    fn blink2(&mut self, stone: usize, times: i32) -> usize {
        if self.cache.contains_key(&(stone, times)) {
            return *self.cache.get(&(stone, times)).unwrap();
        }
        let string = stone.to_string();
        let ret = if times == 0 {
            1
        } else if stone == 0 {
            self.blink2(1, times - 1)
        } else if string.len() % 2 == 0 {
            let x = string.split_at(string.len() / 2);
            self.blink2(x.0.parse::<usize>().unwrap(), times - 1)
                + self.blink2(x.1.parse::<usize>().unwrap(), times - 1)
        } else {
            self.blink2(stone * 2024, times - 1)
        };
        self.cache.insert((stone, times), ret);
        ret
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
        let input = "src/day11/input.txt";
        assert_eq!(235850, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day11/input.txt";
        assert_eq!(279903140844645, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day11/test-input.txt";
        let result = part_a(input);
        assert_eq!(55312, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day11/test-input.txt";
        let result = part_b(input);
        assert_eq!(0, result);
    }
}
