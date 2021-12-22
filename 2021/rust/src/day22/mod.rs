use std::collections::HashSet;
use std::str::FromStr;
use std::thread::current;

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
    follow_instructions(&instructions)
}

fn part_b(input: &Vec<String>) -> u32 {
    todo!()
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

fn follow_instructions(instructions: &Vec<Instruction>) -> usize {
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

impl Cuboid {
    fn overlaps(self, other: Cuboid) -> bool {
        let self_x: HashSet<i64> = HashSet::from_iter((self.x.0..=self.x.1));
        let self_y: HashSet<i64> = HashSet::from_iter((self.y.0..=self.y.1));
        let self_z: HashSet<i64> = HashSet::from_iter((self.z.0..=self.z.1));
        let other_x = HashSet::from_iter((other.x.0..=other.x.1));
        let other_y = HashSet::from_iter((other.y.0..=other.y.1));
        let other_z = HashSet::from_iter((other.z.0..=other.z.1));

        let x = self_x.intersection(&other_x).map(|x| *x).collect::<Vec<i64>>();
        let y = self_y.intersection(&other_y).map(|x| *x).collect::<Vec<i64>>();
        let z = self_z.intersection(&other_z).map(|x| *x).collect::<Vec<i64>>();
        !x.is_empty() && !y.is_empty() && !z.is_empty()
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
        assert_eq!(576600, result);
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
        assert_eq!(2, result);
    }
}
