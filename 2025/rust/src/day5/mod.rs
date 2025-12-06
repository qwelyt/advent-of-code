use crate::util::time;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 5 ==");
    let input = "src/day5/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> u64 {
    let (ranges, ids) = parse(input);

    let mut sum = 0;
    for &id in ids.iter() {
        for &range in ranges.iter() {
            if id >= range.0 && id <= range.1 {
                sum += 1;
                break;
            }
        }
    }

    sum
}

fn part_b(input: &str) -> u64 {
    let (mut ranges, _) = parse(input);
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = ranges[0];
    for i in 1..ranges.len() {
        let next = ranges[i];
        if current.1 >= next.0 {
            current.1 = max(current.1, next.1);
        } else {
            merged.push(current);
            current = next;
        }
    }
    merged.push(current);

    let mut sum = 0;
    for range in merged.iter() {
        sum += range.1 - range.0 + 1;
    }
    sum
}

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut numbers = Vec::new();

    let mut b = true;
    let open = File::open(input).expect("Could not read file");
    for line in BufReader::new(open).lines().flatten() {
        if line.is_empty() {
            b = false;
            continue;
        }
        if b {
            let split = line
                .split_once("-")
                .map(|(s, e)| (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap()))
                .unwrap();
            ranges.push(split);
        } else {
            numbers.push(line.parse::<u64>().unwrap());
        }
    }

    (ranges, numbers)
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
        let input = "src/day5/input.txt";
        assert_eq!(615, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day5/input.txt";
        assert_eq!(353716783056994, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day5/test-input.txt";
        let result = part_a(input);
        assert_eq!(3, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day5/test-input.txt";
        let result = part_b(input);
        assert_eq!(14, result);
    }
}
