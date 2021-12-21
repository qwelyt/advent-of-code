use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use crate::util::lines_from_file;

#[allow(dead_code)]
pub fn day19() {
    println!("== Day 19 ==");
    let input = lines_from_file("src/day19/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: HashMap<u8, HashMap<u8, Vec<Beacon>>>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Scanner { beacons: HashMap::new() }
    }

    fn add_with_all_variations(&mut self, beacon: Beacon) {
        for rotation in 0..4 {
            for up in 0..6 {
                self.beacons.entry(rotation)
                    .or_insert(HashMap::new())
                    .entry(up)
                    .or_insert(Vec::new())
                    .push(beacon);
            }
        }
    }

    fn get(self, rotation: u8, up: u8) -> Option<Vec<Beacon>> {
        self.beacons.get(&rotation)
            .map(|m| m.get(&up))
            .flatten()
            .map(|o| o.clone())
    }


    fn matching_beacon(self, other: &Scanner, rotation: u8, up: u8) -> Option<Beacon> {
        for a in self.beacons.get(&rotation).unwrap().get(&up).unwrap().iter() {
            for b in other.beacons.get(&rotation).unwrap().get(&up).unwrap().iter() {
                let relative = a.minus(b);
                let mut count = 0;
                for c in self.beacons.get(&rotation).unwrap().get(&up).unwrap().iter() {
                    for d in other.beacons.get(&rotation).unwrap().get(&up).unwrap().iter() {
                        let diff = relative.add(c);
                        if diff == *d {
                            count += 1;
                            if count > 11 {
                                return Some(relative.clone());
                            }
                            break;
                        }
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

#[allow(dead_code)]
impl Beacon {
    fn parse(string: &String) -> Self {
        let b: Vec<i32> = string.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| i32::from_str(*s).unwrap())
            .collect();
        Beacon {
            x: *b.get(0).unwrap(),
            y: *b.get(1).unwrap(),
            z: *b.get(2).unwrap(),
        }
    }

    fn zero() -> Beacon {
        Beacon { x: 0, y: 0, z: 0 }
    }

    fn up(self, direction: i8) -> Beacon {
        match direction {
            0 => self.clone(),
            1 => Beacon { x: self.x, y: -self.y, z: -self.z },
            2 => Beacon { x: self.x, y: -self.z, z: self.y },
            3 => Beacon { x: -self.y, y: -self.z, z: self.x },
            4 => Beacon { x: -self.x, y: -self.z, z: -self.y },
            5 => Beacon { x: self.y, y: -self.z, z: -self.x },
            _ => unreachable!()
        }
    }
    fn rotate(self, direction: i8) -> Beacon {
        match direction {
            0 => self.clone(),
            1 => Beacon { x: -self.y, y: self.x, z: self.z },
            2 => Beacon { x: -self.x, y: -self.y, z: self.z },
            3 => Beacon { x: self.y, y: -self.x, z: self.z },
            _ => unreachable!()
        }
    }
    fn minus(self, other: &Beacon) -> Beacon {
        Beacon {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }
    fn add(self, other: &Beacon) -> Beacon {
        Beacon {
            x: other.x + self.x,
            y: other.y + self.y,
            z: other.z + self.z,
        }
    }
}

fn part_a(input: &Vec<String>) -> usize {
    let scanners: Vec<Scanner> = parse(input);
// println!("{} {:?}", scanners.len(), scanners);
    let mut orientation: Vec<(&Scanner, u8, u8)> = Vec::with_capacity(scanners.len());
    let mut beacons = Vec::with_capacity(scanners.len());

    orientation.insert(0, (scanners.get(0).unwrap(), 0, 0));
    beacons.insert(0, Beacon::zero());

    let mut deque = VecDeque::new();
    deque.push_back(0);

    println!("{:?}", orientation);
    println!("{:?}", deque);
    while !deque.is_empty() {
        let dq = deque.pop_front().unwrap();
        for (i, s) in scanners.iter().enumerate() {
            if beacons.get(i).is_none() {
                let (scanner, rotation, up) = *orientation.get(dq).unwrap();
                let found: Option<Beacon> = s.clone().matching_beacon(scanner, rotation, up);
                if found.is_some() {
                    orientation.insert(i, (scanner, rotation, up));

                    let nb = beacons.get(dq).unwrap().add(&found.unwrap());
                    if let Some(beacon) = beacons.get_mut(i) {
                        *beacon = nb;
                    }
                    deque.push_back(i);
                }
            }
        }
    }

    println!("{:?}", beacons);
    println!("{:?}", orientation);

    0
}

fn parse(input: &Vec<String>) -> Vec<Scanner> {
    let mut scanners = Vec::new();

    let mut scanner = Scanner::new();
    for line in input.iter() {
        if line.starts_with("---") {
            if !scanner.beacons.is_empty() {
                scanners.push(scanner.clone());
                scanner = Scanner::new()
            }
        } else if !line.is_empty() {
            scanner.add_with_all_variations(Beacon::parse(line));
        }
    }
    if !scanner.beacons.is_empty() {
        scanners.push(scanner.clone());
    }

    scanners
}

fn part_b(_input: &Vec<String>) -> u32 {
    // todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day19/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(79, result)
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day19/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(4116, result)
    }
}