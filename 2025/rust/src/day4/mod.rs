use crate::util::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 4 ==");
    let input = "src/day4/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
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
    fn part_b_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_b(input);
        assert_eq!(43, result);
    }
}
