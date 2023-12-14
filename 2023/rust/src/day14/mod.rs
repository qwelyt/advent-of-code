use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day14() {
    println!("== Day 14 ==");
    let input = "src/day14/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

struct Platform {
    rocks: Vec<Vec<char>>,
}

impl Platform {
    fn of(rocks: Vec<Vec<char>>) -> Self {
        Self { rocks }
    }
    fn parse(input: &str) -> Self {
        let rocks = File::open(input)
            .map(|f| BufReader::new(f))
            .map(|b| b.lines().flatten()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>())
            .unwrap();
        Platform::of(rocks)
    }

    fn cycle_and_count(&self, cycles: usize) -> usize {
        // Platform::_print_vec(&self.rocks);
        let rotated = Platform::rotate(&self.rocks); // Now we can work on rows only
        let mut weight = 0;
        if cycles == 0 {
            let rolled_rocks = Platform::roll_rocks(&rotated);
            weight += Platform::count_weight(&rolled_rocks);
        } else {
            let mut seen_grids: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
            let mut rolled_rocks = rotated.clone();
            let mut cycle = 0;
            while cycle < cycles {
                for _ in 0..4 {
                    rolled_rocks = Platform::roll_rocks(&rolled_rocks);
                    rolled_rocks = Platform::rotate(&rolled_rocks);
                }
                if seen_grids.contains_key(&rolled_rocks) {
                    let cycle_length = cycle - seen_grids.get(&rolled_rocks).unwrap();
                    // println!("{} || I've seen this before! {:?} => {:?} || {:?}", cycle, seen_grids.get(&rolled_rocks), cycle_length, cycles);
                    let cycles_in_target = (cycles - cycle) / cycle_length;
                    cycle += cycles_in_target * cycle_length;
                    // println!("Jumping to {:?}", cycle);
                }
                let p = cycle;
                *seen_grids.entry(rolled_rocks.clone()).or_default() = p;
                cycle += 1;
            }
            weight += Platform::count_weight(&rolled_rocks);
        }

        weight
    }
    fn roll_rocks(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut rolled_rocks = rocks.clone();
        // Platform::_print_vec(&rolled_rocks);
        let n_rows = rocks.len();
        let n_cols = rocks[0].len();
        for r in 0..n_rows {
            for _ in 0..n_cols {
                for c in (0..n_cols - 1).rev() {
                    // println!("C == {:?}", c);
                    // println!("rc = {:?}", rolled_rocks[r][c]);
                    // println!("rc+1 = {:?}", rolled_rocks[r][c+1]);
                    if rolled_rocks[r][c] == 'O' && rolled_rocks[r][c + 1] == '.' {
                        // println!("Should roll");
                        // println!("{:?}", rolled_rocks[r]);
                        rolled_rocks[r][c] = '.';
                        rolled_rocks[r][c + 1] = 'O';
                        // println!("{:?}", rolled_rocks[r]);
                    }
                }
            }
        }
        // Platform::_print_vec(&rolled_rocks);
        rolled_rocks
    }
    fn count_weight(rocks: &Vec<Vec<char>>) -> usize {
        let mut sum = 0;
        for r in rocks.iter() {
            for (i, c) in r.iter().enumerate() {
                if *c == 'O' {
                    sum += i + 1;
                }
            }
        }
        sum
    }

    fn _print_vec(v: &Vec<Vec<char>>) {
        println!("======================");
        for r in v.iter() {
            println!("{:?}", r)
        }
    }
    fn _tilt_north_and_count(&self) -> usize {
        let rotated = Platform::rotate(&self.rocks); // Now we can work on rows only
        let mut weight = 0;

        for row in rotated.iter() {
            let mut current = row.clone();
            // println!("Before: {:?}", current);
            let mut moved = true;
            while moved {
                moved = false;
                for i in 1..current.len() {
                    // println!("LOOKING AT {} => {}", i, current[i]);
                    if current[i] == 'O' {
                        // println!("FOUND A ROCK AT {}", i);
                        let prev_i = (i as isize - 1) as usize;
                        if current[prev_i] == '.' {
                            // println!("MOVING {} to {}", i, prev_i);
                            current[prev_i] = 'O';
                            current[i] = '.';
                            moved = true;
                        }
                    }
                }
            }
            for (i, c) in current.iter().enumerate() {
                if *c == 'O' {
                    weight += current.len() - i;
                }
            }
            // println!("After:  {:?}\n", current);
        }


        weight
    }

    fn rotate(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut rotated = vec![vec!['.'; v.len()]; v[0].len()];
        for r in 0..v.len() {
            for c in 0..v[0].len() {
                let rr = v.len().checked_sub(r).map(|u| u.checked_sub(1)).flatten().unwrap();
                rotated[c][rr] = v[r][c];
            }
        }
        rotated
    }
}

fn part_a(input: &str) -> usize {
    let platform = Platform::parse(input);
    platform.cycle_and_count(0)
}

fn part_b(input: &str) -> usize {
    let platform = Platform::parse(input);
    platform.cycle_and_count(1_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day14();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day14/input.txt";
        assert_eq!(107053, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day14/input.txt";
        assert_eq!(88371, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day14/test-input.txt";
        let result = part_a(input);
        assert_eq!(136, result);
    }

    #[test]
    fn test_rotate() {
        let rocks_0 = vec![
            "..#.",
            "#...",
        ].iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
        let rocks_1 = vec![
            "#.",
            "..",
            ".#",
            "..",
        ].iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
        let rocks_2 = vec![
            "...#",
            ".#..",
        ].iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
        let rocks_3 = vec![
            "..",
            "#.",
            "..",
            ".#",
        ].iter().map(|s| s.chars().collect::<Vec<char>>()).collect();

        assert_eq!(rocks_1, Platform::rotate(&rocks_0));
        assert_eq!(rocks_2, Platform::rotate(&rocks_1));
        assert_eq!(rocks_3, Platform::rotate(&rocks_2));
        assert_eq!(rocks_0, Platform::rotate(&rocks_3));
    }

    #[test]
    fn test_cycling() {
        let input = "src/day14/test-input.txt";
        let platform = Platform::parse(input);
        let result = platform.cycle_and_count(1000);
        assert_eq!(64, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day14/test-input.txt";
        let result = part_b(input);
        assert_eq!(64, result);
    }
}