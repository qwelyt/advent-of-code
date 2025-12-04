use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day9() {
    println!("== Day 9 ==");
    let input = "src/day9/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> i32 {
    let file = File::open(input).unwrap();
    BufReader::new(file)
        .lines()
        .flatten()
        .map(|s| s.split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|v| find_next_number(v, |v| *v.last().unwrap(), |x, y| x + y))
        .sum()
}

fn part_b(input: &str) -> i32 {
    let file = File::open(input).unwrap();
    BufReader::new(file)
        .lines()
        .flatten()
        .map(|s| s.split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|v| find_next_number(v, |v| *v.first().unwrap(), |x, y| x - y))
        .sum()
}

fn find_next_number(input: Vec<i32>, next_number: fn(Vec<i32>) -> i32, calc: fn(i32, i32) -> i32) -> i32 {
    // println!("Input: {:?}", input);
    let mut differences = Vec::new();
    for (i, _n) in input.iter().enumerate() {
        if i == input.len() - 1 {
            break;
        }
        let a = input.get(i + 1).unwrap() - input.get(i).unwrap();
        differences.push(a);
    }
    // println!("Diff:  {:?}",differences);
    let set: HashSet<&i32> = HashSet::from_iter(differences.iter());
    let last_from_input = next_number(input); //input.last().unwrap();
    if set.len() == 1 {
        let y = differences.last().unwrap();
        let next = calc(last_from_input, *y);
        // println!("Returning {} + {} = {}", last_from_input, y, next);
        return next;
    }
    // println!("Go again");
    let nn = find_next_number(differences, next_number, calc);
    let next = calc(last_from_input, nn);
    // println!("Returning {} + {} = {}",last_from_input,nn,next);
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day9();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day9/input.txt";
        assert_eq!(1743490457, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day9/input.txt";
        assert_eq!(1053, part_b(input));
    }

    #[test]
    fn fnn_test() {
        let f: fn(Vec<i32>) -> i32 = |v| *v.last().unwrap();
        let g: fn(i32, i32) -> i32 = |x, y| x + y;
        {
            let input = vec![1, 2, 3, 4, 5];
            let result = find_next_number(input, f, g);
            assert_eq!(6, result)
        }
        {
            let input = vec![2, 4, 6, 8, 10];
            let result = find_next_number(input, f, g);
            assert_eq!(12, result)
        }
        {
            let input = vec![0, 3, 6, 9, 12, 15];
            let result = find_next_number(input, f, g);
            assert_eq!(18, result)
        }
        {
            let input = vec![1, 3, 6, 10, 15, 21];
            let result = find_next_number(input, f, g);
            assert_eq!(28, result)
        }
        {
            let input = vec![10, 13, 16, 21, 30, 45];
            let result = find_next_number(input, f, g);
            assert_eq!(68, result)
        }
    }

    #[test]
    fn asd() {
        {
            let input = vec![1, 3, 6, 10, 15, 21];
            let result = find_next_number(input, |v| *v.last().unwrap(), |a, b| a + b);
            assert_eq!(28, result)
        }
    }

    #[test]
    fn fpn() {
        let f: fn(Vec<i32>) -> i32 = |v| *v.first().unwrap();
        let g: fn(i32, i32) -> i32 = |x, y| x - y;
        {
            let input = vec![0, 3, 6, 9, 12, 15];
            let result = find_next_number(input, f, g);
            assert_eq!(-3, result)
        }
        {
            let input = vec![1, 3, 6, 10, 15, 21];
            let result = find_next_number(input, f, g);
            assert_eq!(0, result)
        }
        {
            let input = vec![10, 13, 16, 21, 30, 45];
            let result = find_next_number(input, f, g);
            assert_eq!(5, result)
        }
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day9/test-input.txt";
        let result = part_a(input);
        assert_eq!(114, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day9/test-input.txt";
        let result = part_b(input);
        assert_eq!(2, result);
    }
}