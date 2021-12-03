use crate::util::{lines_from_file, string_to_i32};

pub fn day2() {
    println!("== Day 2 ==");
    let input = to_instruction(lines_from_file("src/day2/input.txt"));
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

struct Instruction(String, i32);


fn part_a(input: &Vec<Instruction>) -> i32 {
    let mut h: i32 = 0;
    let mut v: i32 = 0;
    for i in input {
        // println!("{} - {}", i.0, i.1);
        match i.0.as_str() {
            "forward" => h += i.1,
            "up" => v -= i.1,
            "down" => v += i.1,
            &_ => {}
        }
    }
    h * v
}

fn part_b(input: &Vec<Instruction>) -> i32 {
    let mut h = 0;
    let mut v = 0;
    let mut aim = 0;

    for i in input {
        // println!("{} - {}", i.0, i.1);
        match i.0.as_str() {
            "forward" => {
                h += i.1;
                v += aim * i.1;
            }
            "up" => aim -= i.1,
            "down" => aim += i.1,
            &_ => {}
        }
    }
    h * v
}

fn to_instruction(input: Vec<String>) -> Vec<Instruction> {
    input.iter()
        .map(|s| s.split(' ').collect())
        .map(|s: Vec<&str>| Instruction(s[0].to_string(), string_to_i32(s[1])))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day2/test-input.txt";
        let input = lines_from_file(filename);
        let vec = to_instruction(input);
        let result = part_a(&vec);
        assert_eq!(150, result);
    }

    #[test]
    fn part_a_real() {
        let result = part_a(&to_instruction(lines_from_file("src/day2/input.txt")));
        assert_eq!(1990000, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day2/test-input.txt";
        let input = lines_from_file(filename);
        let vec = to_instruction(input);
        let result = part_b(&vec);
        assert_eq!(900, result);
    }

    #[test]
    fn part_b_real() {
        let result = part_b(&to_instruction(lines_from_file("src/day2/input.txt")));
        assert_eq!(1975421260, result);
    }
}