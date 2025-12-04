use crate::util::time;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Div;

pub fn solve() {
    println!("== Day 14 ==");
    let input = "src/day14/input.txt";
    time(|inpt| part_a(inpt, 101, 103), input, "A");
    time(|inpt| part_b(inpt, 101, 103), input, "B");
}

fn part_a(input: &str, width:i32, height: i32) -> u32 {
    // println!("{:?}x{:?}", width, height);
    let mut robots = File::open(input)
        .map(|f| BufReader::new(f).lines()
            .flatten()
            .map(|l| Robot::of(l.as_str(), (width, height)))
            .collect::<Vec<Robot>>()
        ).unwrap();

    // for robot in robots.iter() {
    //     println!("{:?}", robot);
    // }
    // _print(&robots, (width, height));
    // println!();

    for _s in 1..101 {
        // println!("s: {} r: {:?}", _s, robots);
        robots.iter_mut().for_each(Robot::mv);
        // println!("s: {} r: {:?}", _s, robots);
        // println!("s: {:?}", _s);
        // _print(&robots, (width, height));
        // println!();
    }
    // _print(&robots, (width, height));

    count_quadrants(&robots, (width, height)).iter().product()
}

fn count_quadrants(robots: &Vec<Robot>, area: (i32, i32)) -> Vec<u32> {
    let mut quadrants = vec![0; 4];
    let i = area.0.div(2);
    let j = area.1.div(2);
    // println!("{:?}x{:?}", area.0, area.1);
    // println!("{:?}-{:?} {:?}-{:?}", 0,i,0,j);
    // println!("{:?}-{:?} {:?}-{:?}", 0,i,area.1-j,area.1);
    // println!("{:?}-{:?} {:?}-{:?}", area.0-i,area.0,0,j);
    // println!("{:?}-{:?} {:?}-{:?}", area.0-i,area.0,area.1-j,area.1);

    for ii in 0..i {
        for jj in 0..j {
            for r in robots.iter() {
                if r.pos == (ii, jj) {
                    quadrants[0] +=1;
                }
            }
        }
        for jj in area.1-j..area.1 {
            for r in robots.iter() {
                if r.pos == (ii, jj) {
                    quadrants[2] +=1;
                }
            }
        }
    }
    for ii in area.0-i..area.0 {
        for jj in 0..j {
            for r in robots.iter() {
                if r.pos == (ii, jj) {
                    quadrants[1] +=1;
                }
            }
        }
        for jj in area.1-j..area.1 {
            for r in robots.iter() {
                if r.pos == (ii, jj) {
                    quadrants[3] +=1;
                }
            }
        }
    }
    // println!("{:?}", quadrants);

    quadrants
}

fn part_b(input: &str, width:i32, height: i32) -> usize{
    // This is a horrible problem statement.
    // What does a christmas tree look like?
    // Is it over all quadrants?
    // Is it one tree in each quadrant?
    // How are people with screen readers be able to verify this?
    //
    // Just horrible all around. So let's make assumptions!
    // To make a tree I would _assume_ that we don't want robots at the
    // same spot as that would make holes in the tree.
    // I would also assume that we somewhere want lots of robots in a line.
    // And perhaps a few in a 45 degree angle? Or does a christmas tree not
    // have those?
    let mut robots = File::open(input)
        .map(|f| BufReader::new(f).lines()
            .flatten()
            .map(|l| Robot::of(l.as_str(), (width, height)))
            .collect::<Vec<Robot>>()
        ).unwrap();

    let mut seconds = 0;
    loop {
        seconds +=1;
        robots.iter_mut().for_each(Robot::mv);
        if !overlapping(&robots) {
            return seconds;
        }
    }
}


fn overlapping(robots: &Vec<Robot>) -> bool {
    let mut map = HashSet::new();
    for robot in robots.iter() {
        if map.contains(&robot.pos) {
            return true;
        } else {
            map.insert(robot.pos);
        }
    }
    false
}

fn _print(robots: &Vec<Robot>, area:(i32,i32)) {

    for y in 0..area.1 {
        for x in 0..area.0 {
            let mut present_robots = 0;
            for r in robots.iter() {
                if r.pos == (x,y) {
                    present_robots += 1;
                }
            }
            if present_robots > 0 {
                print!("{}", present_robots)
            } else { print!(".") }
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
    area: (i32, i32),
}
impl Robot {
    fn of(line: &str, area:(i32,i32)) -> Self {
        let (p,v)= line.split_once(" ").unwrap();
        let pos = p.split_once("=").unwrap().1.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let velocity= v.split_once("=").unwrap().1.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        Self {
            pos: (pos[0], pos[1]),
            velocity: (velocity[0], velocity[1]),
            area,
        }
    }

    fn mv(&mut self) {
        self.pos.0 = (self.pos.0+self.velocity.0) % self.area.0;
        self.pos.1 = (self.pos.1+self.velocity.1) % self.area.1;

        if self.pos.0 < 0 {
            self.pos.0 += self.area.0;
        }
        if self.pos.1 < 0 {
            self.pos.1 += self.area.1;
        }
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
        let input = "src/day14/input.txt";
        assert_eq!(211773366, part_a(input, 101,103));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day14/input.txt";
        assert_eq!(7344, part_b(input, 101, 103));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day14/test-input.txt";
        let result = part_a(input,11,7);
        assert_eq!(12, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day14/test-input.txt";
        let result = part_b(input,11,7);
        assert_eq!(0, result);
    }
}