use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

use crate::util::time;

pub fn day15() {
    println!("== Day 15 ==");
    let input = "src/day15/input.txt";
    time(run_a, input, "A");
    time(run_b, input, "B");
}

fn run_a(input: &str) -> usize {
    part_a(input, 2_000_000)
}

fn run_b(input: &str) -> usize {
    part_b(input, 4_000_000, 4_000_000)
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Tile {
    Beacon,
    Sensor,
    Empty,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct GridCoord {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for GridCoord {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Sensor {
    position: GridCoord,
    closest_beacon: GridCoord,
    manhattan_distance: u32,
}

impl From<(GridCoord, GridCoord)> for Sensor {
    fn from((sensor, beacon): (GridCoord, GridCoord)) -> Self {
        let manhattan_dist = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
        Self {
            position: sensor,
            closest_beacon: beacon,
            manhattan_distance: manhattan_dist as u32,
        }
    }
}

impl Sensor {
    fn covers(self, coord: &GridCoord) -> bool {
        ((coord.x - self.position.x).abs() + (coord.y - self.position.y).abs()) as u32 <= self.manhattan_distance
    }
    fn cover_area(&self) -> Vec<GridCoord> {
        let mut coords: Vec<GridCoord> = Vec::new();
        let md = self.manhattan_distance as i32;
        for y in -md..=md {
            let sy = self.position.y + y;
            for x in -(md - y.abs())..=md - y.abs() {
                let sx = self.position.x + x;
                coords.push((sx, sy).into());
            }
        }
        coords
    }
    fn cover_area_at_y(&self, y: i32) -> Vec<GridCoord> {
        let mut coords: Vec<GridCoord> = Vec::new();
        let md = self.manhattan_distance as i32;
        if self.position.y - md <= y && self.position.y + md >= y {
            for my in -md..=md {
                let sy = self.position.y + my;
                if sy != y { continue; }
                let yabs = my.abs();
                for x in -md + yabs..=md - yabs {
                    let sx = self.position.x + x;
                    coords.push((sx, sy).into());
                }
            }
        }
        coords
    }
    fn outer_limit(&self) -> Vec<GridCoord> {
        let mut coords: Vec<GridCoord> = Vec::new();
        let md = self.manhattan_distance as i32;
        for y in (self.position.y - md - 1)..=(self.position.y + md + 1) {
            let remains = md - (self.position.y - y).abs() + 1;
            coords.push((self.position.x - remains, y).into());
            if remains > 0 {
                coords.push((self.position.x + remains, y).into());
            }
        }
        coords
    }
}

struct Cave {
    sensors: Vec<Sensor>,
    min_width: i32,
    min_height: i32,
    max_width: i32,
    max_height: i32,

}

impl Cave {
    fn parse(input: &str) -> Self {
        let mut sensors = Vec::new();
        let mut min_width = i32::MAX;
        let mut min_height = i32::MAX;
        let mut max_width = i32::MIN;
        let mut max_height = i32::MIN;

        for line in input.lines() {
            let split = line.split(" ").collect::<Vec<&str>>();
            let sensor_x = split[2].split("=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            let sensor_y = split[3].split("=").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            let beacon_x = split[8].split("=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            let beacon_y = split[9].split("=").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            // print!("Sensor at {},{}  ", sensor_x,sensor_y);
            // println!("beacon at {},{}", beacon_x, beacon_y);
            let sensor = GridCoord { x: sensor_x, y: sensor_y };
            let beacon = GridCoord { x: beacon_x, y: beacon_y };

            sensors.push((sensor, beacon).into());

            max_height = max(max_height, max(sensor_y, beacon_y));
            max_width = max(max_width, max(sensor_x, beacon_x));
            min_height = min(min_height, min(sensor_y, beacon_y));
            min_width = min(min_width, min(sensor_x, beacon_x));
        }

        // println!("Grid size: {},{} -> {},{}", min_width, min_height, max_width, max_height);

        Self {
            sensors,
            min_width,
            min_height,
            max_width,
            max_height,
        }
    }

    fn tile(&self, coord: &GridCoord) -> &Tile {
        for sensor in self.sensors.iter() {
            if sensor.position == *coord {
                return &Tile::Sensor;
            }
            if sensor.closest_beacon == *coord {
                return &Tile::Beacon;
            }
        }
        &Tile::Empty
    }

    fn covered_in_row(&self, row: i32) -> usize {
        self.sensors.iter()
            .flat_map(|s| s.cover_area_at_y(row))
            .filter(|p| self.tile(p) == &Tile::Empty)
            .collect::<HashSet<GridCoord>>()
            .len()
    }

    fn find_distress_beacon(&self, min_limit: usize, max_limit: usize) -> GridCoord {
        let min_limit = min_limit as i32;
        let max_limit = max_limit as i32;
        let pos = self.sensors.iter()
            .flat_map(|s| s.outer_limit())
            .filter(|p| p.x >= min_limit && p.x <= max_limit)
            .filter(|p| p.y >= min_limit && p.y <= max_limit)
            .find(|p| !self.sensors.iter().any(|s| s.covers(p)));
        if pos.is_some() {
            return pos.unwrap();
        }
        (0, 0).into()
    }

    fn find_distress_beacon_freq(&self, search_area: usize, multiplier: usize) -> usize {
        let coord = self.find_distress_beacon(0, search_area);
        coord.x as usize * multiplier + coord.y as usize
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let covered = self.sensors.iter()
            .flat_map(|s| s.cover_area())
            .collect::<HashSet<GridCoord>>();
        let min_x = min(self.min_width, covered.iter().map(|p| p.x).min().unwrap());
        let max_x = max(self.max_width, covered.iter().map(|p| p.x).max().unwrap());
        let min_y = min(self.min_height, covered.iter().map(|p| p.y).min().unwrap());
        let max_y = max(self.max_height, covered.iter().map(|p| p.y).max().unwrap());
        writeln!(f, "min: {},{} : max: {},{}", self.min_width, self.min_height, self.max_width, self.max_height)?;
        writeln!(f, "min: {},{} : max: {},{}", min_x, min_y, max_x, max_y)?;
        for y in min_y..=max_y {
            write!(f, "{y}\t")?;
            for x in min_x..=max_x {
                let tile = self.tile(&(x, y).into());
                let t = match tile {
                    Tile::Beacon => 'B',
                    Tile::Sensor => 'S',
                    Tile::Empty => if covered.contains(&(x, y).into()) { '#' } else { '.' }
                };
                write!(f, "{t}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_a(input: &str, row: i32) -> usize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let cave = Cave::parse(open.as_str());
    // println!("{:?}", cave);
    cave.covered_in_row(row)
}

fn part_b(input: &str, search_area: usize, multiplier: usize) -> usize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let cave = Cave::parse(open.as_str());
    // println!("{:?}", cave);
    cave.find_distress_beacon_freq(search_area, multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day15();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day15/input.txt";
        assert_eq!(5083287, part_a(input, 2_000_000));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day15/input.txt";
        assert_eq!(13134039205729, part_b(input, 4_000_000, 4_000_000));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day15/test-input.txt";
        let result = part_a(input, 10);
        assert_eq!(26, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day15/test-input.txt";
        let result = part_b(input, 20, 4_000_000);
        assert_eq!(56000011, result);
    }
}