use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day17() {
    println!("== Day 17 ==");
    let input = "src/day17/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> u32 {
    File::open(input)
        .map(|f| BufReader::new(f).lines().flatten()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>()
        )
        .map(|v| dijkstra(&v, (0, 0), (v.len().saturating_sub(1), v[0].len().saturating_sub(1)), 0, 3))
        .map(Option::unwrap)
        .unwrap()
}

fn part_b(input: &str) -> u32 {
    File::open(input)
        .map(|f| BufReader::new(f).lines().flatten()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>()
        )
        .map(|v| dijkstra(&v, (0, 0), (v.len().saturating_sub(1), v[0].len().saturating_sub(1)), 4, 10))
        .map(Option::unwrap)
        .unwrap()
}

type Coord = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    pos: Coord,
    dir: usize,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Key {
    pos: Coord,
    dir: usize,
    steps: usize,
}

impl From<State> for Key {
    fn from(state: State) -> Self {
        Self {
            pos: state.pos,
            dir: state.dir,
            steps: state.steps,
        }
    }
}

fn dijkstra(grid: &Vec<Vec<u32>>, start: Coord, end: Coord, min_step: usize, max_step: usize) -> Option<u32> {
    let mut dist: HashMap<Key, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    for dir in 0..4 {
        heap.push(State {
            cost: 0,
            pos: start,
            dir,
            steps: 0,
        });
    }

    while let Some(current) = heap.pop() {
        if current.pos == end && current.steps >= min_step {
            return Some(current.cost);
        }

        if *dist.get(&current.into()).unwrap_or(&u32::MAX) < current.cost {
            continue;
        }

        for (dir, new_pos) in next_tile(current.pos, current.dir, grid).iter() {
            let next = State {
                cost: current.cost + grid[new_pos.0][new_pos.1],
                pos: *new_pos,
                dir: *dir,
                steps: if *dir == current.dir { current.steps + 1 } else { 1 },
            };
            // println!("Next: {:?}", next);
            if *dist.get(&next.into()).unwrap_or(&u32::MAX) <= next.cost
                || next.steps > max_step {
                continue;
            }
            if next.dir != current.dir && current.steps < min_step {
                continue;
            }

            heap.push(next);
            *dist.entry(next.into()).or_default() = next.cost;
        }
    }
    None
}

fn next_tile(from: Coord, dir: usize, grid: &Vec<Vec<u32>>) -> Vec<(usize, Coord)> {
    let deltas = [
        (0, (from.0.saturating_sub(1), from.1)),
        (1, (from.0, from.1.saturating_sub(1))),
        (2, (from.0 + 1, from.1)),
        (3, (from.0, from.1 + 1)),
    ];
    deltas.into_iter()
        .filter_map(|(d, c)| {
            if (c.0, c.1) == (from.0, from.1)
                || c.0 >= grid.len()
                || c.1 == grid[0].len()
                || ((d + 2) % 4) == dir {
                None
            } else {
                Some((d, c))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day17();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day17/input.txt";
        assert_eq!(928, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day17/input.txt";
        assert_eq!(1104, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day17/test-input.txt";
        let result = part_a(input);
        assert_eq!(102, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day17/test-input.txt";
        let result = part_b(input);
        assert_eq!(94, result);
    }
}