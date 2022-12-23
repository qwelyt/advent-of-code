use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::ops::Add;

use crate::util::time;

pub fn day23() {
    println!("== Day 23 ==");
    let input = "src/day23/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Pos {
    fn from(xy: (i32, i32)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
}

impl Pos {
    fn new(xy: (i32, i32)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y }
    }
}

#[derive(Debug)]
struct Elves {
    positions: HashSet<Pos>,
    proposed: HashMap<Pos, Vec<Pos>>,
}

impl Elves {
    fn parse(input: &str) -> Self {
        let mut positions = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            let poss = line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(|(x, _c)| Pos { x: x as i32, y: y as i32 })
                .collect::<Vec<Pos>>();
            for pos in poss.iter() {
                positions.insert(*pos);
            }
        }


        Self {
            positions,
            proposed: HashMap::new(),
        }
    }

    fn has_neighbour(&self, pos: &Pos) -> bool {
        let delta = [
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), /*elf*/ (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        for d in delta {
            let dpos = *pos + d.into();
            if self.positions.contains(&dpos) {
                return true;
            }
        }
        false
    }

    fn move_proposal(&self, elf: &Pos, round: usize) -> Option<Pos> {
        let n = || {
            let north = [(-1, -1), (0, -1), (1, -1)];
            let north = !north.iter()
                .map(|d| Pos::new(*d))
                .map(|d| *elf + d)
                .map(|d| self.positions.contains(&d))
                .collect::<HashSet<bool>>()
                .contains(&true);
            if north { return Some(*elf + (0, -1).into()); }
            None
        };

        let s = || {
            let south = [(-1, 1), (0, 1), (1, 1)];
            let south = !south.iter()
                .map(|d| Pos::new(*d))
                .map(|d| *elf + d)
                .map(|d| self.positions.contains(&d))
                .collect::<HashSet<bool>>()
                .contains(&true);
            if south { return Some(*elf + (0, 1).into()); }
            None
        };

        let w = || {
            let west = [(-1, -1), (-1, 0), (-1, 1)];
            let west = !west.iter()
                .map(|d| Pos::new(*d))
                .map(|d| *elf + d)
                .map(|d| self.positions.contains(&d))
                .collect::<HashSet<bool>>()
                .contains(&true);
            if west { return Some(*elf + (-1, 0).into()); }
            None
        };

        let e = || {
            let east = [(1, -1), (1, 0), (1, 1)];
            let east = !east.iter()
                .map(|d| Pos::new(*d))
                .map(|d| *elf + d)
                .map(|d| self.positions.contains(&d))
                .collect::<HashSet<bool>>()
                .contains(&true);
            if east { return Some(*elf + (1, 0).into()); }
            None
        };
        let fns = match round % 4 {
            0 => ['n', 's', 'w', 'e', ],
            1 => ['s', 'w', 'e', 'n', ],
            2 => ['w', 'e', 'n', 's', ],
            3 => ['e', 'n', 's', 'w', ],
            _ => [' ', ' ', ' ', ' ']
        };

        for f in fns {
            if f == 'n' {
                let o = n();
                if o.is_some() {
                    return o;
                }
            }
            if f == 's' {
                let o = s();
                if o.is_some() {
                    return o;
                }
            }
            if f == 'w' {
                let o = w();
                if o.is_some() {
                    return o;
                }
            }
            if f == 'e' {
                let o = e();
                if o.is_some() {
                    return o;
                }
            }
        }

        None
    }

    fn spread_out(&mut self, rounds: usize) -> usize {
        // self.print();
        for round in 0..rounds {
            // Propose
            for elf in self.positions.iter() {
                if self.has_neighbour(elf) {
                    let prop = self.move_proposal(elf, round);
                    if prop.is_some() {
                        self.proposed.entry(prop.unwrap())
                            .or_default()
                            .push(*elf)
                    }
                }
            }

            // Move if there are proposals
            if self.proposed.len() == 0 {
                break;
            }
            // println!("There were {} proposed moves", self.proposed.len());
            for (prop, elves) in self.proposed.iter() {
                if elves.len() == 1 {
                    let elf = elves.first().unwrap();
                    let new_pos = *prop;
                    // println!("Move {:?} to {:?}", elf, new_pos);
                    self.positions.remove(elf);
                    self.positions.insert(new_pos);
                }
            }
            self.proposed.clear();
            // println!("Round: {}", round+1);
            // self.print();
            // break;
        }


        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        for pos in self.positions.iter() {
            min_y = min(min_y, pos.y);
            min_x = min(min_x, pos.x);
            max_y = max(max_y, pos.y);
            max_x = max(max_x, pos.x);
        }
        // let mut soil = 0;
        // for y in min_y..=max_y{
        //     for x in min_x..=max_x {
        //         if !self.positions.contains(&(x,y).into()){
        //             soil += 1;
        //         }
        //     }
        // }
        let num_y = 1 + (max_y - min_y).abs() as usize; // The full range
        let num_x = 1 + (max_x - min_x).abs() as usize;
        let soil = (num_y * num_x) - self.positions.len();

        soil
    }
    fn spread_out_stable(&mut self) -> usize {
        // self.print();
        let mut round = 0;
        loop {
            // Propose
            for elf in self.positions.iter() {
                if self.has_neighbour(elf) {
                    let prop = self.move_proposal(elf, round);
                    if prop.is_some() {
                        self.proposed.entry(prop.unwrap())
                            .or_default()
                            .push(*elf)
                    }
                }
            }

            // Move if there are proposals
            if self.proposed.len() == 0 {
                break;
            }
            // println!("There were {} proposed moves", self.proposed.len());
            for (prop, elves) in self.proposed.iter() {
                if elves.len() == 1 {
                    let elf = elves.first().unwrap();
                    let new_pos = *prop;
                    // println!("Move {:?} to {:?}", elf, new_pos);
                    self.positions.remove(elf);
                    self.positions.insert(new_pos);
                }
            }
            self.proposed.clear();
            // println!("Round: {}", round+1);
            round += 1;
            // self.print();
            // break;
        }
        round + 1
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        for pos in self.positions.iter() {
            min_y = min(min_y, pos.y);
            min_x = min(min_x, pos.x);
            max_y = max(max_y, pos.y);
            max_x = max(max_x, pos.x);
        }
        println!("({},{}) - ({},{})", min_x, min_y, max_x, max_y);
        for y in min_y..=max_y {
            print!("{}\t", y);
            for x in min_x..=max_x {
                if self.positions.contains(&(x, y).into()) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn part_a(input: &str) -> usize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let mut elves = Elves::parse(open.as_str());
    // println!("{:?}", elves);
    let i = elves.spread_out(10);
    // println!("{:?}", elves);
    i
}

fn part_b(input: &str) -> usize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let mut elves = Elves::parse(open.as_str());
    elves.spread_out_stable()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day23();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day23/input.txt";
        assert_eq!(3920, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day23/input.txt";
        assert_eq!(889, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day23/test-input.txt";
        let result = part_a(input);
        assert_eq!(110, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day23/test-input.txt";
        let result = part_b(input);
        assert_eq!(20, result);
    }
}