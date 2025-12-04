use crate::util::time;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

pub fn solve() {
    println!("== Day 6 ==");
    let input = "src/day6/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let (start, map) = parse_map(input);
    // for l in map.iter() {
    //     println!("{:?}", l);
    // }
    // println!("{:?}", position);

    traverse(start.unwrap(), &map).len()
}
fn part_b(input: &str) -> usize {
    let (start, map) = parse_map(input);
    let possible_block_locations = traverse(start.unwrap(), &map);
    possible_block_locations
        .iter()
        .filter(|p| **p != start.unwrap())
        .filter(|&loc| traverse_with_block(loc, &start.unwrap(), &map))
        .count()
}

fn parse_map(input: &str) -> (Option<Position>, Vec<Vec<char>>) {
    let mut start = None;
    let map = File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .enumerate()
                .map(|(p, s)| {
                    let vec = s.chars().collect::<Vec<char>>();
                    if let Some(pos) = vec.iter().position(|&c| c == '^') {
                        start = Some(Position::new(p as i32, pos as i32));
                    }
                    vec
                })
                .collect::<Vec<Vec<char>>>()
        })
        .unwrap();
    (start, map)
}

fn traverse(start: Position, map: &Vec<Vec<char>>) -> HashSet<Position> {
    let mut position = start;
    let mut visited: HashSet<Position> = HashSet::new();
    let mut dir = Position::new(-1, 0);
    loop {
        visited.insert(position);
        let go_to = position + dir;
        if go_to.x < 0
            || go_to.y < 0
            || go_to.x >= map.len() as i32
            || go_to.y >= map[0].len() as i32
        {
            break;
        }
        if map[go_to.y as usize][go_to.x as usize] == '#' {
            dir = turn_right(&dir);
        }
        position = position + dir;
        // _print(&map, &position);
    }
    visited
}
fn traverse_with_block(block: &Position, start: &Position, map: &Vec<Vec<char>>) -> bool {
    let mut dir = Position::new(-1, 0);
    let mut position = *start;
    let mut visited: HashSet<(Position, Position)> = HashSet::new();
    loop {
        visited.insert((position, dir));
        let go_to = position + dir;
        if go_to.x < 0
            || go_to.y < 0
            || go_to.x >= map.len() as i32
            || go_to.y >= map[0].len() as i32
        {
            return false;
        }
        if map[go_to.y as usize][go_to.x as usize] == '#'  || go_to == *block {
            dir = turn_right(&dir);
        } else {
            position = position + dir;
        }
        if visited.contains(&(position, dir)) {
            return true;
        }
    }
}

fn turn_right(dir: &Position) -> Position {
    Position::new(dir.x, dir.y * -1)
}

fn _print(map: &[Vec<char>], position: &Position) {
    println!();
    println!("{:?}", position);
    for (r, row) in map.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if r == position.y as usize && c == position.x as usize {
                print!("&");
            } else {
                if *ch == '#' {
                    print!("{}", ch);
                } else {
                    print!(".");
                }
            }
        }
        println!()
    }
    println!();
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    y: i32,
    x: i32,
}
impl Position {
    fn new(y: i32, x: i32) -> Position {
        Self { y, x }
    }
}
impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position::new(self.y + rhs.y, self.x + rhs.x)
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
        let input = "src/day6/input.txt";
        assert_eq!(5177, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day6/input.txt";
        assert_eq!(1686, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day6/test-input.txt";
        let result = part_a(input);
        assert_eq!(41, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day6/test-input.txt";
        let result = part_b(input);
        assert_eq!(6, result);
    }
}
