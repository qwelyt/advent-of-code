use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day8() {
    println!("== Day 8 ==");
    let input = "src/day8/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let (instructions, nodes) = read_input(input);
    solve_a("AAA", |s| s.eq("ZZZ"), &instructions, &nodes)
}

fn read_input(input: &str) -> (Vec<String>, HashMap<String, (String, String)>) {
    let file = File::open(input).unwrap();
    let mut instructions = Vec::new();
    let mut nodes = HashMap::new();
    for (i, line) in BufReader::new(file).lines().enumerate() {
        if i == 0 {
            instructions = line.map(|s| s.as_str().split("")
                .filter(|a| !a.is_empty())
                .map(|a| a.to_string())
                .collect())
                .unwrap();
        } else if i > 1 {
            let line = line.unwrap();
            let (node, paths) = line.split_once(" = ").unwrap();
            let paths_ = paths.replace("(", "").replace(")", "");
            let (l, r) = paths_.split_once(", ").unwrap();
            nodes.insert(node.to_string(), (l.to_string(), r.to_string()));
        }
    }
    (instructions, nodes)
}

fn solve_a(start: &str, end: fn(&str) -> bool, instructions: &Vec<String>, nodes: &HashMap<String, (String, String)>) -> usize {
    let mut current_node = start;
    let mut steps_taken = 0;
    // println!("{:?}", instructions);
    while !end(current_node) {
        let i = steps_taken % instructions.len();
        // println!("Current node: {:?}, steps_taken: {:?}, i: {:?}", current_node, steps_taken, i);
        let l_or_r = instructions.get(i).unwrap().as_str();
        let x = nodes.get(current_node).unwrap();
        current_node = match l_or_r {
            "L" => x.0.as_str(),
            "R" => x.1.as_str(),
            &_ => panic!("WHAAT!? {:?} :: {:?}", l_or_r, steps_taken)
        };
        steps_taken += 1;
    }
    steps_taken
}

fn part_b(input: &str) -> usize {
    let (instructions, nodes) = read_input(input);
    let start_nodes = nodes.keys()
        .filter(|&n| n.ends_with("A"))
        .map(String::as_str)
        .collect::<Vec<&str>>();

    // println!("{:?}", start_nodes);
    let is_end: fn(&str) -> bool = |s| s.ends_with("Z");
    let steps = start_nodes.iter()
        .map(|n| solve_a(n, is_end, &instructions, &nodes))
        .collect::<Vec<usize>>();
    // println!("{:?}", steps);
    steps.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
}

fn lcm(a: usize, b: usize) -> usize {
    let mut tmp = a;
    while tmp % b != 0 {
        tmp += a;
    }
    return tmp;
}

// DEAD STARTS
fn _solve_b(start_node: &str, is_end: fn(&str) -> bool, steps_already_taken: usize, instructions: &Vec<String>, nodes: &HashMap<String, (String, String)>) -> usize {
    let mut current_node = start_node;
    let mut steps_taken = steps_already_taken;
    while !is_end(current_node) {
        let i = steps_taken % instructions.len();
        // println!("Current node: {:?}, steps_taken: {:?}, i: {:?}", current_node, steps_taken, i);
        let l_or_r = instructions.get(i).unwrap().as_str();
        let x = nodes.get(current_node).unwrap();
        current_node = match l_or_r {
            "L" => x.0.as_str(),
            "R" => x.1.as_str(),
            &_ => panic!("WHAAT!? {:?} :: {:?}", l_or_r, steps_taken)
        };
        steps_taken += 1;
    }
    steps_taken
}

fn _solve_b_2(start: Vec<&str>, is_end: fn(&str) -> bool, instructions: &Vec<String>, nodes: &HashMap<String, (String, String)>) -> usize {
    let mut current_nodes = start;
    let mut steps_taken = 0;
    // println!("{:?}", instructions);
    while current_nodes.iter().filter(|n| !is_end(n)).count() != 0 {
        let i = steps_taken % instructions.len();
        let l_or_r = instructions.get(i).unwrap().as_str();
        // println!("Current nodes: {:?}, steps_taken: {:?}, i: {:?}, LR: ", current_nodes[0], steps_taken, i);
        let new_nodes = current_nodes.iter()
            .map(|n| match l_or_r {
                "L" => nodes.get(*n).unwrap().0.as_str(),
                "R" => nodes.get(*n).unwrap().1.as_str(),
                &_ => panic!("WHAAT!? {:?} :: {:?}", l_or_r, steps_taken)
            })
            .collect::<Vec<&str>>();
        current_nodes = new_nodes;

        steps_taken += 1;
    }
    steps_taken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day8();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day8/input.txt";
        assert_eq!(16043, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day8/input.txt";
        assert_eq!(15726453850399, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day8/test-input.txt";
        let result = part_a(input);
        assert_eq!(6, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day8/test-input-b.txt";
        let result = part_b(input);
        assert_eq!(6, result);
    }

    #[test]
    fn different_nodes() {
        let input = "src/day8/input.txt";
        let (instructions, nodes) = read_input(input);

        let start_nodes = nodes.keys()
            .filter(|&n| n.ends_with("A"))
            .map(String::as_str)
            .collect::<Vec<&str>>();
        println!("{:?}", start_nodes);
        let is_end: fn(&str) -> bool = |s| s.ends_with("Z");
        for start in start_nodes.iter() {
            let result = solve_a(start, is_end, &instructions, &nodes);
            println!("{:?} = {:?}", start, result);
        }
        println!("\nTest B");
        for start in start_nodes.iter() {
            let result = _solve_b_2(vec![start], is_end, &instructions, &nodes);
            println!("{:?} = {:?}", start, result);
        }

        println!("\nTest B sub");
        let test_with = vec![start_nodes[0], start_nodes[1], start_nodes[2]];
        let result = _solve_b_2(test_with.clone(), is_end, &instructions, &nodes);
        println!("{:?} = {:?}", test_with.clone(), result);
        assert_eq!(6, 0);
    }
}