use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::util::time;

pub fn solve() {
    println!("== Day 18 ==");
    let input = "src/day18/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> u32 {
    p1(input, 71,71, 1024)
}

fn part_b(input: &str) -> String {
    let (x,y) = p2(input, 71, 71);
    format!("{},{}", x, y)
}

fn p1(input: &str, rows: usize, cols: usize, corruptions: usize) -> u32 {
    let mut memory = Memory::new(input, (rows, cols));
    // memory._print();
    memory.corrupt(corruptions);
    // memory._print();
    memory.find_exit().unwrap()
}
fn p2(input: &str, rows: usize, cols: usize) -> (usize, usize) {
    let memory = Memory::new(input, (rows, cols));
    // Brute-force
    // for i in 0..memory.corruptions.len() {
    //     memory.corrupt(1);
    //     // memory._print();
    //     if memory.find_exit().is_none() {
    //         return memory.corruptions[i];
    //     }
    // }
    // Binary search
    let mut low = 0;
    let mut high = memory.corruptions.len()-1;
    while low < high {
        let mid = (low + high) / 2;
        let mut m = memory.clone();
        m.corrupt(mid+1);
        if m.find_exit().is_none() {
            high = mid;
        } else {
            low = mid+1;
        }
    }
    memory.corruptions[low]
}

#[derive(Debug, Clone)]
struct Memory {
    grid: Vec<Vec<bool>>,
    corruptions: Vec<(usize, usize)>,
    corruption_index: usize,
}
impl Memory {
    fn new(input: &str, size: (usize,usize)) -> Self {
        let corruptions = File::open(input)
            .map(|f| BufReader::new(f)
                .lines()
                .flatten()
                .map(|line| {
                    let (a,b) = line.split_once(',').unwrap();
                    (a.parse::<usize  >().unwrap(), b.parse::<usize>().unwrap())
                })
                .collect::<Vec<(usize, usize)>>()
        ).unwrap();
        Self {
            grid: vec![vec![true; size.1]; size.0],
            corruptions,
            corruption_index: 0
        }
    }

    fn corrupt(&mut self, corruptions: usize) {
        for i in self.corruption_index..self.corruption_index+corruptions {
            let coord = self.corruptions[i];
            self.grid[coord.1][coord.0] = false;
        }
        self.corruption_index += corruptions;
    }
    fn find_exit(&self) -> Option<u32> {
        let start = (0,0);
        let end = (self.grid.len() - 1, self.grid.len() - 1);

        // dijkstra
        let mut dist: HashMap<Coord, u32> = HashMap::new();
        let mut heap : BinaryHeap<State>= BinaryHeap::new();
        heap.push(State::of(start, 0));

        while let Some(current) = heap.pop() {
            if current.pos == end {
                return Some(current.cost);
            }

            if *dist.get(&current.pos).unwrap_or(&u32::MAX) < current.cost {
                continue;
            }

            for new_pos in self.next_tile(current.pos).iter() {
                let next = State {
                    cost: current.cost + 1,
                    pos: *new_pos,
                };
                if *dist.get(&next.pos).unwrap_or(&u32::MAX) <= next.cost {
                    continue;
                }

                heap.push(next);
                *dist.entry(next.pos).or_default() = next.cost;
            }
        }
        None
    }
    fn next_tile(&self, pos: Coord) -> Vec<Coord> {
        let dirs = [(0,1), (0,-1), (1,0), (-1,0)];
        let mut v = Vec::new();
        for d in dirs.iter() {
            let x = pos.0 as isize + d.0;
            let y = pos.1 as isize + d.1;
            if x < 0 || x as usize >= self.grid[0].len()
                || y < 0 || y as usize >= self.grid.len() {
                continue;
            }
            if self.grid[y as usize][x as usize] {
                v.push((x as usize, y as usize));
            }
        }
        v
    }

    fn _print(&self) {
        for row in self.grid.iter() {
            for val in row.iter() {
                if *val {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!()
        }
        println!();
        for corruption in self.corruptions.iter() {
            print!("{},{}  ", corruption.0, corruption.1);
        }
        println!();
        for _ in 0..self.corruption_index {
            print!("     ")
        }
        print!("^");
        println!();

    }
}

type Coord = (usize, usize);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    pos: Coord,
}

impl State {
    fn of(pos: Coord, cost: u32) -> State {
        Self { cost, pos }
    }
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
        let input = "src/day18/input.txt";
        assert_eq!(250, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day18/input.txt";
        assert_eq!("56,8", part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day18/test-input.txt";
        let result = p1(input, 7,7, 12);
        assert_eq!(22, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day18/test-input.txt";
        let result = p2(input, 7,7);
        assert_eq!((6,1), result);
    }
}