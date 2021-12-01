use crate::util::{lines_from_file, string_to_i32};

pub fn day1() {
    println!("== Day 1 ==");
    let input = string_to_i32(lines_from_file("src/day1/input.txt"));
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}


fn part_a(input: &Vec<i32>) -> i32 {
    let mut last: i32 = input[0];
    let mut increases: i32 = 0;
    for (i, x) in input.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if x > &last {
            increases += 1;
        }
        last = *x;
    }
    increases
}

fn part_b(input: &Vec<i32>) -> i32 {
    let mut added_points: Vec<i32> = Vec::new();

    for (i, x) in input.iter().enumerate() {
        if i + 2 > input.len() - 1 {
            break;
        }
        added_points.push(*x + input[i + 1] + input[i + 2]);
    }

    part_a(&added_points)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day1/test-input.txt";
        let input = string_to_i32(lines_from_file(filename));
        let result = part_a(&input);
        assert_eq!(7, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day1/test-input.txt";
        let input = string_to_i32(lines_from_file(filename));
        let result = part_b(&input);
        assert_eq!(5, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day1/input.txt";
        let input = string_to_i32(lines_from_file(filename));
        let result = part_a(&input);
        assert_eq!(1393, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day1/input.txt";
        let input = string_to_i32(lines_from_file(filename));
        let result = part_b(&input);
        assert_eq!(1359, result);
    }
}
