use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day1() {
    println!("== Day 1 ==");
    let input = "src/day1/input.txt";
    time(part_a, input, "A_1");
    time(part_a_2, input, "A_2");
    time(part_b, input, "B_1");
    time(part_b_2, input, "B_2");
}

fn part_a(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let mut numbers: Vec<usize> = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let mut s: Vec<char> = Vec::with_capacity(2);
        for c in line.chars() {
            if c.is_digit(10) {
                s.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                s.push(c);
                break;
            }
        }
        numbers.push(s.into_iter().collect::<String>().parse::<usize>().unwrap())
    }
    numbers.iter().sum()
}

fn part_b(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let text_numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let mut numbers: Vec<usize> = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        // println!("On line: {}", line);

        let mut s: Vec<char> = Vec::with_capacity(2);

        let mut queue = VecDeque::with_capacity(5);
        // println!("CHECKING FRONT");
        for c in line.chars() {
            if s.len() == 1 { break; }
            // println!("Checking: {}", c);
            if c.is_digit(10) {
                // println!("Was a digit: {}", c);
                s.push(c);
                break;
            } else {
                // println!("Was NOT a digit: {}", c);
                queue.push_back(c)
            }
            let textq: String = queue.iter().collect();
            for (k, v) in text_numbers.iter() {
                // println!("Checking if text is number: {} == {}", textq, k);
                if textq.contains(k) {
                    // println!("It was a number! {}", k);
                    s.push(*v);
                    break;
                }
            }
        }
        // println!("\nCHECKING BACK");
        queue.clear();
        for c in line.chars().rev() {
            if s.len() == 2 { break; }
            // println!("Checking: {}", c);
            if c.is_digit(10) {
                // println!("Was a digit: {}", c);
                s.push(c);
                break;
            } else {
                // println!("Was NOT a digit: {}", c);
                queue.push_front(c)
            }
            let textq: String = queue.iter().collect();
            for (k, v) in text_numbers.iter() {
                // println!("Checking if text is number: {} == {}", textq, k);
                if textq.contains(k) {
                    // println!("It was a number! {}", k);
                    s.push(*v);
                    break;
                }
            }
        }
        // println!("Numbers: {:?} \n\n", numbers);
        numbers.push(s.into_iter().collect::<String>().parse::<usize>().unwrap())
    }
    numbers.iter().sum()
}

fn part_a_2(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let mut numbers: Vec<usize> = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let s: Vec<char> = line.chars().into_iter().filter(|c| c.is_digit(10)).collect();
        let p: String = [*s.first().unwrap(), *s.last().unwrap()].iter().collect();
        numbers.push(p.parse::<usize>().unwrap());
    }
    numbers.iter().sum()
}

fn part_b_2(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let mut numbers: Vec<usize> = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let mut s: Vec<char> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                s.push(c)
            }
            for (n, v) in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate() {
                if line.split_at(i).1.starts_with(v) {
                    s.push((n + 1).to_string().chars().last().unwrap())
                }
            }
        }
        let p: String = [*s.first().unwrap(), *s.last().unwrap()].iter().collect();
        numbers.push(p.parse::<usize>().unwrap());
    }
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let input = "src/day1/test-input.txt";
        time(part_a, input, "A_1");
        time(part_a_2, input, "A_2");
        assert_eq!(142, part_a(input));
        assert_eq!(142, part_a_2(input));
    }

    #[test]
    fn real_a() {
        assert_eq!(54605, part_a_2("src/day1/input.txt"))
    }

    #[test]
    fn part_b_test_input() {
        assert_eq!(281, part_b("src/day1/test-input-b.txt"))
    }

    #[test]
    fn real_b() {
        let input = "src/day1/input.txt";
        time(part_b, input, "B_1");
        time(part_b_2, input, "B_2");
        assert_eq!(55429, part_b(input));
        assert_eq!(55429, part_b_2(input));
    }
}
