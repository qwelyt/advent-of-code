use crate::util::time;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 9 ==");
    let input = "src/day9/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let disk_sectors = File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>()
                })
                .flatten()
                .collect::<Vec<u32>>()
        })
        .unwrap();
    // println!("{:?}", disk_sectors);

    let mut expanded = expand(&disk_sectors);
    // println!("{:?}", expanded);

    let blanks = expanded
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == -1)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    // println!("{:?}", blanks);

    for blank in blanks.iter() {
        while expanded.last() == Some(&-1) {
            expanded.pop();
        }
        if expanded.len() <= *blank {
            break;
        }
        let last_val = expanded.pop().unwrap();
        expanded[*blank] = last_val;
        // println!("Compacting {:?}", expanded);
    }
    // println!("Compacted {:?}", expanded);
    expanded
        .iter()
        .enumerate()
        .map(|(i, v)| i * (*v as usize))
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}

fn expand(disk_sectors: &Vec<u32>) -> Vec<i32> {
    let mut expanded: Vec<i32> = Vec::new();
    let mut file_id = 0;

    for (i, v) in disk_sectors.iter().enumerate() {
        if i % 2 == 0 {
            // File
            let file = vec![file_id; *v as usize];
            expanded.extend_from_slice(&file);
            file_id += 1;
        } else {
            // Empty space
            let space = vec![-1; *v as usize];
            expanded.extend_from_slice(&space);
        }
    }

    expanded
}

fn part_b(input: &str) -> usize {
    let disk_sectors = File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>()
                })
                .flatten()
                .collect::<Vec<u32>>()
        })
        .unwrap();

    let mut file_id = 0;
    let mut pos = 0;
    let mut files = HashMap::new();
    let mut blanks = Vec::new();
    for (i, v) in disk_sectors.iter().enumerate() {
        if i % 2 == 0 {
            files.insert(file_id, (pos, *v));
            file_id += 1;
        } else {
            if *v != 0 {
                blanks.push((pos, *v));
            }
        }
        pos += *v;
    }

    while file_id > 0 {
        file_id -= 1;
        let (pos, size) = files.get(&file_id).unwrap().clone();
        for i in 0..blanks.len() {
            let (start, len) = blanks[i];
            if start >= pos {
                blanks = blanks.split_at(i + 1).0.to_vec();
                break;
            }
            if size <= len {
                files.insert(file_id, (start, size));
                if size == len {
                    blanks.remove(i);
                } else {
                    blanks[i] = (start + size, len - size);
                }
                break;
            }
        }
    }
    // println!("{:?}", files);
    let mut sum = 0;
    for (file, (pos, size)) in files.iter() {
        for i in *pos..*pos + *size {
            sum += *file as usize * i as usize;
        }
    }
    sum
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
        let input = "src/day9/input.txt";
        assert_eq!(6330095022244, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day9/input.txt";
        assert_eq!(6359491814941, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day9/test-input.txt";
        let result = part_a(input);
        assert_eq!(1928, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day9/test-input.txt";
        let result = part_b(input);
        assert_eq!(2858, result);
    }
}
