use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::day10::Connector::{EastWest, Invalid, NorthEast, NorthSouth, NorthWest, SouthEast, SouthWest};
use crate::day10::Direction::{East, North, South, West};
use crate::util::time;

pub fn day10() {
    println!("== Day 10 ==");
    let input = "src/day10/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction { North, West, South, East }

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Connector {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Invalid,
}

impl Display for Connector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NorthSouth => write!(f, "┃")?,
            EastWest => write!(f, "━")?,
            NorthEast => write!(f, "┗")?,
            NorthWest => write!(f, "┛")?,
            SouthWest => write!(f, "┓")?,
            SouthEast => write!(f, "┏")?,
            Invalid => write!(f, ".")?,
        }
        Ok(())
    }
}

impl Connector {
    fn from(c: &char) -> Connector {
        match *c {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            _ => Invalid
        }
    }
    fn from_direction(a: &Direction, b: &Direction) -> Self {
        match (a, b) {
            (North, South) | (South, North) => NorthSouth,
            (East, West) | (West, East) => EastWest,
            (North, East) | (East, North) => NorthEast,
            (North, West) | (West, North) => NorthWest,
            (South, East) | (East, South) => SouthEast,
            (South, West) | (West, South) => SouthWest,
            (_, _) => Invalid
        }
    }
    fn connections(&self) -> Option<Vec<Direction>> {
        match self {
            NorthSouth => Some(vec![North, South]),
            EastWest => Some(vec![East, West]),
            NorthEast => Some(vec![North, East]),
            NorthWest => Some(vec![North, West]),
            SouthWest => Some(vec![South, West]),
            SouthEast => Some(vec![South, East]),
            Invalid => None
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    y: i64,
    x: i64,
}

impl Coord {
    fn from(y: i64, x: i64) -> Self {
        Coord { y, x }
    }
    fn add(&self, dy: i64, dx: i64) -> Coord {
        Coord::from(self.y + dy, self.x + dx)
    }
    fn go(&self, dir: &Direction) -> Coord {
        match dir {
            North => self.add(-1, 0),
            West => self.add(0, -1),
            South => self.add(1, 0),
            East => self.add(0, 1),
        }
    }
}

struct Pipes {
    grid: Vec<Vec<Connector>>,
    start: Coord,
    size: (usize, usize),
}

impl Pipes {
    fn from(input: &str) -> Self {
        let file = File::open(input).unwrap();
        // let grid = BufReader::new(file)
        //     .lines()
        //     .flatten()
        //     .map(|l| l.chars().collect::<Vec<char>>())
        //     .collect::<Vec<Vec<char>>>();
        let mut grid = Vec::new();
        let mut start = None;
        for (y, line) in BufReader::new(file).lines().enumerate() {
            let line = line.unwrap();
            let row = line.chars().collect::<Vec<char>>();
            let option = row.iter().position(|&c| c == 'S');
            if option.is_some() {
                start = Some(Coord::from(y as i64, option.unwrap() as i64));
            }
            grid.push(row.iter().map(Connector::from).collect::<Vec<Connector>>())
        }
        let start = start.unwrap();

        let size = (grid.len(), grid[0].len());
        let orig = Self {
            grid,
            start,
            size,
        };
        let real_grid = Pipes::replace_start(&orig);


        real_grid
    }

    fn replace_start(orig: &Pipes) -> Pipes {
        let dy = [-1, 0, 1, 0];
        let dx = [0, -1, 0, 1];
        let mut neighbours = Vec::new();
        for i in 0..dy.len() {
            let m: Coord = orig.start.add(dy[i], dx[i]);
            // println!("{:?} -> {:?} || ({}, {})", orig.start, m, dy[i], dx[i]);
            if orig.is_inside(&m) {
                let connector = orig.grid[m.y as usize][m.x as usize];
                neighbours.push(connector);
            } else {
                neighbours.push(Invalid)
            }
        }
        // println!("{:?}", neighbours);
        let mut required_mid = Vec::new();
        for i in 0..neighbours.len() {
            if neighbours[i].connections().is_some() {
                let connections = neighbours[i].connections().unwrap();
                match i {
                    0 => if connections.contains(&South) { required_mid.push(North); },// Top
                    1 => if connections.contains(&East) { required_mid.push(West); },// Left
                    2 => if connections.contains(&North) { required_mid.push(South); } // Bottom
                    3 => if connections.contains(&West) { required_mid.push(East); } // Right
                    _ => {}
                }
            }
        }
        // println!("{:?}", required_mid);
        if required_mid.len() != 2 {
            panic!("Mid requires more than 2 connections!")
        }
        let mid = Connector::from_direction(&required_mid[0], &required_mid[1]);
        // println!("Mid: {:?}", mid);
        let mut replaced = orig.grid.clone();
        replaced[orig.start.y as usize][orig.start.x as usize] = mid;
        Pipes {
            grid: replaced,
            start: orig.start.clone(),
            size: orig.size,
        }
    }

    fn is_inside(&self, coord: &Coord) -> bool {
        if coord.y < 0 || coord.y > self.size.0 as i64 {
            return false;
        }
        if coord.x < 0 || coord.x > self.size.1 as i64 {
            return false;
        }

        true
    }

    fn connector_at(&self, coord: &Coord) -> Connector {
        if self.is_inside(coord) {
            return self.grid[coord.y as usize][coord.x as usize];
        }
        Invalid
    }

    fn find_furthest_point(&self) -> (Coord, usize) {
        let connector = self.connector_at(&self.start);
        let mut steps = Vec::new();
        for direction in connector.connections().unwrap() {
            let current = self.start;
            let next = current.go(&direction);
            // println!("START WALKING from {:?} in {:?}", current, direction);
            let map: HashMap<Coord, usize> = self.walk(self.start, next, current);
            steps.push(map);
        }
        let mut step_to: HashMap<Coord, Vec<usize>> = HashMap::new();
        for m in steps.iter() {
            for (k, v) in m.iter() {
                // println!("Should add {} to {:?}", v, k);
                step_to.entry(*k).or_default().push(*v);
            }
        }
        // println!("{:?}", step_to.len());

        let mut max = (self.start, 0);
        // println!("{:?}", step_to);
        // println!("{:?}", max);
        for (i, n) in step_to.iter() {
            let min_steps_to = *n.iter().min().unwrap_or(&usize::MAX);
            if min_steps_to > max.1 {
                max = (*i, min_steps_to);
            }
        }
        max
    }

    fn walk(&self, end: Coord, current: Coord, prev: Coord) -> HashMap<Coord, usize> {
        let mut steps = 0;
        let mut map = HashMap::new();
        let mut path = Vec::new();
        path.push(prev);
        path.push(current);
        while path.last().unwrap() != &end {
            steps += 1;
            // println!("Path: {:?}", path);
            // println!("Steps: {}", steps);
            map.insert(*path.last().unwrap(), steps);
            let cur = path.last().unwrap();
            let vec = self.connector_at(cur)
                .connections()
                .unwrap()
                .iter()
                .map(|d| cur.go(d))
                .filter(|c| c != path.get(path.len() - 2).unwrap())
                .collect::<Vec<Coord>>();
            // println!("Next: {:?}", vec);
            path.push(*vec.first().unwrap());
            // println!("Path: {:?}", path);
        }
        map
    }

    fn tiles_inside_loop(&self) -> usize {
        let connector = self.connector_at(&self.start);
        let dir = *connector.connections().unwrap().first().unwrap();
        let steps_to = self.walk(self.start, self.start.go(&dir), self.start);
        let path = steps_to.keys().collect::<Vec<&Coord>>();

        let mut sum = 0;
        let mut inside = false;
        let wall_kinds = vec![
            Connector::from(&'|'),
            Connector::from(&'J'),
            Connector::from(&'L'),
        ];
        for y in 0..self.size.0 {
            let mut walls = path.iter()
                .filter(|c| c.y as usize == y)
                .map(|&&c| c.x)
                .collect::<Vec<i64>>();
            walls.sort();
            for x in 0..self.size.1 {
                let connector = self.grid[y][x];
                let is_wall = walls.contains(&(x as i64));
                if is_wall && wall_kinds.contains(&connector) {
                    inside = !inside;
                    // print!("|");
                } else if inside && !is_wall {
                    // print!("#");
                    sum += 1;
                } else {
                    // print!(".");
                }
            }
            // println!();
        }
        sum
    }
}

impl Debug for Pipes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let start = self.start;
        writeln!(f, "Start at {:?}", start)?;
        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                if Coord::from(y as i64, x as i64) == self.start {
                    write!(f, "#")?;
                } else {
                    let c = self.grid[y][x];
                    write!(f, "{c}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_a(input: &str) -> usize {
    let pipes = Pipes::from(input);
    // println!("{:?}", pipes);

    pipes.find_furthest_point().1
}

fn part_b(input: &str) -> usize {
    let pipes = Pipes::from(input);
    // println!("{:?}", pipes);
    pipes.tiles_inside_loop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day10();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day10/input.txt";
        assert_eq!(6860, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day10/input.txt";
        assert_eq!(343, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day10/test-input.txt";
        let result = part_a(input);
        assert_eq!(8, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day10/test-input-b.txt";
        let result = part_b(input);
        assert_eq!(10, result);
    }
}