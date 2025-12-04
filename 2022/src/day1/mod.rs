use crate::util::{lines_as_i32, vecs_i32};

pub fn day1() {
    println!("== Day 1 ==");
    let input = get_input("src/day1/input.txt");
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}

fn get_input(file: &str) -> Vec<i32> {
    let input = lines_as_i32(file);
    let numbers = vecs_i32(&input);
    let summed: Vec<i32> = numbers.iter().map(|v| v.iter().sum()).collect();
    summed
}

fn part_a(input: &Vec<i32>) -> i32 {
    *input.iter().max().unwrap()
}

fn part_b(input: &Vec<i32>) -> i32 {
    let mut summed = input.clone();
    summed.sort();
    summed.reverse();

    let top_three = vec![summed[0], summed[1], summed[2]];

    top_three.iter().sum()
}

fn part_b_2(input: &Vec<i32>) -> i32 {
    let mut top_three = vec![0, 0, 0];
    let mut index = 0;

    for v in input {
        let mut lowest = i32::MAX;
        for i in 0..top_three.len() {
            if top_three[i] < lowest {
                lowest = top_three[i];
                index = i;
            }
        }
        if *v > top_three[index.clone()] {
            top_three[index.clone()] = *v;
        }
    }

    top_three.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day1();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = get_input("src/day1/input.txt");
        assert_eq!(67450, part_a(&input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = get_input("src/day1/input.txt");
        assert_eq!(199357, part_b_2(&input));
    }

    #[ignore]
    #[test]
    fn uppe_the_ante() {
        let instant = Instant::now();
        let input = get_input("src/day1/aoc_2022_day01_large_input.txt");
        let read_input = Instant::now();
        println!("Read input in {}ms", read_input.duration_since(instant).as_millis());

        print!("Part A: {}", part_a(&input));
        let part_a_time = Instant::now();
        println!("  took {}ms", part_a_time.duration_since(read_input).as_millis());

        print!("Part B: {}", part_b(&input));
        let part_b_time = Instant::now();
        println!("  took {}ms", part_b_time.duration_since(part_a_time).as_millis());

        print!("Part B 2: {}", part_b_2(&input));
        let part_b_2_time = Instant::now();
        println!("  took {}ms", part_b_2_time.duration_since(part_b_time).as_millis());
    }

    #[test]
    fn part_a_test_input() {
        let input = get_input("src/day1/test-input.txt");
        let result = part_a(&input);
        assert_eq!(24000, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = get_input("src/day1/test-input.txt");
        let result = part_b(&input);
        assert_eq!(45000, result);
    }
}