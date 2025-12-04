use std::collections::HashMap;

use crate::util::{lines_from_file, string_to_i32};

pub fn day6() {
    println!("== Day 6 ==");
    let input = lines_from_file("src/day6/input.txt");
    let a = part_a(&input, 80);
    println!("Part A: {}", a);
    let b = part_b(&input, 256);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>, days: i32) -> usize {
    let mut current_fishes: Vec<i32> = input.get(0).unwrap().split(",").map(|s| string_to_i32(s)).collect();
    for _ in 0..days {
        // println!("Current gen {}: {:?}",i,&current_fishes);
        let mut next_generation: Vec<i32> = Vec::new();
        for fish in &current_fishes {
            if *fish < 1 {
                next_generation.push(6);
                next_generation.push(8);
            } else {
                next_generation.push(*fish - 1);
            }
        }
        // println!("Next gen: {:?}",&next_generation);
        current_fishes = next_generation;
    }
    current_fishes.len()
}

fn part_b(input: &Vec<String>, days: usize) -> usize {
    let fishes: Vec<u8> = input.get(0).unwrap().split(",").map(|s| s.parse::<u8>().unwrap()).collect();
    let mut map: HashMap<u8, usize> = HashMap::new();

    // Count how many fishes we have for each cycle
    for fish in &fishes {
        let num_fishes = *map.get(fish).unwrap_or(&0);
        map.insert(*fish, num_fishes + 1);
    }

    for _ in 0..days {
        let mut m: HashMap<u8, usize> = HashMap::new();
        for (i, v) in &map {
            if *i == 0 { // If we have fishes with 0 days left, spawn. But the old ones at 6 and new ones at 8
                *m.entry(6).or_insert(0) += v;
                *m.entry(8).or_insert(0) += v;
            } else { // Otherwise just count down
                *m.entry(*i - 1).or_insert(0) += v;
            }
        }
        map = m;
    }

    // println!("{:?}", &fishes);
    // println!("{:?}", map);
    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day6/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input, 80);
        assert_eq!(5934, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day6/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input, 80);
        assert_eq!(375482, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day6/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input, 256);
        assert_eq!(26984457539, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day6/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input, 256);
        assert_eq!(1689540415957, result);
    }
}
