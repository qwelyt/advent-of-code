use crate::util::time;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 4 ==");
    let input = "src/day4/input.txt";
    let run2 = false;
    time(part_a, input, "A");
    if run2 {
        time(part_a2, input, "A2");
    }
    time(part_b, input, "B");
    if run2 {
        time(part_b2, input, "B2");
    }
}

fn part_a(input: &str) -> i32 {
    let mut sum = 0;
    let grid = grid(input);
    let dy = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '@' {
                continue;
            }
            let mut count = 0;
            for d in 0..dy.len() {
                if y as isize + dy[d] < 0 {
                    continue;
                }
                if y as isize + dy[d] >= grid.len() as isize {
                    continue;
                }
                if x as isize + dx[d] < 0 {
                    continue;
                }
                if x as isize + dx[d] >= grid[y].len() as isize {
                    continue;
                }

                let yy = (y as isize + dy[d]) as usize;
                let xx = (x as isize + dx[d]) as usize;

                if grid[yy][xx] == '@' {
                    count += 1;
                }
                if count > 3 {
                    break;
                }
            }
            if count < 4 {
                sum += 1;
            }
        }
    }
    sum
}

fn part_a2(input: &str) -> usize {
    let map = map(input);
    let mut sum = 0;
    let dy = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
    for (k, v) in map.iter() {
        if *v != '@' {
            continue;
        }
        let mut count = 0;
        for d in 0..dy.len() {
            let yy = k.0 + dy[d];
            let xx = k.1 + dx[d];
            if map.contains_key(&(yy, xx)) {
                if map.get(&(yy, xx)).unwrap() == &'@' {
                    count += 1;
                }
                if count > 3 {
                    break;
                }
            }
        }
        if count < 4 {
            sum += 1;
        }
    }
    sum
}

fn part_b(input: &str) -> usize {
    let mut grid = PaperGrid::parse(input);
    let mut prev = 0;
    loop {
        grid.run_removal();
        if prev == grid.removed {
            break;
        }
        prev = grid.removed;
    }
    grid.removed
}
fn part_b2(input: &str) -> usize {
    let mut grid = PaperMap::parse(input);
    let mut prev = 0;
    loop {
        grid.run_removal();
        if prev == grid.removed {
            break;
        }
        prev = grid.removed;
    }
    grid.removed
}

fn grid(input: &str) -> Vec<Vec<char>> {
    File::open(input)
        .map(|f| BufReader::new(f))
        .map(|b| {
            b.lines()
                .flatten()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .unwrap()
}

fn map(input: &str) -> HashMap<(i32, i32), char> {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in BufReader::new(File::open(input).expect("Could not read file"))
        .lines()
        .flatten()
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            map.insert((y as i32, x as i32), c);
        }
    }
    map
}

struct PaperGrid {
    grid: Vec<Vec<char>>,
    removed: usize,
}

impl PaperGrid {
    fn parse(input: &str) -> PaperGrid {
        let grid = grid(input);
        PaperGrid { grid, removed: 0 }
    }

    fn run_removal(&mut self) {
        let dy = [-1, -1, -1, 0, 0, 1, 1, 1];
        let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
        let mut removable: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] != '@' {
                    continue;
                }
                let mut count = 0;
                for d in 0..dy.len() {
                    if y as isize + dy[d] < 0 {
                        continue;
                    }
                    if y as isize + dy[d] >= self.grid.len() as isize {
                        continue;
                    }
                    if x as isize + dx[d] < 0 {
                        continue;
                    }
                    if x as isize + dx[d] >= self.grid[y].len() as isize {
                        continue;
                    }

                    let yy = (y as isize + dy[d]) as usize;
                    let xx = (x as isize + dx[d]) as usize;

                    if self.grid[yy][xx] == '@' {
                        count += 1;
                    }
                    if count > 3 {
                        break;
                    }
                }
                if count < 4 {
                    removable.push((y, x));
                }
            }
        }
        self.remove(removable)
    }

    fn remove(&mut self, to_be_removed: Vec<(usize, usize)>) {
        for v in to_be_removed.iter() {
            self.grid[v.0][v.1] = '.';
        }
        self.removed += to_be_removed.len();
    }
}

struct PaperMap {
    map: HashMap<(i32, i32), char>,
    removed: usize,
}
impl PaperMap {
    fn parse(input: &str) -> PaperMap {
        let map = map(input);
        PaperMap { map, removed: 0 }
    }
    fn run_removal(&mut self) {
        let dy = [-1, -1, -1, 0, 0, 1, 1, 1];
        let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
        let mut removable: Vec<(i32, i32)> = Vec::new();
        for (k, v) in self.map.iter() {
            if *v != '@' {
                continue;
            }
            let mut count = 0;
            for d in 0..dy.len() {
                let yy = k.0 + dy[d];
                let xx = k.1 + dx[d];
                if self.map.contains_key(&(yy, xx)) {
                    if self.map.get(&(yy, xx)).unwrap() == &'@' {
                        count += 1;
                    }
                    if count > 3 {
                        break;
                    }
                }
            }
            if count < 4 {
                removable.push(*k);
            }
        }
        self.remove(removable);
    }
    fn remove(&mut self, to_be_removed: Vec<(i32, i32)>) {
        for v in to_be_removed.iter() {
            self.map.insert(*v, '.');
        }
        self.removed += to_be_removed.len();
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
        let input = "src/day4/input.txt";
        assert_eq!(1346, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day4/input.txt";
        assert_eq!(8493, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_a(input);
        assert_eq!(13, result);
    }
    #[test]
    fn part_a2_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_a2(input);
        assert_eq!(13, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_b(input);
        assert_eq!(43, result);
    }
    #[test]
    fn part_b2_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_b2(input);
        assert_eq!(43, result);
    }
}
