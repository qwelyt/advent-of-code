use std::collections::HashSet;
use std::str::FromStr;

use crate::day22::Direction::{Back, Bottom, Front, Left, Right, Top};
use crate::util::lines_from_file;

pub fn day22() {
    println!("== Day 22 ==");
    let input = lines_from_file("src/day22/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> usize {
    let instructions: Vec<Instruction> = input.iter()
        .map(|s| to_instruction(s))
        .collect();
    // for i in instructions.iter() {
    //     println!("{:?}", i);
    // }
    follow_instructions_a(&instructions)
}

fn part_b(input: &Vec<String>) -> usize {
    let instructions: Vec<Instruction> = input.iter()
        .map(|s| to_instruction(s))
        .collect();
    follow_instructions_b(&instructions)
}

fn to_instruction(line: &String) -> Instruction {
    let vec = line.split(" ").collect::<Vec<&str>>();
    let on = *vec.get(0).unwrap() == "on";

    let n = vec.get(1).unwrap()
        .split(",")
        .map(|n| n.split("=").collect::<Vec<&str>>())
        .map(|v| *v.get(1).unwrap())
        .map(|n| n.split("..").map(|s| i64::from_str(s).unwrap()).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();

    let cuboid = Cuboid {
        x: (n[0][0], n[0][1]),
        y: (n[1][0], n[1][1]),
        z: (n[2][0], n[2][1]),
    };

    Instruction { on, cuboid }
}

fn follow_instructions_a(instructions: &Vec<Instruction>) -> usize {
    let mut set: HashSet<Cube> = HashSet::new();
    for instruction in instructions.iter() {
        let vec: Vec<Cube> = instruction.cuboid.cubes_in_range(-50, 50);
        if instruction.on {
            for c in vec.iter() {
                set.insert(*c);
            }
        } else {
            for c in vec.iter() {
                set.remove(c);
            }
        }
    }

    set.len()
}

fn follow_instructions_b(instructions: &Vec<Instruction>) -> usize {
    let mut cuboids: Vec<Cuboid> = Vec::new();

    for instruction in instructions.iter() {
        let current_cuboid = instruction.cuboid;
        let mut new_cuboids = Vec::new();

        if instruction.on {
            new_cuboids.push(current_cuboid);
        }

        for c in cuboids.iter() {
            let mut cuboid = c.clone();
            if !cuboid.overlaps(current_cuboid) {
                new_cuboids.push(cuboid);
            } else {
                // Slice'n'dice with NicerDicer+
                if cuboid.x.0 < current_cuboid.x.0 {
                    new_cuboids.push(cuboid.slice(current_cuboid, Right));
                    cuboid.x.0 = current_cuboid.x.0;
                }

                if cuboid.x.1 > current_cuboid.x.1 {
                    new_cuboids.push(cuboid.slice(current_cuboid, Left));
                    cuboid.x.1 = current_cuboid.x.1;
                }

                if cuboid.y.0 < current_cuboid.y.0 {
                    new_cuboids.push(cuboid.slice(current_cuboid, Top));
                    cuboid.y.0 = current_cuboid.y.0;
                }

                if cuboid.y.1 > current_cuboid.y.1 {
                    new_cuboids.push(cuboid.slice(current_cuboid, Bottom));
                    cuboid.y.1 = current_cuboid.y.1;
                }

                if cuboid.z.0 < current_cuboid.z.0 {
                    new_cuboids.push(cuboid.slice(current_cuboid, Front));
                    cuboid.z.0 = current_cuboid.z.0;
                }

                if cuboid.z.1 > current_cuboid.z.1 {
                    new_cuboids.push(cuboid.slice(current_cuboid, Back));
                    cuboid.z.1 = current_cuboid.z.1;
                }
            }
        }
        cuboids = new_cuboids.clone();
    }


    cuboids.iter()
        .map(|c| {
            let x = (c.x.0 - c.x.1 - 1).abs() as usize;
            let y = (c.y.0 - c.y.1 - 1).abs() as usize;
            let z = (c.z.0 - c.z.1 - 1).abs() as usize;
            return x * y * z;
        })
        .sum()
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Cuboid {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

enum Direction {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back,
}

impl Cuboid {
    fn overlaps(self, other: Cuboid) -> bool {
        !(i64::max(self.x.0, other.x.0) > i64::min(self.x.1, other.x.1)
            || i64::max(self.y.0, other.y.0) > i64::min(self.y.1, other.y.1)
            || i64::max(self.z.0, other.z.0) > i64::min(self.z.1, other.z.1))
    }

    fn cubes_in_range(self, a: i64, b: i64) -> Vec<Cube> {
        let mut vec = Vec::new();
        for x in self.x.0..=self.x.1 {
            if x < a || x > b { continue; }
            for y in self.y.0..=self.y.1 {
                if y < a || y > b { continue; }
                for z in self.z.0..=self.z.1 {
                    if z < a || z > b { continue; }
                    vec.push(Cube { x, y, z })
                }
            }
        }
        vec
    }

    fn slice(self, other: Cuboid, cut: Direction) -> Cuboid {
        match cut {
            Direction::Right => {
                Cuboid {
                    x: (self.x.0, other.x.0 - 1),
                    y: self.y,
                    z: self.z,
                }
            }
            Direction::Left => {
                Cuboid {
                    x: (other.x.1 + 1, self.x.1),
                    y: self.y,
                    z: self.z,
                }
            }
            Direction::Top => {
                Cuboid {
                    x: self.x,
                    y: (self.y.0, other.y.0 - 1),
                    z: self.z,
                }
            }
            Direction::Bottom => {
                Cuboid {
                    x: self.x,
                    y: (other.y.1 + 1, self.y.1),
                    z: self.z,
                }
            }
            Direction::Front => {
                Cuboid {
                    x: self.x,
                    y: self.y,
                    z: (self.z.0, other.z.0 - 1),
                }
            }
            Direction::Back => {
                Cuboid {
                    x: self.x,
                    y: self.y,
                    z: (other.z.1 + 1, self.z.1),
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    on: bool,
    cuboid: Cuboid,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day22/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(590784, result)
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day22/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(602574, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day22/test-input2.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(2758514936282235, result)
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day22/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(1288707160324706, result);
    }
}
