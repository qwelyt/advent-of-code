use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::lines_from_file;

pub fn day10() {
    println!("== Day 10 ==");
    let input = lines_from_file("src/day10/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> i32 {
    let lines: Vec<Vec<char>> = input.iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let open_close: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>')
    ]);
    let opening: HashSet<char> = open_close.keys().map(|a| *a).collect();
    let scores: HashMap<char, i32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    let mut illegals: HashMap<char, i32> = open_close.iter()
        .map(|(_, v)| (*v, 0))
        .collect();
    for line in lines {
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in line {
            if opening.contains(&c) {
                stack.push_back(c);
            } else { // Closing
                if stack.is_empty() {
                    panic!("Closing without any open left");
                } else {
                    let oc = stack.pop_back().unwrap();
                    if c != *(&open_close).get(&oc).unwrap() {
                        // println!("Found {} - {} to be incorrect", oc, c);
                        if let Some(x) = illegals.get_mut(&c) {
                            *x += 1;
                        }
                    }
                }
            }
        }
    }

    // println!("{:?}", illegals);
    let score: Vec<i32> = illegals.iter().map(|(c, n)| scores.get(c).unwrap_or(&0) * *n).collect();
    // println!("{:?}", score);

    score.iter().sum()
}

fn part_b(input: &Vec<String>) -> u64 {
    let lines: Vec<Vec<char>> = input.iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let open_close: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>')
    ]);
    let opening: HashSet<char> = open_close.keys().map(|a| *a).collect();
    let scores: HashMap<char, i32> = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4)
    ]);
    let mut unclosed: Vec<VecDeque<char>> = Vec::new();
    for line in lines {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut add_stack = true;
        for c in line {
            if opening.contains(&c) {
                stack.push_back(c);
            } else { // Closing
                if stack.is_empty() {
                    panic!("Closing without any open left");
                } else {
                    let oc = stack.pop_back().unwrap();
                    if c != *open_close.get(&oc).unwrap() {
                        add_stack = false;
                    }
                }
            }
        }
        if add_stack {
            unclosed.push(stack);
        }
    }
    // println!("{:?}",unclosed);
    let mut completion: Vec<Vec<char>> = Vec::new();
    for stack in unclosed.iter_mut() {
        let mut v: Vec<char> = Vec::new();
        while !stack.is_empty() {
            let close_this = stack.pop_back().unwrap();
            v.push(*open_close.get(&close_this).unwrap());
        }
        completion.push(v);
    }
    // println!("{:?}",completion);

    let mut completion_score: Vec<u64> = completion.iter()
        .map(|l| l.iter()
            .map(|c| *scores.get(c).unwrap() as u64)
            .fold(0, |total, next| total * 5 + next))
        .collect();

    completion_score.sort();
    // println!("{:?}", completion_score);
    *completion_score.get(completion_score.len() / 2).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day10/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(26397, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day10/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(339411, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day10/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(288957, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day10/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(2289754624, result);
    }
}
