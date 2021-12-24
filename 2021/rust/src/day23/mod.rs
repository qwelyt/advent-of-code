use std::collections::{HashMap, HashSet};

use crate::util::lines_from_file;

pub fn day23() {
    println!("== Day 23 ==");
    let input = lines_from_file("src/day23/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

#[derive(Debug)]
struct Data {
    hallway: Vec<char>,
    locations: HashMap<char, Vec<(usize, usize)>>,
    destinations: HashMap<char, Vec<(usize, usize)>>,
}

fn part_a(input: &Vec<String>) -> usize {
    let mut hallway = Vec::new();
    let mut rooms: Vec<Vec<char>> = Vec::new();
    for line in input.iter() {
        if line == "#############" { continue; }
        if line == "  #########  " { continue; }
        if line.starts_with("#.") {
            hallway = line.chars().filter(|c| *c != '#').collect::<Vec<char>>();
        } else {
            rooms.push(line.chars()
                // .enumerate()
                .skip(1) // Skip first #
                // .filter_map(|(index, c)| if index == line.len() - 1 { None } else { Some(c) }) // Skip last #
                .map(|c| {
                    match c {
                        ' ' => '#',
                        _ => c
                    }
                }).collect::<Vec<char>>());
        }
    }
    let mut locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (ri, r) in rooms.iter().enumerate() {
        for (ci, c) in r.iter().enumerate() {
            if *c == '#' { continue; }
            locations.entry(*c).or_insert(Vec::new()).push((ri + 1, ci))
        }
    }
    let destinations = HashMap::from([
        ('A', vec![(1, 2), (2, 2)]),
        ('B', vec![(1, 4), (2, 4)]),
        ('C', vec![(1, 6), (2, 6)]),
        ('D', vec![(1, 8), (2, 8)]),
    ]);
    let data = Data { hallway, locations, destinations };

    println!("{:?}", data);
    // TODO
    0
}

fn part_b(input: &Vec<String>) -> usize {
    // TODO
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day23/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(12521, result)
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day23/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(true, result < 18322);
        assert_eq!(1, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day23/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(2758514936282235, result)
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day23/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(1288707160324706, result);
    }
}
