use std::collections::VecDeque;
use crate::util::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 15 ==");
    let input = "src/day15/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let mut warehouse = Warehouse::parse(input);
    //println!("{:?}", warehouse);
    // warehouse._print_map();

    // for _ in 0..warehouse.instructions.len() {
    //     warehouse.follow_instructions(1);
    //     warehouse._print_map();
    //     println!()
    // }
    warehouse.follow_instructions(warehouse.instructions.len());

    warehouse.map.iter().enumerate()
        .map(|(y, row)| row.iter().enumerate()
            .filter(|(_,c)| **c == 'O')
            .map(|(x,_)| y*100+x)
            .collect::<Vec<usize>>()
        )
        .flatten()
        .sum()
}

fn part_b(input: &str) -> usize {
    let mut warehouse = ExpandedWarehouse::parse(input);
    // warehouse._print_map();


    // for _ in 0..20 {
    //     warehouse.follow_instructions(1);
    //     warehouse._print_map();
    //     println!()
    // }

    warehouse.follow_instructions(warehouse.instructions.len());

    warehouse.map.iter().enumerate()
        .map(|(y, row)| row.iter().enumerate()
            .filter(|(_,c)| **c == '[')
            .map(|(x,_)| y*100+x)
            .collect::<Vec<usize>>()
        )
        .flatten()
        .sum()
}

#[derive(Debug)]
struct Warehouse {
    instructions: Vec<char>,
    map: Vec<Vec<char>>,
    robot: (usize, usize),
    current_instruction: usize,
}
impl Warehouse {
    fn parse(input: &str) -> Self {
        let mut map = Vec::new();
        let mut instructions: Vec<char> = Vec::new();
        let mut add_to_warehouse = true;
        for line in File::open(input)
            .map(|f| BufReader::new(f).lines().flatten())
            .unwrap()
        {
            let mut vec = line.chars().collect::<Vec<char>>();
            if line.is_empty() {
                add_to_warehouse = false;
            }
            if add_to_warehouse {
                map.push(vec);
            } else {
                instructions.append(&mut vec);
            }
        }
        let mut robot = None;
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '@' {
                    robot = Some((x, y));
                }
            }
        }
        robot.map(|(x, y)| map[y][x] = '.');
        Self {
            instructions,
            map,
            robot: robot.unwrap(),
            current_instruction: 0,
        }
    }

    fn follow_instructions(&mut self, steps: usize) {
        for instruction_index in self.current_instruction..self.current_instruction + steps {
            match self.instructions[instruction_index] {
                // '^' => self.go_up(),
                // '>' => self.go_right(),
                // 'v' => self.go_down(),
                // '<' => self.go_left(),
                '^' => self.go((0,-1)),
                '>' => self.go((1,0)),
                'v' => self.go((0,1)),
                '<' => self.go((-1,0)),
                _ => {}
            }
        }
        self.current_instruction += steps;
    }

    fn go(&mut self, dir:(isize,isize)) {
        let (x, y) = self.robot;
        let mut xn = x as isize;
        let mut yn = y as isize;
        let mut movers = Vec::new();
        // movers.push((xn, yn));
        let mut mv = true;
        loop {
            xn += dir.0;
            yn += dir.1;
            match self.map[yn as usize][xn as usize] {
                '#' => {mv =false; break;},
                '.' => break,
                'O' => movers.push((xn, yn)),
                _ => {}
            }
        }
        if mv {
            self.map[y][x] = '.';
            self.robot = (( self.robot.0 as isize + dir.0) as usize, (self.robot.1 as isize + dir.1) as usize    );
            for b in movers {
                self.map[(b.1+dir.1) as usize][(b.0+dir.0) as usize] = 'O';
            }
        }


    }

    fn _go_up(&mut self) {
        let (x, y) = self.robot;
        if self.map[y - 1][x] == '.' {
            self.robot = (x, y - 1)
        } else if self.map[y - 1][x] == 'O' {
            // Start moving boxes
            let mut d = 1;
            loop {
                if self.map[y - d][x] == '.' {
                    break;
                }
                if self.map[y - d][x] == '#' {
                    d = 0;
                    break;
                }
                d += 1;
            }
            for dd in 1..=d {
                self.map[y - dd][x] = 'O';
            }
            if d > 1 {
                self.map[y][x] = '.';
                self.map[y-1][x] = '.';
                self.robot = (x, y - 1);
            }
        }
    }
    fn _go_down(&mut self) {
        let (x, y) = self.robot;
        if self.map[y + 1][x] == '.' {
            self.robot = (x, y + 1)
        } else if self.map[y + 1][x] == 'O' {
            // Start moving boxes
            let mut d = 1;
            loop {
                if self.map[y + d][x] == '.' {
                    break;
                }
                if self.map[y + d][x] == '#' {
                    d = 0;
                    break;
                }
                d += 1;
            }
            for dd in 1..=d {
                self.map[y + dd][x] = 'O';
            }
            if d > 1 {
                self.map[y][x] = '.';
                self.map[y+1][x] = '.';
                self.robot = (x, y + 1);
            }
        }
    }
    fn _go_left(&mut self) {
        let (x, y) = self.robot;
        if self.map[y][x - 1] == '.' {
            self.robot = (x - 1, y)
        } else if self.map[y][x - 1] == 'O' {
            // Start moving boxes
            let mut d = 1;
            loop {
                if self.map[y][x - d] == '.' {
                    break;
                }
                if self.map[y][x - d] == '#' {
                    d = 0;
                    break;
                }
                d += 1;
            }
            for dd in 1..=d {
                self.map[y][x - dd] = 'O';
            }
            if d > 1 {
                self.map[y][x] = '.';
                self.map[y][x-1] = '.';
                self.robot = (x - 1, y);
            }
        }
    }
    fn _go_right(&mut self) {
        let (x, y) = self.robot;
        if self.map[y][x + 1] == '.' {
            self.robot = (x + 1, y)
        } else if self.map[y][x + 1] == 'O' {
            // Start moving boxes
            let mut d = 1;
            loop {
                if self.map[y][x + d] == '.' {
                    break;
                }
                if self.map[y][x + d] == '#' {
                    d = 0;
                    break;
                }
                d += 1;
            }
            for dd in 1..=d {
                self.map[y][x + dd] = 'O';
            }
            if d > 1 {
                self.map[y][x] = '.';
                self.map[y][x+1] = '.';
                self.robot = (x + 1, y);
            }
        }
    }

    fn _print_map(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if (x, y) == self.robot {
                    print!("@");
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
        for i in self.instructions.iter() {
            print!(" {} ", i);
        }
        println!();
        for (i, _) in self.instructions.iter().enumerate() {
            if i == self.current_instruction {
                print!(" ^ ");
            } else {
                print!("   ");
            }
        }
        println!();
    }
}

