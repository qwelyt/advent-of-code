use crate::util::time;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 7 ==");
    let input = "src/day7/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|line| compute(line.as_str(), false))
                .sum()
        })
        .unwrap()
}
fn part_b(input: &str) -> usize {
    File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|line| compute(line.as_str(), true))
                .sum()
        })
        .unwrap()
}

fn compute(line: &str, part2: bool) -> usize {
    let (sum_s, operands_s) = line.split_once(": ").unwrap();
    let sum = sum_s.parse::<usize>().unwrap();
    let operands = operands_s
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut sums = HashMap::new();
    sums.insert(0, vec![*operands.get(0).unwrap()]);
    for (i, operand) in operands.iter().enumerate().skip(1) {
        // println!("{} {}", i, operand);
        let mut n = Vec::new();
        for v in sums.get(&(i.checked_sub(1).unwrap())).unwrap().iter() {
            n.push(*v + operand);
            n.push(*v * operand);
            if part2 {
                let num = *v * 10usize.pow(operand.ilog10() + 1) + operand;
                n.push(num)
            }
        }
        sums.insert(i, n);
    }

    // println!("{:?}", line);
    // println!("{:?}  {:?}", sum_s, sum);
    // println!("{:?}  {:?}", operands_s, operands);
    // println!("{:?}", sums);
    if sums
        .get(&(operands.len().checked_sub(1).unwrap()))
        .unwrap()
        .contains(&sum)
    {
        return sum;
    }
    0
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
        let input = "src/day7/input.txt";
        assert_eq!(1298300076754, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day7/input.txt";
        assert_eq!(248427118972289, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day7/test-input.txt";
        let result = part_a(input);
        assert_eq!(3749, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day7/test-input.txt";
        let result = part_b(input);
        assert_eq!(11387, result);
    }
}
