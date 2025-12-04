use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day5() {
    println!("== Day 5 ==");
    let input = "src/day5/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> String {
    let (t, instructions) = parse_input(input);
    let towers = build_towers(t);
    // print_towers(&towers);
    let result = follow_instructions(towers, instructions);
    // let result:Vec<Vec<String>> = Vec::new();

    let mut v = Vec::new();
    for tower in result.iter() {
        v.push(tower.last().unwrap().as_str())
    }
    v.concat()
}

fn print_towers(towers: &Vec<Vec<String>>) {
    for t in towers.iter().enumerate() {
        println!("{}: {:?}", t.0, t.1);
    }
}

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut t = Vec::new();
    let mut instructions = Vec::new();
    let open = File::open(input).expect("Could not read file");
    let mut towers = true;
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            towers = false;
            continue;
        }
        if towers {
            t.push(line);
        } else {
            instructions.push(line);
        }
    }
    (t, instructions)
}

fn follow_instructions(initial_towers: Vec<Vec<String>>, instructions: Vec<String>) -> Vec<Vec<String>> {
    let mut towers = initial_towers.clone();
    for line in instructions.iter() {
        let instruction: Vec<&str> = line.split(" ").collect();
        // println!("{:?}", instruction);
        let amount = instruction.get(1).map(|s| *s).map(|s| s.parse::<i32>()).unwrap().unwrap();
        let from = instruction.get(3).map(|s| *s).map(|s| s.parse::<usize>()).unwrap().unwrap() - 1;
        let to = instruction.get(5).map(|s| *s).map(|s| s.parse::<usize>()).unwrap().unwrap() - 1;
        // println!("{}, {}, {}", amount, from, to);
        // println!("Before");
        // for t in towers.iter().enumerate() {
        //     println!("{}: {:?}", t.0, t.1);
        // }
        for _ in 0..amount {
            let string = towers[from].pop();
            towers[to].push(string.unwrap());
        }
        // println!("After");
        // for t in towers.iter().enumerate() {
        //     println!("{}: {:?}", t.0, t.1);
        // }
        // println!();
    }
    towers
}

fn build_towers(initial: Vec<String>) -> Vec<Vec<String>> {
    let mut towers = Vec::new();
    // println!("{:?}", initial);
    let (last, t) = initial.split_last().unwrap();
    let num_towers = last.chars().into_iter().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10)).max().unwrap().unwrap();
    // println!("nt: {}", num_towers);
    for _ in 0..num_towers {
        towers.push(Vec::new());
    }
    for line in t.iter() {
        let chars = line.split("").collect::<Vec<&str>>();
        for i in (2..chars.len()).step_by(4) {
            let into_tower = ((i - 2) / 4) % (num_towers as usize);
            // println!("{}, {}({}) -> {}", i, chars[i], !chars[i].trim().is_empty(), into_tower);
            if !chars[i].trim().is_empty() {
                // println!("Tower: {} ({}), Value: {}", into_tower, i, chars[i]);
                towers[into_tower].insert(0, chars[i].to_string())
            }
        }
    }
    towers
}

fn part_b(input: &str) -> String {
    let (tower_data, instructions) = parse_input(input);
    let towers = build_towers(tower_data);
    let result = follow_bulk_instructions(towers, instructions);
    // let result:Vec<Vec<String>> = Vec::new();

    let mut v = Vec::new();
    for tower in result.iter() {
        v.push(tower.last().unwrap().as_str())
    }
    v.concat()
}

fn follow_bulk_instructions(initial_towers: Vec<Vec<String>>, instructions: Vec<String>) -> Vec<Vec<String>> {
    let mut towers = initial_towers.clone();
    for line in instructions.iter() {
        let instruction: Vec<&str> = line.split(" ").collect();
        let amount = instruction.get(1).map(|s| *s).map(|s| s.parse::<i32>()).unwrap().unwrap();
        let from = instruction.get(3).map(|s| *s).map(|s| s.parse::<usize>()).unwrap().unwrap() - 1;
        let to = instruction.get(5).map(|s| *s).map(|s| s.parse::<usize>()).unwrap().unwrap() - 1;
        let i = towers[from].len() - amount as usize;
        let mut containers = towers[from].drain(i..).collect();
        towers[to].append(&mut containers)
    }
    towers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day5();
    }

    #[ignore]
    #[test]
    fn print() {
        let input = "src/day5/test-input.txt";
        let (t, _) = parse_input(input);
        let towers = build_towers(t);
        print_towers(&towers);
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day5/test-input.txt";
        let result = part_a(input);
        assert_eq!("CMZ", result);
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day5/input.txt";
        let result = part_a(input);
        assert_eq!("CWMTGHBDW", result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day5/test-input.txt";
        let result = part_b(input);
        assert_eq!("MCD", result);
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day5/input.txt";
        let result = part_b(input);
        assert_eq!("SSCGWJCRB", result);
    }
}