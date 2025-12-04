use crate::util::time;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 3 ==");
    let input = "src/day3/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> i32 {
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
    let string = File::open(input)
        .map(|f| BufReader::new(f).lines().flatten().collect::<Vec<String>>())
        .unwrap()
        .join("");

    //println!("{:?}", string);
    let matches: Vec<&str> = regex
        .captures_iter(&string)
        .map(|cap| {
            let (_, [s]) = cap.extract();
            s
        })
        .collect();
    //println!("{:?}", matches);

    let muls = matches
        .iter()
        .map(|s| s.replace("mul(", "").replace(")", ""))
        .map(|s| {
            s.split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    //println!("{:?}", muls);

    muls.iter().map(|v| v.iter().product::<i32>()).sum()
}

fn part_b(input: &str) -> i32 {
    let string = File::open(input)
        .map(|f| BufReader::new(f).lines().flatten().collect::<Vec<String>>())
        .unwrap()
        .join("");
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    let matches: Vec<&str> = regex
        .captures_iter(&string)
        .map(|cap| {
            let (_, [s]) = cap.extract();
            s
        })
        .collect();

    // println!("{:?}", string);
    // println!("{:?}", matches);

    let mut add = true;
    let mut v = Vec::new();
    for m in matches {
        if m.starts_with("mul") && add {
            v.push(m)
        } else if m.eq("don't()") {
            add = false;
        } else if m.eq("do()") {
            add = true;
        }
    }
    // println!("{:?}", v);

    v.iter()
        .map(|s| s.replace("mul(", "").replace(")", ""))
        .map(|s| {
            s.split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| v.iter().product::<i32>())
        .sum()
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
        assert_eq!(160672468, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day3/input.txt";
        assert_eq!(84893551, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day3/test-input.txt";
        let result = part_a(input);
        assert_eq!(161, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day3/test-input2.txt";
        let result = part_b(input);
        assert_eq!(48, result);
    }
}
