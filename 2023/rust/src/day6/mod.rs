use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day6() {
    println!("== Day 6 ==");
    let input = "src/day6/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let mut lines = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        lines.push(
            line.split(" ").skip(1)
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        );
    }
    let mut time_record = Vec::new();
    for i in 0..lines[0].len() {
        time_record.push((lines[0][i], lines[1][i]));
    }
    // println!("{:?}", lines);
    // println!("{:?}", time_record);
    time_record.iter()
        .map(num_combinations_to_beat_record)
        .product()
}

fn num_combinations_to_beat_record(time_record: &(usize, usize)) -> usize {
    let (time, record) = *time_record;
    let mut sum = 0;
    for i in 0..time {
        let time_left = time - i;
        let traveled_distance = time_left * i;
        if traveled_distance > record {
            sum += 1;
        }
    }
    sum
}


fn part_b(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let mut lines = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let s = line.split(" ").skip(1)
            .filter(|s| !s.is_empty())
            .collect::<String>();
        lines.push(s.parse::<usize>().unwrap());
    }
    // println!("{:?}", lines);
    let time_record = (lines[0], lines[1]);
    num_combinations_to_beat_record(&time_record)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day6();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day6/input.txt";
        assert_eq!(4403592, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day6/input.txt";
        assert_eq!(38017587, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day6/test-input.txt";
        let result = part_a(input);
        assert_eq!(288, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day6/test-input.txt";
        let result = part_b(input);
        assert_eq!(71503, result);
    }
}