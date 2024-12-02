use crate::util::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 2 ==");
    let input = "src/day2/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|line| numbers(line.as_str()))
                .filter(|line| is_valid(line))
                .count()
        })
        .unwrap()
}

fn part_b(input: &str) -> usize {
    File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|line| numbers(line.as_str()))
                .filter(|line| is_valid2(line))
                .count()
        })
        .unwrap()
}

fn numbers(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|w| w.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn is_valid(vec: &Vec<i32>) -> bool {
    if vec[0] == vec[1] {
        return false;
    } else if vec[0] < vec[1] {
        // Increasing
        for pair in vec.windows(2) {
            // println!("{} {}", pair[0], pair[1]);
            return if pair[0] == pair[1] {
                false
            } else if pair[0] > pair[1] {
                false
            } else if pair[1] - pair[0] < 1 {
                false
            } else if pair[1] - pair[0] > 3 {
                false
            } else {
                continue;
            };
        }
    } else if vec[0] > vec[1] { // Decreasing
        for pair in vec.windows(2) {
            // println!("{} {}", pair[0], pair[1]);
            return if pair[0] == pair[1] {
                false
            } else if pair[0] < pair[1] {
                false
            } else if pair[0] - pair[1] < 1 {
                false
            } else if pair[0] - pair[1] > 3 {
                false
            } else {
                continue;
            };
        }
    }

    true
}

fn is_valid2(vec: &Vec<i32>) -> bool {
    if is_valid(vec) {
        return true;
    }
    // This is so stupid...
    for (idx,_) in vec.iter().enumerate() {
        let mut vec1 = vec.clone();
        vec1.remove(idx);
        if is_valid(&vec1) {
            return true;
        }
    }
    false
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
        assert_eq!(299, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day2/input.txt";
        assert_eq!(364, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day2/test-input.txt";
        let result = part_a(input);
        assert_eq!(2, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day2/test-input.txt";
        let result = part_b(input);
        assert_eq!(4, result);
    }
}