#[derive(Debug)]
struct ExpandedWarehouse {
    instructions: Vec<char>,
    map: Vec<Vec<char>>,
    robot: (usize, usize),
    current_instruction: usize,
}

impl ExpandedWarehouse {
    fn parse(input: &str) -> Self {
        let mut instructions = Vec::new();
        let mut map = Vec::new();
        let mut add_to_map = true;
        for line in File::open(input)
            .map(|f| BufReader::new(f).lines().flatten())
            .unwrap()
        {
            let mut vec = line.chars().collect::<Vec<char>>();
            if line.is_empty() {
                add_to_map = false;
            }
            if add_to_map {
                let expanded = vec.iter().map(|c| match c {
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '#' => vec!['#', '#'],
                    '@' => vec!['@', '.'],
                    _ => panic!("Unexpected character: {}", c),
                }).flatten().collect::<Vec<char>>();
                map.push(expanded);
            } else {
                instructions.append(&mut vec);
            }
        }
        let mut robot = None;
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '@' {
                    robot = Some((y, x));
                }
            }
        }
        robot.map(|(y, x)| map[y][x] = '.');

        Self {
            instructions,
            map,
            robot: robot.unwrap(),
            current_instruction: 0,
        }
    }

    fn follow_instructions(&mut self, steps: usize) {
        for instruction_index in self.current_instruction..self.current_instruction + steps {
            match self.instructions[instruction_index] {
                '^' => self.go((-1,0)),
                '>' => self.go((0,1)),
                'v' => self.go((1,0)),
                '<' => self.go((0,-1)),
                _ => {}
            }

        }
        self.current_instruction += steps;
    }

    fn go(&mut self, dir: (isize, isize)) {
        let (y, x) = self.robot;
        let (dy, dx) = dir;
        let mut movers = Vec::new();
        let mut q = VecDeque::new();
        movers.push((y as isize, x as isize));
        q.push_back((y as isize, x as isize));
        let mut mv = true;
        while let Some((yy,xx)) = q.pop_front() {
            let yn = yy+dy;
            let xn = xx+dx;
            if movers.contains(&(yn, xn)) { continue; }
            match self.map[yn as usize][xn as usize] {
                '#' => {mv =false; break;},
                '[' => {
                    q.push_back((yn, xn));movers.push((yn, xn));
                    q.push_back((yn, xn+1));movers.push((yn, xn+1));
                },
                ']' => {
                    q.push_back((yn, xn));movers.push((yn, xn));
                    q.push_back((yn, xn-1));movers.push((yn, xn-1));
                },
                _ => {}
            }


        }
        // println!("{:?}", movers);
        // println!("{:?}", self.robot);
        if mv {
            self.map[y][x] = '.';
            self.robot = (( self.robot.0 as isize + dy) as usize, (self.robot.1 as isize + dx) as usize);
            for (by,bx) in movers.iter().rev() {
                self.map[(by + dy) as usize][(bx + dx) as usize] = self.map[*by as usize][*bx as usize];
                self.map[*by as usize][*bx as usize] = '.';
            }
        }
    }

    fn _print_map(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if (y,x) == self.robot {
                    print!("@");
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
        for i in self.instructions.iter() {
            print!(" {} ", i);
        }
        println!();
        for (i, _) in self.instructions.iter().enumerate() {
            if i == self.current_instruction {
                print!(" ^ ");
            } else {
                print!("   ");
            }
        }
        println!();
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
        let input = "src/day15/input.txt";
        assert_eq!(1577255, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day15/input.txt";
        assert_eq!(1597035, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day15/test-input.txt";
        let result = part_a(input);
        assert_eq!(2028, result);
    }

    #[test]
    fn part_a_test_input2() {
        let input = "src/day15/test-input-2.txt";
        let result = part_a(input);
        assert_eq!(10092, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day15/test-input-2.txt";
        let result = part_b(input);
        assert_eq!(9021, result);
    }
}
