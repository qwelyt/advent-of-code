use std::collections::{HashSet, VecDeque};

use crate::util::time;

pub fn day18() {
    println!("== Day 18 ==");
    let input = "src/day18/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn x(self, i: i32) -> Pos {
        (
            self.x + i,
            self.y,
            self.z,
        ).into()
    }
    fn y(self, i: i32) -> Pos {
        (
            self.x,
            self.y + i,
            self.z,
        ).into()
    }
    fn z(self, i: i32) -> Pos {
        (
            self.x,
            self.y,
            self.z + i,
        ).into()
    }
}

impl From<Vec<&str>> for Pos {
    fn from(data: Vec<&str>) -> Self {
        Self {
            x: data[0].parse::<i32>().unwrap(),
            y: data[1].parse::<i32>().unwrap(),
            z: data[2].parse::<i32>().unwrap(),
        }
    }
}

impl From<(i32, i32, i32)> for Pos {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }
}

struct LavaPalace {
    lava_balls: HashSet<Pos>,
}

impl LavaPalace {
    fn parse(input: &str) -> Self {
        let mut lava_balls = HashSet::new();
        for line in input.lines() {
            lava_balls.insert((line.split(",").collect::<Vec<&str>>()).into());
        }
        Self {
            lava_balls
        }
    }

    fn exposed_ball_faces(&self) -> u32 {
        let dx = [-1, 0, 0, 1, 0, 0];
        let dy = [0, -1, 0, 0, 1, 0];
        let dz = [0, 0, -1, 0, 0, 1];
        let mut exposed = 0;
        for ball in self.lava_balls.iter() {
            for i in 0..dx.len() {
                let ob = ball.x(dx[i]).y(dy[i]).z(dz[i]);
                if !self.lava_balls.contains(&ob) {
                    exposed += 1;
                }
            }
        }
        exposed
    }

    fn exterior_ball_faces(&self) -> u32 {
        let dx = [-1, 0, 0, 1, 0, 0];
        let dy = [0, -1, 0, 0, 1, 0];
        let dz = [0, 0, -1, 0, 0, 1];
        let max = Pos {
            x: self.lava_balls.iter().map(|b| b.x).max().unwrap() + 1,
            y: self.lava_balls.iter().map(|b| b.y).max().unwrap() + 1,
            z: self.lava_balls.iter().map(|b| b.z).max().unwrap() + 1,
        };
        let min = Pos {
            x: self.lava_balls.iter().map(|b| b.x).min().unwrap() - 1,
            y: self.lava_balls.iter().map(|b| b.y).min().unwrap() - 1,
            z: self.lava_balls.iter().map(|b| b.z).min().unwrap() - 1,
        };
        // Max and Min range now contain all lava balls
        // Flood fill the outside volume, starting from min
        // Keep track of the already visited cubes so we don't
        // do extra work, and put the next cube we want to visit into
        // a queue. A sort of BSF algo.

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(min);
        let mut exposed = 0;

        while let Some(ball) = queue.pop_front() {
            if visited.contains(&ball) {
                continue;
            }
            visited.insert(ball);
            for i in 0..dx.len() {
                let ob = ball.x(dx[i]).y(dy[i]).z(dz[i]);
                if ob.x >= min.x && ob.x <= max.x
                    && ob.y >= min.y && ob.y <= max.y
                    && ob.z >= min.z && ob.z <= max.z {
                    if self.lava_balls.contains(&ob) {
                        exposed += 1;
                    } else {
                        queue.push_back(ob);
                    }
                }
            }
        }

        exposed
    }
}

fn part_a(input: &str) -> u32 {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let palace = LavaPalace::parse(open.as_str());
    // println!("{:?}", palace.lava_balls);
    palace.exposed_ball_faces()
}

fn part_b(input: &str) -> u32 {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let palace = LavaPalace::parse(open.as_str());
    palace.exterior_ball_faces()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day18();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day18/input.txt";
        assert_eq!(4390, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day18/input.txt";
        assert_eq!(2534, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day18/test-input.txt";
        let result = part_a(input);
        assert_eq!(64, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day18/test-input.txt";
        let result = part_b(input);
        assert_eq!(58, result);
    }
}