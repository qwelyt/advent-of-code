use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day18() {
    println!("== Day 18 ==");
    let input = "src/day18/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> isize {
    let f = File::open(input).unwrap();
    let mut ins = Vec::new();
    for l in BufReader::new(f).lines().flatten() {
        ins.push(parse(&l));
    }
    dig(&ins)
}

fn part_b(input: &str) -> isize {
    let f = File::open(input).unwrap();
    let mut ins = Vec::new();
    for l in BufReader::new(f).lines().flatten() {
        ins.push(parse2(&l));
    }
    dig(&ins)
}

type Instruction = (usize, isize, String);
#[allow(dead_code)]
type Point = (isize, isize);

// [R,D,L,U]
const DELTAS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse(s: &str) -> Instruction {
    let sp = s.split(" ").collect::<Vec<&str>>();
    let dir = match sp[0] {
        "R" => 0,
        "D" => 1,
        "L" => 2,
        "U" => 3,
        _ => panic!()
    };
    let amount = sp[1].parse::<isize>().unwrap();
    let hex = sp[2].to_string();
    (dir, amount, hex)
}

fn parse2(s: &str) -> Instruction {
    let sp = s.split(" ").collect::<Vec<&str>>();
    let (d, s) = parse_hex(sp[2]);
    let dir = match d.as_str() {
        "R" => 0,
        "D" => 1,
        "L" => 2,
        "U" => 3,
        _ => panic!()
    };
    let amount = s.parse::<isize>().unwrap();
    let hex = sp[2].to_string();
    (dir, amount, hex)
}

fn parse_hex(s: &str) -> (String, String) {
    // println!("{:?}", s);
    let chars = s.chars()
        .skip(2)
        .filter(|c| *c != '(')
        .filter(|c| *c != ')')
        .collect::<Vec<char>>();
    // println!("{:?}", chars);
    let dir = chars.last()
        .map(|c| match *c {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => panic!()
        })
        .unwrap()
        .to_string();

    let (a, _) = chars.split_at(chars.len().saturating_sub(1));
    let string = a.iter().collect::<String>();
    let amount = usize::from_str_radix(string.as_str(), 16).unwrap();

    (dir, amount.to_string())
}

fn _dig_trench(instructions: &Vec<Instruction>) -> HashSet<Point> {
    let mut points = HashSet::new();
    let mut current = (0, 0);
    points.insert(current);

    for ins in instructions.iter() {
        let delta = DELTAS[ins.0];
        let new_point = (current.0 + (delta.0 * ins.1), current.1 + (delta.1 * ins.1));

        let mmy = (min(current.0, new_point.0), max(current.0, new_point.0));
        let mmx = (min(current.1, new_point.1), max(current.1, new_point.1));
        for y in mmy.0..=mmy.1 {
            for x in mmx.0..=mmx.1 {
                points.insert((y, x));
            }
        }
        current = new_point;
        // points.insert(current);
        // println!("{:?} -> {:?}", ins, current);
    }
    points
}


fn dig(instructions: &Vec<Instruction>) -> isize {
    /*
        Greens theorem
        https://en.wikipedia.org/wiki/Green%27s_theorem
        https://stackoverflow.com/a/451482
        We find each segment, calculate the area for that,
        and then with mathemagics it gives us our area*2.
        So at the end we have to divide by 2, and then +1 because rounding.
     */
    let mut sum = 0;
    let mut current = (0, 0);
    for (dir, amount, _) in instructions.iter() {
        let dir = *dir;
        let amount = *amount;
        let delta = DELTAS[dir];
        let (dy, dx) = (amount * delta.0, amount * delta.1);
        current = (current.0 + dy, current.1 + dx);
        sum += -dx * current.0 + dy * current.1 + amount;
    }
    sum / 2 + 1
}

fn _print_grid(points: &HashSet<Point>) {
    let min_y = points.iter().map(|p| p.0).min().unwrap();
    let max_y = points.iter().map(|p| p.0).max().unwrap() + 1;

    let min_x = points.iter().map(|p| p.1).min().unwrap();
    let max_x = points.iter().map(|p| p.1).max().unwrap() + 1;
    let rows = min_y.abs() + max_y;
    let cols = min_x.abs() + max_x;
    let mut v = vec![vec!['.'; cols as usize]; rows as usize];
    for p in points.iter() {
        let r = p.0 + min_y.abs();
        let c = p.1 + min_x.abs();

        v[r as usize][c as usize] = '#';
    }
    for (y, l) in v.iter().enumerate() {
        println!("{:?}\t{:?}", l.iter().collect::<String>(), y);
    }
    println!("{:?}", points.len());
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
        let result = part_a(input);
        assert_eq!(true, 42941 > result);
        assert_eq!(true, 31030 < result);
        assert_eq!(35244, result);
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day18/input.txt";
        assert_eq!(85070763635666, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day18/test-input.txt";
        let result = part_a(input);
        assert_eq!(62, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day18/test-input.txt";
        let result = part_b(input);
        assert_eq!(952408144115, result);
    }
}