use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day2() {
    println!("== Day 2 ==");
    let input = "src/day2/input.txt";
    time(part_a, input, "A_1");
    time(part_a_2, input, "A_2");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let mred = 12;
    let mgreen = 13;
    let mblue = 14;
    let mut sum = 0;
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let split = line.split(": ").collect::<Vec<&str>>();
        let draws = split.last().unwrap().split("; ").collect::<Vec<&str>>();
        let mut valid = true;
        for draw in draws.iter() {
            let colours = draw.split(", ").collect::<Vec<&str>>();
            for colour in colours.iter() {
                if colour.ends_with("red") {
                    if colour.split_at(colour.len() - " red".len()).0.parse::<usize>().unwrap() > mred {
                        valid = false;
                    }
                } else if colour.ends_with("green") {
                    if colour.split_at(colour.len() - " green".len()).0.parse::<usize>().unwrap() > mgreen {
                        valid = false;
                    }
                } else if colour.ends_with("blue") {
                    if colour.split_at(colour.len() - " blue".len()).0.parse::<usize>().unwrap() > mblue {
                        valid = false;
                    }
                }
            }
        }
        if valid {
            let game_id = split.first().unwrap().split_at("Game ".len()).1.parse::<usize>().unwrap();
            sum += game_id;
        }
        // println!("{}, {}", game_id, valid);
    }
    sum
}

fn part_a_2(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    BufReader::new(open).lines().into_iter()
        .map(Result::unwrap)
        //.map(|l| l.replace(";",","))
        .map(|l| get_game_id(l.as_str()))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .sum()
}

fn get_game_id(line: &str) -> Option<usize> {
    let split = line.split(": ").collect::<Vec<&str>>();
    let over = split.last().iter()
        .flat_map(|l| l.split([',', ';']))
        .map(|p| {
            let sp = p.trim().split(" ").collect::<Vec<&str>>();
            match *sp.last().unwrap() {
                "red" => sp.first().map(|s| s.parse::<usize>().unwrap()).filter(|p| *p > 12),
                "green" => sp.first().map(|s| s.parse::<usize>().unwrap()).filter(|p| *p > 13),
                "blue" => sp.first().map(|s| s.parse::<usize>().unwrap()).filter(|p| *p > 14),
                &_ => None
            }
        })
        .filter(Option::is_some)
        .collect::<Vec<Option<usize>>>();

    if over.is_empty() {
        return split.first()
            .map(|s| s.split(" ").last())
            .flatten()
            .map(|s| s.parse::<usize>().unwrap())
    }
    None
}

fn part_b(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let mut sum = 0;
    for line in BufReader::new(open).lines() {
        let mut mred = 0;
        let mut mgreen = 0;
        let mut mblue = 0;

        let line = line.unwrap();
        let split = line.split(": ").collect::<Vec<&str>>();
        let draws = split.last().unwrap().split("; ").collect::<Vec<&str>>();
        for draw in draws.iter() {
            let colours = draw.split(", ").collect::<Vec<&str>>();
            for colour in colours.iter() {
                if colour.ends_with("red") {
                    let found = colour.split_at(colour.len() - " red".len()).0.parse::<usize>().unwrap();
                    mred = max(mred, found);
                } else if colour.ends_with("green") {
                    let found = colour.split_at(colour.len() - " green".len()).0.parse::<usize>().unwrap();
                    mgreen = max(mgreen, found);
                } else if colour.ends_with("blue") {
                    let found = colour.split_at(colour.len() - " blue".len()).0.parse::<usize>().unwrap();
                    mblue = max(mblue, found);
                }
            }
        }
        let power = mred * mgreen * mblue;
        // println!("{}: red {}, green {}, blue {} == {}", game_id, mred, mgreen, mblue, power);
        sum += power;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "src/day2/test-input.txt";
    const REAL_INPUT: &str = "src/day2/input.txt";

    #[test]
    fn part_a_test_input() {
        let result = part_a(TEST_INPUT);
        assert_eq!(8, result);
        let result = part_a_2(TEST_INPUT);
        assert_eq!(8, result);
    }

    #[test]
    fn real_a() {
        let result = part_a(REAL_INPUT);
        assert_eq!(2006, result)
    }

    #[test]
    fn part_b_test_input() {
        let result = part_b(TEST_INPUT);
        assert_eq!(2286, result)
    }

    #[test]
    fn real_b() {
        let result = part_b(REAL_INPUT);
        assert_eq!(84911, result)
    }
}
