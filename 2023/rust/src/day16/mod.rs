use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day16() {
    println!("== Day 16 ==");
    let input = "src/day16/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    File::open(input)
        .map(|f| BufReader::new(f).lines().flatten()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
        )
        .map(|v| shine(&v, ((0, 0), Direction::RIGHT)))
        .unwrap()
}

fn part_b(input: &str) -> usize {
    File::open(input)
        .map(|f| BufReader::new(f).lines().flatten()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
        )
        .map(|v| {
            let rows = v.len();
            let cols = v[0].len();
            let rights = (0..rows).map(|r| ((r, 0), Direction::RIGHT)).collect::<Vec<((usize, usize), Direction)>>();
            let lefts = (0..rows).map(|r| ((r, cols - 1), Direction::LEFT)).collect::<Vec<((usize, usize), Direction)>>();
            let downs = (0..cols).map(|c| ((0, c), Direction::DOWN)).collect::<Vec<((usize, usize), Direction)>>();
            let ups = (0..cols).map(|c| ((rows - 1, c), Direction::UP)).collect::<Vec<((usize, usize), Direction)>>();

            let mut dirs = Vec::new();
            dirs.extend(&rights);
            dirs.extend(&lefts);
            dirs.extend(&downs);
            dirs.extend(&ups);

            dirs.iter().map(|&d| shine(&v, d)).max().unwrap()
        })
        .unwrap()
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn turn(&self, c: char) -> Option<Direction> {
        match c {
            '/' => match self {
                Direction::UP => Some(Direction::RIGHT),
                Direction::DOWN => Some(Direction::LEFT),
                Direction::LEFT => Some(Direction::DOWN),
                Direction::RIGHT => Some(Direction::UP),
            },
            '\\' => match self {
                Direction::UP => Some(Direction::LEFT),
                Direction::DOWN => Some(Direction::RIGHT),
                Direction::LEFT => Some(Direction::UP),
                Direction::RIGHT => Some(Direction::DOWN),
            }
            _ => None
        }
    }

    fn split(&self, c: char) -> Option<(Direction, Direction)> {
        match c {
            '-' => match self {
                Direction::UP | Direction::DOWN => Some((Direction::LEFT, Direction::RIGHT)),
                _ => None,
            },
            '|' => match self {
                Direction::LEFT | Direction::RIGHT => Some((Direction::UP, Direction::DOWN)),
                _ => None
            }
            _ => None
        }
    }
}

fn shine(grid: &Vec<Vec<char>>, starting_pos: ((usize, usize), Direction)) -> usize {
    let mut visited: HashMap<(usize, usize), Vec<((usize, usize), Direction)>> = HashMap::new();
    let mut next_start = Vec::new();
    next_start.push(starting_pos);
    while let Some(start) = next_start.pop() {
        if visited.get(&start.0).unwrap_or(&Vec::new()).contains(&start) {
            continue;
        }

        let mut pos = start.0;
        let mut dir = start.1;
        loop {
            if visited.get(&pos).unwrap_or(&Vec::new()).contains(&(pos, dir)) {
                break;
            }
            visited.entry(pos).or_default().push((pos, dir));

            let tile = grid[pos.0][pos.1];
            if let Some(turn) = dir.turn(tile) {
                dir = turn;
            } else if let Some(split) = dir.split(tile) {
                dir = split.0;
                next_start.push((pos, split.1));
            } else {
                // Just keep walking
            }
            let move_t = match dir {
                Direction::UP => (-1, 0),
                Direction::DOWN => (1, 0),
                Direction::LEFT => (0, -1),
                Direction::RIGHT => (0, 1),
            };
            let np = (
                (pos.0 as isize + move_t.0),
                pos.1 as isize + move_t.1
            );
            if np.0 < 0 || np.0 >= grid.len() as isize
                || np.1 < 0 || np.1 >= grid[np.0 as usize].len() as isize {
                break;
            }
            pos = (np.0 as usize, np.1 as usize);
        }
    }

    visited.keys().count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day16();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day16/input.txt";
        assert_eq!(7236, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day16/input.txt";
        assert_eq!(7521, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day16/test-input.txt";
        let result = part_a(input);
        assert_eq!(46, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day16/test-input.txt";
        let result = part_b(input);
        assert_eq!(51, result);
    }
}