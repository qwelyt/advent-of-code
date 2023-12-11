use std::cmp::{max, min, Ordering};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day11() {
    println!("== Day 11 ==");
    let input = "src/day11/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    y: i64,
    x: i64,
}

impl Coord {
    fn of(y: i64, x: i64) -> Self {
        Coord { y, x }
    }
    fn add(&self, dy: i64, dx: i64) -> Coord {
        Coord::of(self.y + dy, self.x + dx)
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y)
            .then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Space {
    map: Vec<Vec<char>>,
}


impl Space {
    fn of(map: Vec<Vec<char>>) -> Self {
        Self { map }
    }
    fn parse(input: &str) -> Self {
        let file = File::open(input).unwrap();
        let orig = BufReader::new(file)
            .lines()
            .flatten()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self::of(orig)
    }
    fn expand(orig: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut expanded = Vec::new();
        let mut populated_col = HashSet::new();
        let mut expanded_rows = Vec::new();
        for (_y, line) in orig.iter().enumerate() {
            let mut empty_row = true;
            for (x, point) in line.iter().enumerate() {
                match *point {
                    '.' => {}
                    '#' => {
                        empty_row = false;
                        populated_col.insert(x);
                    }
                    _ => panic!("Whats this!? {:?}", point)
                }
            }
            expanded.push(line.clone());
            if empty_row {
                expanded_rows.push(_y);
                expanded.push(line.clone());
            }
        }
        let mut extra_col = 0;
        let mut cols = Vec::new();
        for c in 0..orig[0].len() {
            if !populated_col.contains(&c) {
                cols.push(c);
            }
        }
        for &k in cols.iter() {
            for line in expanded.iter_mut() {
                line.insert(k + extra_col, '.');
            }
            extra_col += 1;
        }
        // println!("Expanded \nRows: {:?} \ncols: {:?}", expanded_rows, cols);
        expanded
    }
    fn expand_with_marker(orig: &Vec<Vec<char>>, marker: char) -> Vec<Vec<char>> {
        let mut expanded = Vec::new();
        for row in orig.iter() {
            if row.iter().all(|&c| c == '.') { // Blank row, so mark it for expansion
                expanded.push(vec![marker; row.len()]);
            } else {
                expanded.push(row.clone());
            }
        }

        let rotated: Vec<Vec<char>> = Space::rotate(&expanded);
        expanded.clear();
        for col in rotated.iter() {
            if col.iter().all(|&c| c == '.' || c == marker) {
                expanded.push(vec![marker; col.len()]);
            } else {
                expanded.push(col.clone());
            }
        }
        let rotated: Vec<Vec<char>> = Space::rotate(&expanded);
        rotated
    }
    fn rotate(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut rotated = vec![vec!['.'; map.len()]; map[0].len()];
        for (y, row) in map.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                rotated[x][y] = col;
            }
        }
        rotated
    }

    fn galaxy_positions(&self) -> HashSet<Coord> {
        let mut set = HashSet::new();
        for (y, line) in self.map.iter().enumerate() {
            for (x, point) in line.iter().enumerate() {
                if *point == '#' {
                    set.insert(Coord::of(y as i64, x as i64));
                }
            }
        }

        set
    }

    fn pairs(points: &HashSet<Coord>) -> HashSet<(Coord, Coord)> {
        let mut set = HashSet::new();
        for &p1 in points.iter() {
            for &p2 in points.iter() {
                if p2.eq(&p1) {
                    continue;
                }
                let pair1 = (p1, p2);
                let pair2 = (p2, p1);
                if !set.contains(&pair1) && !set.contains(&pair2) {
                    set.insert(pair1);
                }
            }
        }
        set
    }


    fn path_cost(from: &Coord, to: &Coord) -> usize {
        ((to.x - from.x).abs() + (to.y - from.y).abs()) as usize
    }
    fn path_cost_marker(&self, from: &Coord, to: &Coord, marker: char, expand_amount: usize) -> usize {
        // println!("Going: {:?} -> {:?}", from, to);
        let mut steps = 0;
        let mut cur = *min(from, to);
        let end = *max(from, to);
        // println!("     {:?} -> {:?}", cur, end);
        while cur != end {
            while cur.y != end.y {
                if self.map[cur.y as usize][cur.x as usize] == marker {
                    steps += expand_amount;
                } else {
                    steps += 1;
                }
                cur = cur.add(1, 0);
            }
            while cur.x != end.x {
                if self.map[cur.y as usize][cur.x as usize] == marker {
                    steps += expand_amount;
                } else {
                    steps += 1;
                }
                let dx = if cur.x < end.x {
                    1
                } else {
                    -1
                };
                cur = cur.add(0, dx);
            }
        }
        steps
    }

    fn sum_all_paths(&self) -> usize {
        let points = self.galaxy_positions();
        // println!("Points ({}): {:?}", points.len(), points);
        let pairs = Space::pairs(&points);
        // println!("Pairs ({}): {:?}",pairs.len(), pairs);
        pairs.iter()
            .map(|&p| Space::path_cost(&p.0, &p.1))
            .sum()
    }

    fn sum_all_paths_with_marker(&self, marker: char, expand_amount: usize) -> usize {
        let points = self.galaxy_positions();
        let pairs = Space::pairs(&points);
        pairs.iter()
            .map(|&p| self.path_cost_marker(&p.0, &p.1, marker, expand_amount))
            .sum()
    }

    fn _print(map: &Vec<Vec<char>>) {
        let y = map.len();
        let x = map[0].len();
        println!("Size: ({}, {})", y, x);
        for line in map.iter() {
            println!("{:?}", line);
        }
    }
}

fn part_a(input: &str) -> usize {
    let space = Space::parse(input);
    let expanded = Space::of(Space::expand(&space.map));
    expanded.sum_all_paths()
}

fn part_b(input: &str) -> usize {
    part_b_solve(input, 1_000_000)
}

fn part_b_solve(input: &str, expand_amount: usize) -> usize {
    let space = Space::parse(input);
    // Space::print(&space.map);
    let expanded = Space::of(Space::expand_with_marker(&space.map, 'x'));
    // Space::print(&expanded.map);
    expanded.sum_all_paths_with_marker('x', expand_amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day11();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day11/input.txt";
        assert_eq!(9556896, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day11/input.txt";
        // assert_eq!(true, 68511186836< part_b(input));
        assert_eq!(685038186836, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day11/test-input.txt";
        let result = part_a(input);
        assert_eq!(374, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day11/test-input.txt";
        {
            let result = part_b_solve(input, 10);
            assert_eq!(1030, result);
        }
        {
            let result = part_b_solve(input, 100);
            assert_eq!(8410, result);
        }
    }
}