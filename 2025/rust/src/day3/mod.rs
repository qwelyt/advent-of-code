use crate::util::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 3 ==");
    let input = "src/day3/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> i32 {
    let mut joltages = Vec::new();
    let open = File::open(input).expect("Could not read file");
    for line in BufReader::new(open).lines().flatten() {
        let battery_bank = battery_bank(line.as_str());

        // get first number
        let mut max = (0, 0);
        for (i, v) in battery_bank.iter().enumerate() {
            if i == battery_bank.len() - 1 {
                continue;
            }
            if *v > max.0 {
                max = (*v, i);
            }
        }

        // Get second number
        let mut second = (0, 0);
        let rest = battery_bank.split_at(max.1 + 1).1;
        for (i, v) in rest.iter().enumerate() {
            if *v > second.0 {
                second = (*v, i);
            }
        }

        let jotage = (max.0.to_string() + second.0.to_string().as_str())
            .parse::<i32>()
            .unwrap();
        joltages.push(jotage);
    }
    joltages.iter().sum()
}

fn part_b(input: &str) -> usize {
    let mut joltages = Vec::new();
    let open = File::open(input).expect("Could not read file");
    for line in BufReader::new(open).lines().flatten() {
        let battery_bank = battery_bank(line.as_str());
        joltages.push(big_joltages(&battery_bank))
    }
    joltages.iter().sum()
}

fn battery_bank(line: &str) -> Vec<usize> {
    line.split("")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&s| *s != "")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn big_joltages(battery_bank: &Vec<usize>) -> usize {
    let mut batteries = Vec::new();
    let mut start_from = 0;
    for battery in (0..12).rev() {
        let mut max = 0;
        for i in start_from..battery_bank.len() - battery {
            if battery_bank[i] > max {
                max = battery_bank[i];
                start_from = i + 1;
            }
        }
        batteries.push(max);
    }
    batteries
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
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
        let input = "src/day3/input.txt";
        assert_eq!(17244, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day3/input.txt";
        assert_eq!(171435596092638, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day3/test-input.txt";
        let result = part_a(input);
        assert_eq!(357, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day3/test-input.txt";
        let result = part_b(input);
        assert_eq!(3121910778619, result);
    }

    #[test]
    fn big_joltages_test_1() {
        let input = "987654321111111";
        let result = big_joltages(&battery_bank(input));
        assert_eq!(987654321111, result);
    }
    #[test]
    fn big_joltages_test_2() {
        let input = "811111111111119";
        let result = big_joltages(&battery_bank(input));
        assert_eq!(811111111119, result);
    }
    #[test]
    fn big_joltages_test_3() {
        let input = "234234234234278";
        let result = big_joltages(&battery_bank(input));
        assert_eq!(434234234278, result);
    }
    #[test]
    fn big_joltages_test_4() {
        let input = "818181911112111";
        let result = big_joltages(&battery_bank(input));
        assert_eq!(888911112111, result);
    }
}
