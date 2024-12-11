use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::util::time;

pub fn solve() {
    println!("== Day 10 ==");
    let input = "src/day10/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let map = map(input);
    let starts = starts(&map);

    // for l in map.iter(){
    //     println!("{:?}", l);
    // }
    // println!("{:?}", starts);

    starts.iter().map(|p| walk(&map, *p)).sum()
}

fn starts(map: &Vec<Vec<u32>>) -> Vec<(i32, i32)> {
    map.iter()
        .enumerate()
        .map(|(r, row)| row.iter().enumerate()
            .filter(|(_, val)| **val == 0)
            .map(|(c, _)| (r as i32, c as i32))
            .collect::<Vec<(i32, i32)>>())
        .flatten()
        .collect::<Vec<(i32, i32)>>()
}

fn walk(map: &Vec<Vec<u32>>, start: (i32, i32)) -> usize {
    let mut q = VecDeque::new();
    q.push_back(start);
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut count = 0;
    let dr = [-1,0,1,0];
    let dc = [0,1,0,-1];
    while !q.is_empty() {
        let pos = q.pop_front().unwrap();
        for dir in 0..4 {
            let new_pos = (pos.0 + dr[dir], pos.1 + dc[dir]);
            if new_pos.0 < 0 || new_pos.1 < 0 {continue;}
            if new_pos.0 as usize >= map.len() || new_pos.1 as usize >= map[new_pos.0 as usize].len(){ continue;}
            if map[new_pos.0 as usize][new_pos.1 as usize] != map[pos.0 as usize][pos.1 as usize] +1 { continue; }
            if visited.contains(&new_pos){ continue;}
            visited.insert(new_pos);
            if map[new_pos.0 as usize][new_pos.1 as usize] == 9 {
                count += 1;
            } else {
                q.push_back(new_pos);
            }
        }
    }
    count
}

fn map(input: &str) -> Vec<Vec<u32>> {
    File::open(input)
        .map(|f| BufReader::new(f)
            .lines()
            .flatten()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>()
        ).unwrap()
}

fn part_b(input: &str) -> usize {
    let map = map(input);
    let starts = starts(&map);

    starts.iter().map(|p| rate_starts(&map, *p)).sum()
}
fn rate_starts(map: &Vec<Vec<u32>>, start: (i32, i32)) -> usize {
    let mut q = VecDeque::new();
    q.push_back(start);
    let mut visited = HashMap::new();
    visited.insert(start, 1);
    let mut count = 0;
    let dr = [-1,0,1,0];
    let dc = [0,1,0,-1];
    while !q.is_empty() {
        let pos = q.pop_front().unwrap();
        if map[pos.0 as usize][pos.1 as usize] == 9 {
            count += visited.get(&pos).unwrap();
        }
        for dir in 0..4 {
            let new_pos = (pos.0 + dr[dir], pos.1 + dc[dir]);

            if new_pos.0 < 0 || new_pos.1 < 0 {continue;}
            if new_pos.0 as usize >= map.len() || new_pos.1 as usize >= map[new_pos.0 as usize].len(){ continue;}
            if map[new_pos.0 as usize][new_pos.1 as usize] != map[pos.0 as usize][pos.1 as usize] +1 { continue; }

            if visited.contains_key(&new_pos){
                let x = visited.get(&pos).unwrap().clone();
                visited.entry(new_pos).and_modify(|mut e| *e += x);
                continue;
            }
            let x = visited.get(&pos).unwrap().clone();
            visited.insert(new_pos, x);
            q.push_back(new_pos);
        }
    }
    count
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
        let input = "src/day10/input.txt";
        assert_eq!(531, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day10/input.txt";
        assert_eq!(1210, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day10/test-input.txt";
        let result = part_a(input);
        assert_eq!(36, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day10/test-input.txt";
        let result = part_b(input);
        assert_eq!(81, result);
    }
}