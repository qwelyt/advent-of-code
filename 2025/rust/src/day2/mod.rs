use crate::util::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 2 ==");
    let input = "src/day2/input.txt";
    //time(_part_a, input, "A");
    time(part_a2, input, "A");
    time(part_b, input, "B");
}

fn _part_a(input: &str) -> usize {
    let ranges = ranges(input);
    let invalids = ranges
        .iter()
        .map(|range| invalid_in_range(range))
        .collect::<Vec<Vec<usize>>>();

    invalids.iter().flatten().sum()
}
fn part_a2(input: &str) -> usize {
    let ranges = ranges(input);
    let invalids = ranges
        .iter()
        .map(|range| invalid_in_range2(range))
        .collect::<Vec<Vec<usize>>>();

    invalids.iter().flatten().sum()
}

fn ranges(input: &str) -> Vec<String> {
    File::open(input)
        .map(|file| {
            BufReader::new(file)
                .lines()
                .flatten()
                .collect::<Vec<String>>()
                .iter()
                .map(|s| {
                    s.split(",")
                        .map(|str| str.to_string())
                        .collect::<Vec<String>>()
                })
                .flatten()
                .collect::<Vec<String>>()
        })
        .unwrap()
}

fn invalid_in_range(range: &str) -> Vec<usize> {
    let (start_s, end_s) = range.split_once("-").unwrap();
    let end = end_s.parse::<usize>().unwrap();
    let mut vals = Vec::new();

    // Make sure we can split the string in half
    let mut start = start_s.parse::<usize>().unwrap();
    while (start.to_string().len() % 2) != 0 {
        start += 1
    }

    let start_s = start.to_string();
    let (curr_s, _) = start_s.split_at(start_s.len() / 2);

    let mut curr = curr_s.parse::<usize>().unwrap();
    let mut double = (curr.to_string() + curr.to_string().as_str())
        .parse::<usize>()
        .unwrap();
    while double <= end {
        if double >= start {
            vals.push(double);
        }

        curr += 1;
        double = (curr.to_string() + curr.to_string().as_str())
            .parse::<usize>()
            .unwrap();
    }

    vals
}
fn invalid_in_range2(range: &str) -> Vec<usize> {
    let (start_s, end_s) = range.split_once("-").unwrap();
    let start = start_s.parse::<usize>().unwrap();
    let end = end_s.parse::<usize>().unwrap();
    let mut vals = Vec::new();
    // Time to beat:
    //    43898358387ns
    for current in start..=end {
        let current_s = current.to_string();
        if current_s.len() % 2 != 0 {
            continue;
        }
        let sub = &current_s[0..current_s.len() / 2];
        let full_s = sub.repeat(2);
        let full = full_s.parse::<usize>().unwrap();
        if full == current && full >= start && full <= end {
            vals.push(full);
        }
    }
    vals
}

fn part_b(input: &str) -> usize {
    let ranges = ranges(input);
    let invalids = ranges
        .iter()
        .map(|range| invalid_in_range_multiple(range))
        .collect::<Vec<Vec<usize>>>();

    invalids.iter().flatten().sum()
}

fn invalid_in_range_multiple(range: &str) -> Vec<usize> {
    let (start_s, end_s) = range.split_once("-").unwrap();
    let end = end_s.parse::<usize>().unwrap();
    let start = start_s.parse::<usize>().unwrap();
    let mut vals = Vec::new();

    // First try to split in half and do as part A
    // Then try split in 3
    // Then 4
    // Continue until you are checking for just the same number always
    // OR, start with groups of 1, then 2, then 3 etc. Then we can count up until len/2

    for current in start..=end {
        let current_s = current.to_string();
        for group_size in 1..=current_s.len() / 2 {
            let sub = current_s[0..group_size].to_string();
            let repetitions = current_s.len() as f64 / group_size as f64;
            if repetitions.fract() != 0.0 {
                continue; // We can't split the string evenly, which means we can't have repeating patterns
            }
            let full_s = sub.repeat(repetitions as usize);
            let full = full_s.parse::<usize>().unwrap();
            if full == current && full >= start && full <= end {
                vals.push(full);
                break;
            }
        }
    }

    vals
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
        let input = "src/day2/input.txt";
        assert_eq!(16793817782, _part_a(input));
    }
    #[ignore]
    #[test]
    fn real_a2() {
        let input = "src/day2/input.txt";
        assert_eq!(16793817782, part_a2(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day2/input.txt";
        assert_eq!(27469417404, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day2/test-input.txt";
        let result = _part_a(input);
        assert_eq!(1227775554, result);
    }
    #[test]
    fn part_a2_test_input() {
        let input = "src/day2/test-input.txt";
        let result = part_a2(input);
        assert_eq!(1227775554, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day2/test-input.txt";
        let result = part_b(input);
        assert_eq!(4174379265, result);
    }

    #[test]
    fn invalid_in_range_test() {
        let input = "98-115";
        let result = invalid_in_range(input);
        assert_eq!(vec![99], result);

        let input = "11-22";
        let result = invalid_in_range(input);
        assert_eq!(vec![11, 22], result);

        let input = "1188511880-1188511890";
        let result = invalid_in_range(input);
        assert_eq!(vec![1188511885], result);

        let input = "100-999";
        let result = invalid_in_range(input);
        assert_eq!(Vec::<usize>::new(), result);

        let input = "100-999";
        let result = invalid_in_range(input);
        assert_eq!(Vec::<usize>::new(), result);
    }
    #[test]
    fn invalid_in_range2_test() {
        let input = "98-115";
        let result = invalid_in_range2(input);
        assert_eq!(vec![99], result);

        let input = "11-22";
        let result = invalid_in_range2(input);
        assert_eq!(vec![11, 22], result);

        let input = "1188511880-1188511890";
        let result = invalid_in_range2(input);
        assert_eq!(vec![1188511885], result);

        let input = "100-999";
        let result = invalid_in_range2(input);
        assert_eq!(Vec::<usize>::new(), result);

        let input = "100-999";
        let result = invalid_in_range2(input);
        assert_eq!(Vec::<usize>::new(), result);
    }

    #[test]
    fn invalid_in_range_multiple_test() {
        let input = "222220-222224";
        let result = invalid_in_range_multiple(input);
        assert_eq!(vec![222222], result);
    }
}
