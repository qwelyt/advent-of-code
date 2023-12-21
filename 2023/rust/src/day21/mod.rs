use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day21() {
    println!("== Day 21 ==");
    let input = "src/day21/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    solve_a(input, 64)
}

fn part_b(input: &str) -> usize {
    solve_b(input, 26501365)
}

fn solve_a(input: &str, steps: u8) -> usize {
    let garden = Garden::parse(input);
    // println!("{:?}", garden);
    garden.possible_tiles(steps)
}

fn solve_b(input: &str, steps: u32) -> usize {
    let garden = Garden::parse(input);
    garden.possible_tiles_big(steps)
}

type Pos = (i32, i32);

fn add(a: Pos, b: Pos) -> Pos {
    let y = a.0 + b.0;
    let x = a.1 + b.1;
    (y, x).into()
}

#[derive(Debug, Clone)]
struct Garden {
    map: Vec<Vec<char>>,
    start: Pos,
    size: Pos,
}

impl Garden {
    fn parse(input: &str) -> Self {
        let mut map = File::open(input)
            .map(|f| BufReader::new(f).lines().flatten()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
            )
            .unwrap();

        let mut start_pos = None;
        for (r, l) in map.iter().enumerate() {
            for (c, ch) in l.iter().enumerate() {
                if ch == &'S' {
                    start_pos = Some((r, c));
                }
            }
        }
        let start_pos = start_pos.unwrap();
        map[start_pos.0][start_pos.1] = '.';

        let start = (start_pos.0 as i32, start_pos.1 as i32).into();
        let size = (map.len() as i32, map[0].len() as i32).into();
        Self { map, start, size }
    }

    fn walkable(&self, pos: &Pos) -> bool {
        let y = pos.0;
        let x = pos.1;
        return if y < 0
            || x < 0
            || y as usize >= self.map.len()
            || x as usize >= self.map[y as usize].len() {
            false
        } else if self.map[y as usize][x as usize] == '.' {
            true
        } else {
            false
        };
    }

    fn possible_tiles(&self, steps: u8) -> usize {
        let mut reachable = HashSet::new();

        let delta = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut queue = VecDeque::<Pos>::new();
        queue.push_back(self.start);
        reachable.insert(self.start);

        // self._print(&reachable);

        let mut new_starts = HashSet::new();
        for _i in 1..=steps {
            new_starts.into_iter().for_each(|p| queue.push_back(p));
            new_starts = HashSet::new();
            while let Some(start) = queue.pop_front() {
                reachable.insert(start);
                for d in delta.iter() {
                    let new_pos = add(start, *d);
                    // print!("Going from {:?} to {:?}", start, new_pos);
                    if self.walkable(&new_pos) {
                        // println!(" -> Works! {:?}", self.map[new_pos.0 as usize][new_pos.1 as usize]);
                        new_starts.insert(new_pos);
                    } else {
                        // println!(" -> NO! walkable {:?}, reached: {:?}", self.walkable(&new_pos), reachable.contains(&new_pos));
                    }
                }
            }
            // println!("{:?} :: NS size: {:?}, reached: {:?}", i, new_starts.len(), reachable.len());
            // println!("=== {}", i);
            // self._print(&new_starts);
            // println!("===");
            // self._print(&reachable);
            // println!();
        }
        // self._print(&reachable);
        // println!("===");
        // println!("===");
        // self._print(&new_starts);
        new_starts.len()
    }
    fn possible_tiles_big(&self, steps: u32) -> usize {
        let y_tiles_to_edge = (self.size.0 - self.start.0) as u32;
        let x_tiles_to_edge = (self.size.1 - self.start.1) as u32;
        assert_eq!(y_tiles_to_edge, x_tiles_to_edge);
        if steps <= y_tiles_to_edge {
            return self.possible_tiles(steps as u8);
        }
        println!("{:?}", y_tiles_to_edge);
        println!("{:?}", steps % y_tiles_to_edge);
        // self.possible_tiles()
        0
    }

    fn _print(&self, positions: &HashSet<Pos>) {
        println!("{:?}: {:?}", positions.len(), positions);
        for (y, l) in self.map.iter().enumerate() {
            let mut v = Vec::new();
            for (x, c) in l.iter().enumerate() {
                if positions.contains(&(y as i32, x as i32).into()) {
                    v.push('O');
                } else {
                    v.push(*c);
                }
            }
            println!("{:?}", v.iter().collect::<String>());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day21();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day21/input.txt";
        assert_eq!(3729, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day21/input.txt";
        assert_eq!(0, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day21/test-input.txt";
        let result = solve_a(input, 6);
        assert_eq!(16, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day21/test-input.txt";
        assert_eq!(16, solve_b(input, 6));
        assert_eq!(50, solve_b(input, 10));
        // assert_eq!(1594, solve_b(input, 50));
        // assert_eq!(6536, solve_b(input, 100));
        // assert_eq!(16733044, solve_b(input, 5000));
    }
}