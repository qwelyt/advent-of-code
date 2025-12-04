use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use crate::util::lines_from_file;

pub fn day24() {
    println!("== Day 24 ==");
    let input = lines_from_file("src/day24/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> i64 {
    let blocks = extract_blocks(input);
    let i = alu2(&blocks, 9);
    let (w, x, y, z) = alu(input, i);
    if z != 0 { panic!("z is not zero! {}", z) }
    i
}

fn part_b(input: &Vec<String>) -> i64 {
    let blocks = extract_blocks(input);
    let i = alu2(&blocks, 1);
    let (w, x, y, z) = alu(input, i);
    if z != 0 { panic!("z is not zero! {}", z) }
    i
}

fn extract_blocks(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut blocks = Vec::new();
    let mut block = Vec::new();
    for line in input.iter() {
        if line == "inp w" && !block.is_empty() {
            blocks.push(block);
            block = Vec::new()
        }
        block.push(line.clone());
    }
    blocks.push(block);
    blocks
}

fn calculate_serial(blocks: Vec<Vec<String>>) -> (i64, i64) {
    for (bi, block) in blocks.iter().enumerate() {
        // println!("{:?}", block);
        // for n in 1..=9 {
        //     let number = concat(&vec![n; 14]);
        //     let (w,x,y,z) = alu(block, number);
        //     if z == 0 {
        //         println!("{}, {}:  ({},{},{},{})", bi, n, w, x, y, z);
        //     }
        // }
        let (min, max) = try_serial(block);
        println!("{}: {} , {}", bi, min, max);
    }
    (0, 0)
}

fn try_serial(instructions: &Vec<String>) -> (i64, i64) {
    let mut numbers: Vec<i64> = Vec::new();
    // To slow
    // for n in 100_000_000_000_00..100_000_000_000_000{
    //     let digits: Vec<_> = n.to_string().chars().collect::<Vec<char>>();
    //     if digits.contains(&'0'){
    //         continue;
    //     }
    //     let (w,x,y,z) = alu(instructions, n);
    //     if z == 0 {
    //         numbers.push(n);
    //     }
    // }
    let valid = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for n1 in valid {
        println!("n1 is {}", n1);
        for n2 in valid {
            for n3 in valid {
                for n4 in valid {
                    for n5 in valid {
                        for n6 in valid {
                            for n7 in valid {
                                for n8 in valid {
                                    for n9 in valid {
                                        for n10 in valid {
                                            for n11 in valid {
                                                for n12 in valid {
                                                    for n13 in valid {
                                                        for n14 in valid {
                                                            let digits = vec![n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14];
                                                            let number = concat(&digits);
                                                            println!("{}", number);
                                                            let (w, x, y, z) = alu(instructions, number);
                                                            if z == 0 {
                                                                numbers.push(number);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (*numbers.iter().min().unwrap(), *numbers.iter().max().unwrap())
}

fn concat(digits: &Vec<i64>) -> i64 {
    digits.iter().fold(0, |acc, elem| acc * 10 + elem)
}

fn alu2(instructions: &Vec<Vec<String>>, start_with: i64) -> i64 {
    let mut number = [start_with; 14];
    let mut stack: VecDeque<(usize, i64)> = VecDeque::new();
    for (i, block) in instructions.iter().enumerate() {
        let div = get_value_from_line(block.get(4).unwrap());
        let chk = get_value_from_line(block.get(5).unwrap());
        let add = get_value_from_line(block.get(15).unwrap());
        if div == 1 {
            stack.push_back((i, add));
        } else if div == 26 {
            let (j, sadd) = stack.pop_back().unwrap();
            number[i] = (number[j] + sadd + chk);
            if number[i] > 9 {
                number[j] = number[j] - (number[i] - 9);
                number[i] = 9;
            } else if number[i] < 1 {
                number[j] = number[j] + (1 - number[i]);
                number[i] = 1;
            }
        }
    }
    concat(&number.to_vec())
}

fn get_value_from_line(line: &String) -> i64 {
    i64::from_str(line.split(" ").collect::<Vec<&str>>().get(2).unwrap()).unwrap()
}

fn alu(instructions: &Vec<String>, number: i64) -> (i64, i64, i64, i64) {
    let mut registers: HashMap<&str, i64> = HashMap::from([
        ("w", 0),
        ("x", 0),
        ("y", 0),
        ("z", 0),
    ]);

    let number_of_inps = instructions.iter().filter(|l| l.starts_with("inp")).collect::<Vec<&String>>().len();
    let mut deque = if number_of_inps == 1 {
        VecDeque::from([number])
    } else {
        let digits: Vec<_> = number.to_string().chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
        VecDeque::from_iter(digits.into_iter())
    };

    for i in instructions.iter() {
        let parts = i.split(" ").collect::<Vec<&str>>();
        match parts[0] {
            "inp" => { registers.insert(parts[1], deque.pop_front().unwrap()); }
            "add" => {
                let number = if registers.contains_key(parts[2]) {
                    *registers.get(parts[2]).unwrap()
                } else {
                    i64::from_str(parts[2]).unwrap()
                };
                *registers.entry(parts[1]).or_insert(0) += number;
            }
            "mul" => {
                let number = if registers.contains_key(parts[2]) {
                    *registers.get(parts[2]).unwrap()
                } else {
                    i64::from_str(parts[2]).unwrap()
                };
                *registers.entry(parts[1]).or_insert(0) *= number;
            }
            "div" => {
                let number = if registers.contains_key(parts[2]) {
                    *registers.get(parts[2]).unwrap()
                } else {
                    i64::from_str(parts[2]).unwrap()
                };
                *registers.entry(parts[1]).or_insert(0) /= number;
            }
            "mod" => {
                let number = if registers.contains_key(parts[2]) {
                    *registers.get(parts[2]).unwrap()
                } else {
                    i64::from_str(parts[2]).unwrap()
                };
                *registers.entry(parts[1]).or_insert(0) %= number;
            }
            "eql" => {
                let number = if registers.contains_key(parts[2]) {
                    *registers.get(parts[2]).unwrap()
                } else {
                    i64::from_str(parts[2]).unwrap()
                };
                let equal = if *registers.get(parts[1]).unwrap() == number { 1 } else { 0 };
                *registers.entry(parts[1]).or_insert(0) = equal;
            }
            _ => unreachable!()
        }
    }

    (
        *registers.get(&"w").unwrap(),
        *registers.get(&"x").unwrap(),
        *registers.get(&"y").unwrap(),
        *registers.get(&"z").unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alu_t_0() {
        let number = 13579246899999;
        let input = vec![
            "inp w".to_string(),
            "add z w".to_string(),
            "mod z 2".to_string(),
            "div w 2".to_string(),
            "add y w".to_string(),
            "mod y 2".to_string(),
            "div w 2".to_string(),
            "add x w".to_string(),
            "mod x 2".to_string(),
            "div w 2".to_string(),
            "mod w 2".to_string(),
        ];
        let (w, x, y, z) = alu(&input, number);
        println!("{} {} {} {}", w, x, y, z);
        assert_eq!(1, w);
        assert_eq!(1, x);
        assert_eq!(1, y);
        assert_eq!(1, z);
    }

    #[test]
    fn alu_t_1() {
        let number = 12345;
        let input = vec![
            "inp w",
            "inp x",
            "inp y",
            "inp z",
        ];
        let (w, x, y, z) = alu(&input.iter().map(|s| s.to_string()).collect(), number);
        println!("{} {} {} {}", w, x, y, z);
        assert_eq!(1, w);
        assert_eq!(2, x);
        assert_eq!(3, y);
        assert_eq!(4, z);
    }

    #[test]
    fn alu_t_2() {
        let number = 12345;
        let input = vec![
            "inp w",
            "inp x",
            "add w x",
            "inp z",
        ];
        let (w, x, y, z) = alu(&input.iter().map(|s| s.to_string()).collect(), number);
        println!("{} {} {} {}", w, x, y, z);
        assert_eq!(3, w);
        assert_eq!(2, x);
        assert_eq!(0, y);
        assert_eq!(3, z);
    }

    #[test]
    fn alu_t_3() {
        let number = 12345;
        let input = vec![
            "inp w",
            "inp x",
            "mul w x",
        ];
        let (w, x, y, z) = alu(&input.iter().map(|s| s.to_string()).collect(), number);
        println!("{} {} {} {}", w, x, y, z);
        assert_eq!(2, w);
        assert_eq!(2, x);
        assert_eq!(0, y);
        assert_eq!(0, z);
    }

    #[test]
    fn alu_t_4() {
        let number = 92;
        let input = vec![
            "inp w",
            "inp x",
            "div w x",
        ];
        let (w, x, y, z) = alu(&input.iter().map(|s| s.to_string()).collect(), number);
        println!("{} {} {} {}", w, x, y, z);
        assert_eq!(4, w);
        assert_eq!(2, x);
        assert_eq!(0, y);
        assert_eq!(0, z);
    }

    #[test]
    fn alu_t_5() {
        let number = 92;
        let input = vec![
            "inp w",
            "inp x",
            "mod w x",
        ];
        let (w, x, y, z) = alu(&input.iter().map(|s| s.to_string()).collect(), number);
        println!("{} {} {} {}", w, x, y, z);
        assert_eq!(1, w);
        assert_eq!(2, x);
        assert_eq!(0, y);
        assert_eq!(0, z);
    }

    #[test]
    fn alu_t_6() {
        let number = 9244;
        let input = vec![
            "inp w",
            "inp x",
            "eql w x",
            "inp y",
            "inp z",
            "eql y z",
        ];
        let (w, x, y, z) = alu(&input.iter().map(|s| s.to_string()).collect(), number);
        println!("{} {} {} {}", w, x, y, z);
        assert_eq!(0, w);
        assert_eq!(2, x);
        assert_eq!(1, y);
        assert_eq!(4, z);
    }

    #[test]
    fn alu_t_7() {
        let number = 9244;
        let input = vec![
            "inp w",
            "inp x",
            "add x -23",
        ];
        let (w, x, y, z) = alu(&input.iter().map(|s| s.to_string()).collect(), number);
        println!("{} {} {} {}", w, x, y, z);
        assert_eq!(9, w);
        assert_eq!(-21, x);
        assert_eq!(0, y);
        assert_eq!(0, z);
    }

    #[test]
    fn alu_t_8() {
        let number = 9;
        let filename = "src/day24/input.txt";
        let input = lines_from_file(filename);
        let blocks = extract_blocks(&input);
        let (w, x, y, z) = alu(blocks.get(0).unwrap(), number);
        println!("{} {} {} {}", w, x, y, z);
        assert_eq!(9, w);
        assert_eq!(1, x);
        assert_eq!(10, y);
        assert_eq!(10, z);
    }

    #[test]
    fn alu_t_9() {
        let filename = "src/day24/input.txt";
        let input = lines_from_file(filename);
        let blocks = extract_blocks(&input);
        for b in blocks {
            println!("{}, {}, {}", b.get(4).unwrap(), b.get(5).unwrap(), b.get(15).unwrap());
            println!("{}, {}, {}", b.get(3).unwrap(), b.get(4).unwrap(), b.get(14).unwrap());
            println!()
        }
        // let mut number = 0;
        // loop{
        //     let (w, x, y, z) = alu(blocks.last().unwrap(), number);
        //     if z == 0 {
        //         println!("{} {} {} {}", w, x, y, z);
        //         break;
        //     }
        //     number += 1;
        // }
    }

    #[test]
    fn extract_blocks_t() {
        let filename = "src/day24/input.txt";
        let input = lines_from_file(filename);
        let result = extract_blocks(&input);
        println!("{:?}", result);
        assert_eq!(14, result.len())
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day24/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_ne!(95649919999961, result);
        assert_eq!(true, 95649919999961 < result);
        assert_eq!(99299513899971, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day24/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(93185111127911, result);
    }
}

