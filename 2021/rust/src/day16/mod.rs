use std::usize;

use crate::util::lines_from_file;

mod other;

pub fn day16() {
    println!("== Day 16 ==");
    let input = lines_from_file("src/day16/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> usize {
    let packet = input.get(0).unwrap();
    // println!("{}", packet);
    let binary = to_binary(packet);
    // println!("{:?}", binary);
    let (versions, _data, _value) = parse(&binary, None);
    versions.iter().sum()
}

fn part_b(input: &Vec<String>) -> u128 {
    let packet = input.get(0).unwrap();
    // println!("{}", packet);
    let binary = to_binary(packet);
    // println!("{:?}", binary);
    let (_versions, _data, value) = parse(&binary, None);

    return *value.get(0).unwrap();
}


fn parse(binary: &Vec<char>, operations: Option<usize>) -> (Vec<usize>, Vec<char>, Vec<u128>) {
    // println!("Received: {:?} ::({}) {:?}", operations, binary.len(), binary);
    if binary.iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>() == 0 {
        return (Vec::new(), Vec::new(), Vec::new());
    }

    let mut versions: Vec<usize> = Vec::new();

    let mut data = binary.clone();


    let mut ops = 0;
    let mut values: Vec<u128> = Vec::new();
    let f: fn(Vec<u128>, usize) -> u128 = |v: Vec<u128>, type_id: usize| {
        match type_id {
            0 => v.into_iter().fold(0, |acc, add| acc + add),
            1 => v.into_iter().fold(1, |acc, add| acc * add),
            2 => v.into_iter().fold(u128::MAX, |a, b| std::cmp::min(a, b)),
            3 => v.into_iter().fold(0, |a, b| std::cmp::max(a, b)),
            5 => (v[0] > v[1]) as u128,
            6 => (v[0] < v[1]) as u128,
            7 => (v[0] == v[1]) as u128,
            _ => { 0 }
        }
    };
    while data.len() > 0 && ops < operations.unwrap_or(usize::MAX) {
        if data.iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>() == 0 {
            break;
        }
        // println!("## Data is: {:?}", data);
        let version = usize::from_str_radix(data[0..3].to_vec().iter().collect::<String>().as_str(), 2).unwrap();
        let type_id = usize::from_str_radix(data[3..6].to_vec().iter().collect::<String>().as_str(), 2).unwrap();
        data = data.split_at(6).1.to_vec();
        // println!("## Version: {:?}", version);
        // println!("## type: {:?}", type_id);

        versions.push(version);
        match type_id {
            4 => {
                // Literal value
                ops += 1;
                let mut numbers: Vec<Vec<char>> = Vec::new();
                let mut left: Vec<Vec<char>> = Vec::new();
                let mut done = false;
                for c in data.as_slice().chunks(5) {
                    if c[0] == '0' {
                        if c.len() == 5 && !done {
                            done = true;
                            numbers.push(c[1..5].to_vec());
                            continue;
                        } else {
                            left.push(c.to_vec())
                        }
                    } else {
                        if !done {
                            numbers.push(c[1..5].to_vec());
                        } else {
                            left.push(c.to_vec());
                        }
                    }
                }
                // println!("====== Numbers: {:?}", numbers);
                // for n in numbers.iter() {
                //     let number = usize::from_str_radix(n.iter().collect::<String>().as_str(), 2).unwrap();
                //     println!("{:?} = {}", n, number);
                // }
                // println!("======  Left: {:?}", left);
                let big_num: Vec<char> = numbers.into_iter().flatten().collect::<Vec<char>>();
                let number = usize::from_str_radix(big_num.iter().collect::<String>().as_str(), 2).unwrap();
                values.push(number as u128);
                data = left.iter().flatten().map(|c| *c).collect();
            }
            _ => {
                println!("## type: {:?}", type_id);
                // Operator
                let length = if data[0] == '0' { 15 } else { 11 };
                data = data.split_at(1).1.to_vec();
                // println!("Operator, length {}", length);

                let number_bits = data[0..length].to_vec();
                let number = usize::from_str_radix(number_bits.to_vec().iter().collect::<String>().as_str(), 2).unwrap();

                // println!("Number is {}", number);

                data = data.split_at(length).1.to_vec();

                if length == 15 {
                    // sub_package says the next 'number' of bits is the sub_pkg
                    let sub_packages = data[0..number as usize].to_vec();
                    let (mut sub_versions, _left, sub_values) = parse(&sub_packages, None);
                    versions.append(&mut sub_versions);
                    // println!("15 left :: {} {:?}", _left.len(), _left);
                    let v = f(sub_values, type_id);
                    println!("o: {} ::: v: {}", type_id, v);
                    values.push(v);

                    data = data.split_at(number as usize).1.to_vec();
                } else if length == 11 {
                    // number of sub packages contained by this package
                    // println!("11 ||| n: {}  d: {:?}", number, data);
                    let (mut sub_versions, left, sub_values) = parse(&data, Some(number));
                    versions.append(&mut sub_versions);
                    let v = f(sub_values, type_id);
                    println!("o: {} ::: v: {}", type_id, v);
                    values.push(v);
                    data = left.clone();
                }
            }
        }
    }

    // println!("Versions: {:?}", versions);
    // println!("Data: {:?}", data);
    // println!("Ops: {}", ops);
    // println!("Value: {:?}", values);

    (versions, data.to_vec(), values)
}
//     // keep mut vec with all the data left in binary
//     // When we find an operator, take out the sub-vec needed
//     // for that operation and call this method with the sub
//     // How to deal with operator length = 11? Then we specify
//     // how many operations in the substring we want to find
//     // So send in a Option<usize> for it and count number of
//     // operations up to that? INF if None.
//     // The return value of this function need to contain a list
//     // of all versions. The caller can then sum the versions.
//     // Also need to return the bits not processed yet.
//     // So a (Vec<usize>, Vec<char>) return? Will probably have to expand for p2.
//     // And a (binary: &Vec<char>, operations: Option<usize>) header?

fn to_binary(hex: &String) -> Vec<char> {
    hex.chars()
        .into_iter()
        .map(htob)
        .map(|s| s.chars())
        .flatten()
        .collect()
}

fn htob(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_decimal_t() {
        let binary = "110100101111111000101000".chars().collect();
        let (a, b, c) = parse(&binary, None);
        assert_eq!(2021, *c.get(0).unwrap())
    }

    #[test]
    fn parse_1() {
        let input = vec!["38006F45291200".to_string()];
        assert_eq!(9, part_a(&input))
    }

    #[test]
    fn parse_2() {
        let input = vec!["EE00D40C823060".to_string()];
        assert_eq!(14, part_a(&input))
    }

    #[test]
    fn test_input_1() {
        let input = vec!["8A004A801A8002F478".to_string()];
        assert_eq!(16, part_a(&input))
    }

    #[test]
    fn test_input_2() {
        let input = vec!["620080001611562C8802118E34".to_string()];
        assert_eq!(12, part_a(&input))
    }

    #[test]
    fn test_input_3() {
        let input = vec!["C0015000016115A2E0802F182340".to_string()];
        assert_eq!(23, part_a(&input))
    }

    #[test]
    fn part_a_test_input() {
        let filename = "src/day16/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(31, result)
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day16/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(877, result)
    }


    #[test]
    fn test_input_b_1() {
        let input = vec!["C200B40A82".to_string()];
        assert_eq!(3, part_b(&input));
    }

    #[test]
    fn test_input_b_2() {
        let input = vec!["04005AC33890".to_string()];
        assert_eq!(54, part_b(&input));
    }

    #[test]
    fn test_input_b_3() {
        let input = vec!["880086C3E88112".to_string()];
        assert_eq!(7, part_b(&input));
    }

    #[test]
    fn test_input_b_4() {
        let input = vec!["CE00C43D881120".to_string()];
        assert_eq!(9, part_b(&input));
    }

    #[test]
    fn test_input_b_5() {
        let input = vec!["D8005AC2A8F0".to_string()];
        assert_eq!(1, part_b(&input));
    }

    #[test]
    fn test_input_b_6() {
        let input = vec!["F600BC2D8F".to_string()];
        assert_eq!(0, part_b(&input));
    }

    #[test]
    fn test_input_b_7() {
        let input = vec!["9C005AC2F8F0".to_string()];
        assert_eq!(0, part_b(&input));
    }

    #[test]
    fn test_input_b_8() {
        let input = vec!["9C0141080250320F1802104A08".to_string()];
        assert_eq!(1, part_b(&input));
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day16/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(31, result)
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day16/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_ne!(10760589785870465, result)
    }
}