use std::{fmt, fs};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Formatter;

use crate::util::time;

pub fn day14() {
    println!("== Day 14 ==");
    let input = "src/day14/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Cell {
    Stone,
    Air,
    Sand,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Left,
    Down,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl GridCoord {
    fn down(coord: &GridCoord) -> GridCoord {
        GridCoord {
            x: coord.x,
            y: coord.y + 1,
        }
    }
    fn left(coord: &GridCoord) -> GridCoord {
        GridCoord {
            x: coord.x - 1,
            y: coord.y + 1,
        }
    }
    fn right(coord: &GridCoord) -> GridCoord {
        GridCoord {
            x: coord.x + 1,
            y: coord.y + 1,
        }
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

// #[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    coord: GridCoord,
    cell: Cell,
}

struct Cave {
    width: usize,
    height: usize,
    occupied: HashSet<Position>,
}

impl Cave {
    fn parse(input: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut stones = HashSet::new();
        for line in input.lines() {
            let positions = line.split(" -> ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.split(",").collect::<Vec<&str>>())
                .map(|s| (s[0].parse::<u32>().unwrap(), s[1].parse::<u32>().unwrap()))
                .collect::<Vec<(u32, u32)>>();
            for i in 0..positions.len() {
                let (x, y) = positions[i];
                if x > width {
                    width = x;
                }
                if y > height {
                    height = y;
                }
                if (i + 1) != positions.len() {
                    let (to_x, to_y) = positions[i + 1];
                    for xp in min(x, to_x)..=max(x, to_x) {
                        for yp in min(y, to_y)..=max(y, to_y) {
                            stones.insert(Position {
                                coord: GridCoord {
                                    x: xp as usize,
                                    y: yp as usize,
                                },
                                cell: Cell::Stone,
                            });
                        }
                    }
                }
            }
        }
        // println!("{:?}", stones);
        Self {
            width: 1 + width as usize,
            height: 1 + height as usize,
            occupied: stones,
        }
    }

    fn in_bounds(&self, coord: &GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn cell(&self, coord: &GridCoord) -> Option<&Cell> {
        // if !self.in_bounds(coord) {
        //     return None;
        // }
        let occupied = self.occupied.iter()
            .find(|p| p.coord.eq(&coord))
            .map(|p| &p.cell);
        if occupied.is_none() {
            return Some(&Cell::Air);
        }
        occupied
    }

    fn pour_sand(&mut self) -> u32 {
        let start = GridCoord { x: 500, y: 0 };
        let mut units = 0;
        let mut full = false;
        while !full {
            let mut curr_pos = start;
            while let Some(direction) = self.go_to(&curr_pos, false) {
                if !self.in_bounds(&curr_pos) {
                    full = true;
                    break;
                }

                curr_pos = match direction {
                    Direction::Left => GridCoord::left(&curr_pos),
                    Direction::Down => GridCoord::down(&curr_pos),
                    Direction::Right => GridCoord::right(&curr_pos),
                }
            }
            if !full {
                units += 1;
                self.occupied.insert(Position {
                    coord: curr_pos,
                    cell: Cell::Sand,
                });
            }
            // println!("{:?}", self);
        }
        units
    }

    fn go_to(&self, sand: &GridCoord, has_floor: bool) -> Option<Direction> {
        if has_floor && GridCoord::down(sand).y == self.height + 1 {
            return None;
        }
        if !self.is_occupied(&GridCoord::down(sand)) {
            return Some(Direction::Down);
        } else if !self.is_occupied(&GridCoord::left(sand)) {
            return Some(Direction::Left);
        } else if !self.is_occupied(&GridCoord::right(sand)) {
            return Some(Direction::Right);
        }
        None
    }

    fn is_occupied(&self, coord: &GridCoord) -> bool {
        let cell = self.cell(coord);
        if cell.is_some() {
            return match cell.unwrap() {
                Cell::Stone => true,
                Cell::Air => false,
                Cell::Sand => true,
            };
        }
        false
    }
    fn pour_sand_2(&mut self) -> u32 {
        let start = GridCoord { x: 500, y: 0 };
        let mut units = 0;
        let mut _full = false;
        while !_full {
            let mut curr_pos = start;
            while let Some(direction) = self.go_to(&curr_pos, true) {
                curr_pos = match direction {
                    Direction::Left => GridCoord::left(&curr_pos),
                    Direction::Down => GridCoord::down(&curr_pos),
                    Direction::Right => GridCoord::right(&curr_pos),
                };
            }
            if curr_pos.eq(&start) {
                units += 1;
                _full = true;
                break;
            }
            if !_full {
                units += 1;
                self.occupied.insert(Position {
                    coord: curr_pos,
                    cell: Cell::Sand,
                });
            }
        }
        units
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_occupied_y = self.occupied.iter().map(|p| p.coord.y).max().unwrap_or(0) + 1;
        let max_occupied_x = self.occupied.iter().map(|p| p.coord.x).max().unwrap_or(0) + 1;
        let height = max(self.height, max_occupied_y);
        let width = max(max_occupied_x, self.width);
        writeln!(f, "{}x{} grid ({}x{}):", width, height, self.width, self.height)?;
        for y in 0..height {
            for x in 0..width {
                if y == 0 && x == 500 {
                    write!(f, "+")?;
                } else {
                    let cell = self.cell(&(x, y).into()).unwrap();
                    let c = match cell {
                        Cell::Stone => '#',
                        Cell::Sand => 'o',
                        Cell::Air => ' ',
                    };
                    write!(f, "{c}")?;
                }
            }
            writeln!(f)?;
        }
        // for p in self.occupied.iter() {
        //     writeln!(f, "{:?}, {:?}", p.coord, p.cell)?;
        // }
        Ok(())
    }
}


fn part_a(input: &str) -> u32 {
    let open = fs::read_to_string(input.to_string()).expect("Could not read file");
    let mut cave = Cave::parse(open.as_str());
    let i = cave.pour_sand();
    // println!("{:?}", cave);
    i
}

fn part_b(input: &str) -> u32 {
    let open = fs::read_to_string(input.to_string()).expect("Could not read file");
    let mut cave = Cave::parse(open.as_str());
    let i = cave.pour_sand_2();
    // println!("{:?}", cave);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day14();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day14/input.txt";
        assert_eq!(696, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day14/input.txt";
        assert_eq!(23610, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day14/test-input.txt";
        let result = part_a(input);
        assert_eq!(24, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day14/test-input.txt";
        let result = part_b(input);
        assert_eq!(93, result);
    }
}