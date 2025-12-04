use crate::util::time;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 8 ==");
    let input = "src/day8/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let map = map(input);
    let antennas = antennas(&map);
    // println!("{:?}", antennas);

    let mut antinodes = HashSet::new();
    for nodes in antennas.values() {
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let (r1, c1) = nodes[i];
                let (r2, c2) = nodes[j];

                let a = (2 * r1 - r2, 2 * c1 - c2);
                if in_bounds(a, &map) {
                    antinodes.insert(a);
                }

                let b = (2 * r2 - r1, 2 * c2 - c1);
                if in_bounds(b, &map) {
                    antinodes.insert(b);
                }
            }
        }
    }

    antinodes.len()
}

fn in_bounds(location: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    location.0 >= 0
        && location.0 < map.len() as i32
        && location.1 >= 0
        && location.1 < map[0].len() as i32
}

fn part_b(input: &str) -> usize {
    let map = map(input);
    let antennas = antennas(&map);

    let mut antinodes = HashSet::new();
    for nodes in antennas.values() {
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let (r1, c1) = nodes[i];
                let (r2, c2) = nodes[j];

                let dr = r2 - r1;
                let dc = c2 - c1;
                let mut r = r1;
                let mut c = c1;
                while in_bounds((r, c), &map) {
                    antinodes.insert((r, c));
                    r -= dr;
                    c -= dc;
                }
                let mut r = r1;
                let mut c = c1;
                while in_bounds((r, c), &map) {
                    antinodes.insert((r, c));
                    r += dr;
                    c += dc;
                }
            }
        }
    }

    antinodes.len()
}

fn map(input: &str) -> Vec<Vec<char>> {
    File::open(input)
        .map(|file| {
            BufReader::new(file)
                .lines()
                .flatten()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .unwrap()
}

fn antennas(map: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas = HashMap::new();
    for (r, row) in map.iter().enumerate() {
        for (c, node) in row.iter().enumerate() {
            if node == &'.' {
                continue;
            }
            antennas
                .entry(*node)
                .or_insert(Vec::new())
                .push((r as i32, c as i32));
        }
    }
    antennas
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
        let input = "src/day8/input.txt";
        assert_eq!(392, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day8/input.txt";
        assert_eq!(1235, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day8/test-input.txt";
        let result = part_a(input);
        assert_eq!(14, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day8/test-input.txt";
        let result = part_b(input);
        assert_eq!(34, result);
    }
}
