use crate::util::{lines, to_i32, vecs};

pub fn day1() {
    println!("== Day 1 ==");
    let input = lines("src/day1/input.txt");
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}

fn part_a(input: &Vec<String>) -> i32 {
    let vec = vecs(input);
    let numbers: Vec<Vec<i32>> = vec.into_iter().map(|v| to_i32(&v)).collect();
    let summed: Vec<i32> = numbers.iter().map(|v| v.iter().sum()).collect();


    *summed.iter().max().unwrap()
}

fn part_b(input: &Vec<String>) -> i32 {
    let vec = vecs(input);
    let numbers: Vec<Vec<i32>> = vec.into_iter().map(|v| to_i32(&v)).collect();
    let mut summed: Vec<i32> = numbers.iter().map(|v| v.iter().sum()).collect();

    summed.sort();
    summed.reverse();

    let top_three = vec![summed[0], summed[1], summed[2]];

    top_three.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runday() {
        day1();
    }

    #[test]
    fn part_a_test_input() {
        let input = lines("src/day1/test-input.txt");
        let result = part_a(&input);
        assert_eq!(24000, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = lines("src/day1/test-input.txt");
        let result = part_b(&input);
        assert_eq!(45000, result);
    }
}