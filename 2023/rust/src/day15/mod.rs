use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day15() {
    println!("== Day 15 ==");
    let input = "src/day15/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let file = File::open(input).unwrap();
    BufReader::new(file).lines()
        .flatten()
        .map(|l| l.split(",").collect::<Vec<&str>>()
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
        )
        .flatten()
        .map(|v| hash(&v))
        .sum()
}

fn part_b(input: &str) -> usize {
    let file = File::open(input).unwrap();
    let sequences = BufReader::new(file).lines()
        .flatten()
        .map(|l| l.split(",").collect::<Vec<&str>>()
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
        )
        .flatten()
        .collect::<Vec<Vec<char>>>();
    focal_power(&sequences)
}

fn hash(chars: &Vec<char>) -> usize {
    let mut sum = 0;
    for c in chars.iter() {
        let u = *c as u32;
        sum += u;
        sum *= 17;
        sum = sum % 256;
    }

    sum as usize
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn focal_power(sequences: &Vec<Vec<char>>) -> usize {
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();
    for v in sequences.iter() {
        // println!("{:?} -{:?} ={:?}", v, v.iter().position(|&c| c == '-'), v.iter().position(|&c| c == '='));
        let dash = v.iter().position(|&c| c == '-');
        let equal = v.iter().position(|&c| c == '=');
        let split_pos = dash.or(equal).unwrap();
        let (chars, strength) = v.split_at(split_pos);
        let chars = chars.iter().map(|&c| c).collect::<Vec<char>>();

        let box_number = hash(&chars);
        let label = chars.iter().collect::<String>();

        let pos = boxes.get(&box_number).unwrap_or(&Vec::new()).iter().position(|b| b.label == label);
        if dash.is_some() {
            if pos.is_some() {
                boxes.entry(box_number).or_default().remove(pos.unwrap());
            }
        } else if equal.is_some() {
            let lens = Lens { label, focal_length: strength.iter().skip(1).collect::<String>().parse::<usize>().unwrap() };
            if pos.is_some() {
                boxes.entry(box_number).or_default().remove(pos.unwrap());
                boxes.entry(box_number).or_default().insert(pos.unwrap(), lens);
            } else {
                boxes.entry(box_number).or_default().push(lens);
            }
        }
        // println!("After {}", v.iter().collect::<String>());
        // for (number, lenses) in boxes.iter() {
        //     println!("Box {}: {:?}", number, lenses);
        // }
        // println!()
    }

    /*

    rn: 1 (box 0) * 1 (first slot) * 1 (focal length) = 1
    cm: 1 (box 0) * 2 (second slot) * 2 (focal length) = 4
    ot: 4 (box 3) * 1 (first slot) * 7 (focal length) = 28
    ab: 4 (box 3) * 2 (second slot) * 5 (focal length) = 40
    pc: 4 (box 3) * 3 (third slot) * 6 (focal length) = 72

     */
    let mut power = 0;
    for (number, lenses) in boxes.iter() {
        for (n, lens) in lenses.iter().enumerate() {
            let box_power = (number + 1) * (n + 1) * lens.focal_length;
            // println!("{}", box_power);
            power += box_power
        }
    }
    power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day15();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day15/input.txt";
        assert_eq!(504449, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day15/input.txt";
        assert_eq!(262044, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day15/test-input.txt";
        let result = part_a(input);
        assert_eq!(1320, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day15/test-input.txt";
        let result = part_b(input);
        assert_eq!(145, result);
    }

    #[test]
    fn test_hash() {
        let chars = "HASH".chars().collect::<Vec<char>>();
        let result = hash(&chars);
        assert_eq!(52, result)
    }
}