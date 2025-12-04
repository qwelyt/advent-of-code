use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::util::time;

pub fn solve() {
    println!("== Day 1 ==");
    let input = "src/day1/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> i32 {
    let mut pointer = 50;
    let mut zeros = 0;

    let open = File::open(input).expect("Could not read file");
    for line in BufReader::new(open).lines().flatten() {
        let replaced = line.replace("R", "+").replace("L", "-");
        let i = replaced.parse::<i32>().unwrap();
        pointer += i;
        pointer %= 100;
        if pointer < 0 {
            pointer += 100;
        }
        if pointer == 0 {
            zeros += 1;
        }
    }

    zeros
}


fn part_b(input: &str) -> i32 {
    let mut pointer = 50;
    let mut zeros = 0;

    let open = File::open(input).expect("Could not read file");
    for line in BufReader::new(open).lines().flatten() {
        let (dir, ns) = line.split_at(1);
        let n = ns.parse::<i32>().unwrap();
        for _ in 0..n {
            if dir == "L" {
                pointer -= 1;
            } else {
                pointer += 1;
            }
            pointer %= 100;
            if pointer < 0 {
                pointer += 100;
            }
            if pointer == 0 {
                zeros += 1;
            }
        }
    }

    zeros
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
        let input = "src/day1/input.txt";
        assert_eq!(1172, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day1/input.txt";
        assert_eq!(6932, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day1/test-input.txt";
        let result = part_a(input);
        assert_eq!(3, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day1/test-input.txt";
        let result = part_b(input);
        assert_eq!(6, result);
    }
}