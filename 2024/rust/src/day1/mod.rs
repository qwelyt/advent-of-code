use crate::util::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 1 ==");
    let input = "src/day1/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> u32 {
    let lists = get_lists(input);
    let mut left = lists.0;
    left.sort();
    let mut right = lists.1;
    right.sort();

    let mut diff = Vec::new();
    for (idx, l) in left.iter().enumerate() {
        let r = *right.get(idx).unwrap() as i32;
        let d = r - (*l as i32);
        diff.push(d.abs() as u32);
    }

    diff.iter().sum()
}

fn part_b(input: &str) -> usize {
    let lists = get_lists(input);

    let mut score = 0;
    for l in lists.0 {
        let multiplier = lists.1.iter().filter(|&x| *x == l).count();
        score += l as usize * multiplier
    }
    score
}

fn get_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let map = File::open(input)
        .map(|file| {
            BufReader::new(file)
                .lines()
                .flatten()
                .collect::<Vec<String>>()
        })
        .unwrap();
    for l in map {
        let whitespace = l
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        left.push(*whitespace.get(0).unwrap());
        right.push(*whitespace.get(1).unwrap());
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        solve();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day1/input.txt";
        assert_eq!(1830467, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day1/input.txt";
        assert_eq!(26674158, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day1/test-input.txt";
        let result = part_a(input);
        assert_eq!(11, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day1/test-input.txt";
        let result = part_b(input);
        assert_eq!(31, result);
    }
}
