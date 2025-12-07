use crate::util::time;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 7 ==");
    let input = "src/day7/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let tachyon_field = TachyonField::parse(input);
    tachyon_field.shoot_beam()
}

fn part_b(input: &str) -> usize {
    let tachyon_field = TachyonField::parse(input);
    tachyon_field.shoot_quantum_beam()
}

struct TachyonField {
    field: Vec<Vec<char>>,
}
impl TachyonField {
    fn parse(input: &str) -> Self {
        let field = File::open(input)
            .map(BufReader::new)
            .map(|reader| {
                reader
                    .lines()
                    .flatten()
                    .map(|line| line.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>()
            })
            .expect("Should have input");
        TachyonField { field }
    }

    fn shoot_beam(self) -> usize {
        let mut splits = 0;
        let start = self.field[0]
            .iter()
            .position(|c| *c == 'S')
            .expect("Should have a start");
        let mut positions = HashSet::new();
        positions.insert(start);
        let mut new_positions = HashSet::new();
        for i in 1..self.field.len() {
            for &ii in positions.iter() {
                match self.field[i][ii] {
                    '.' => _ = new_positions.insert(ii),
                    '^' => {
                        _ = new_positions.insert(ii - 1);
                        _ = new_positions.insert(ii + 1);
                        splits += 1
                    }
                    _ => panic!("Should unknown! {:?}", self.field[i][ii]),
                }
            }
            positions = new_positions;
            new_positions = HashSet::new();
        }
        splits
    }
    fn shoot_quantum_beam(self) -> usize {
        let start = self.field[0]
            .iter()
            .position(|c| *c == 'S')
            .expect("Should have a start");
        let mut positions: HashMap<usize, usize> = HashMap::new();
        _ = positions.insert(start, 1);
        let mut new_positions: HashMap<usize, usize> = HashMap::new();
        for i in 1..self.field.len() {
            for (&ii, count) in positions.iter() {
                match self.field[i][ii] {
                    '.' => {
                        _ = {
                            if !new_positions.contains_key(&ii) {
                                new_positions.insert(ii, 0);
                            }
                            *new_positions.entry(ii).or_default() += count;
                        }
                    }
                    '^' => {
                        if !new_positions.contains_key(&(ii - 1)) {
                            new_positions.insert(ii - 1, 0);
                        }
                        if !new_positions.contains_key(&(ii + 1)) {
                            new_positions.insert(ii + 1, 0);
                        }
                        *new_positions.entry(ii - 1).or_default() += count;
                        *new_positions.entry(ii + 1).or_default() += count;
                    }
                    _ => panic!("Should unknown! {:?}", self.field[i][ii]),
                }
            }
            positions = new_positions;
            new_positions = HashMap::new();
        }
        positions.iter().map(|(_, v)| v).sum::<usize>()
    }
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
        let input = "src/day7/input.txt";
        assert_eq!(1566, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day7/input.txt";
        assert_eq!(5921061943075, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day7/test-input.txt";
        let result = part_a(input);
        assert_eq!(21, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day7/test-input.txt";
        let result = part_b(input);
        assert_eq!(40, result);
    }
}
