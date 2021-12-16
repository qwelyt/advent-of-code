use crate::util::lines_from_file;

pub fn day16() {
    println!("== Day 16 ==");
    let input = lines_from_file("src/day16/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> u32 {
    let packet = input.get(0).unwrap();
    // println!("{}", packet);
    let binary = to_binary(packet);
    // println!("{:?}", binary);
    let (versions, data) = parse(&binary, None);
    versions.iter().sum()
}

fn parse(binary: &Vec<char>, operations: Option<u32>) -> (Vec<u32>, Vec<char>) {
    // println!("Received: {:?} ::({}) {:?}", operations, binary.len(), binary);
    if binary.iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>() == 0 {
        return (Vec::new(), Vec::new());
    }

    let mut versions: Vec<u32> = Vec::new();

    let mut data = binary.clone();


    let mut ops = 0;
    while data.len() > 0 && ops < operations.unwrap_or(u32::MAX) {
        if data.iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>() == 0 {
            break;
        }
        // println!("## Data is: {:?}", data);
        let version = u32::from_str_radix(data[0..3].to_vec().iter().collect::<String>().as_str(), 2).unwrap();
        let type_id = u32::from_str_radix(data[3..6].to_vec().iter().collect::<String>().as_str(), 2).unwrap();
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
                //     let number = u32::from_str_radix(n.iter().collect::<String>().as_str(), 2).unwrap();
                //     println!("{:?} = {}", n, number);
                // }
                // println!("======  Left: {:?}", left);
                // TODO: Do something with numbers later maybe?
                data = left.iter().flatten().map(|c| *c).collect();
            }
            _ => {
                // Operator
                let length = if data[0] == '0' { 15 } else { 11 };
                data = data.split_at(1).1.to_vec();
                // println!("Operator, length {}", length);

                let number_bits = data[0..length].to_vec();
                let number = u32::from_str_radix(number_bits.to_vec().iter().collect::<String>().as_str(), 2).unwrap();

                // println!("Number is {}", number);

                data = data.split_at(length).1.to_vec();

                if length == 15 {
                    // sub_package says the next 'number' of bits is the sub_pkg
                    let sub_packages = data[0..number as usize].to_vec();
                    let (mut sub_versions, _left) = parse(&sub_packages, None);
                    versions.append(&mut sub_versions);
                    // println!("15 left :: {} {:?}", _left.len(), _left);

                    data = data.split_at(number as usize).1.to_vec();
                } else if length == 11 {
                    // number of sub packages contained by this package
                    // println!("11 ||| n: {}  d: {:?}", number, data);
                    let (mut sub_versions, left) = parse(&data, Some(number));
                    versions.append(&mut sub_versions);
                    data = left.clone();
                }
            }
        }
    }

    // println!("Versions: {:?}", versions);
    // println!("Data: {:?}", data);
    // println!("Ops: {}", ops);

    (versions, data.to_vec())
}
// fn parse_to_decimal(binary: &Vec<char>) -> u32 {
//     if binary.iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>() == 0 {
//         return 0;
//     }
//     let version = u32::from_str_radix(binary[0..3].to_vec().iter().collect::<String>().as_str(), 2).unwrap();
//     let type_id = u32::from_str_radix(binary[3..6].to_vec().iter().collect::<String>().as_str(), 2).unwrap();
//     println!("Version: {:?}", version);
//     println!("type: {:?}", type_id);
//     let data = binary.split_at(6).1;
//
//     // keep mut vec with all the data left in binary
//     // When we find an operator, take out the sub-vec needed
//     // for that operation and call this method with the sub
//     // How to deal with operator length = 11? Then we specify
//     // how many operations in the substring we want to find
//     // So send in a Option<u32> for it and count number of
//     // operations up to that? INF if None.
//     // The return value of this function need to contain a list
//     // of all versions. The caller can then sum the versions.
//     // Also need to return the bits not processed yet.
//     // So a (Vec<u32>, Vec<char>) return? Will probably have to expand for p2.
//     // And a (binary: &Vec<char>, operations: Option<u32>) header?
//     match type_id {
//         4 => {
//             // Literal value
//             // let chunks = data.chunks(5).map(|c| c.to_vec().iter().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
//             let mut numbers: Vec<Vec<char>> = Vec::new();
//             let mut left : Vec<Vec<char>> = Vec::new();
//             let mut done = false;
//             for c in data.chunks(5) {
//                 println!("{:?}", c);
//                 if c[0] == '0' {
//                     if c.len() == 5 && !done{
//                         done = true;
//                         println!("Yo I'm done!");
//                         numbers.push(c[1..5].to_vec());
//                         continue;
//                     } else {
//                         left.push(c.to_vec())
//                     }
//
//                 } else {
//                     if !done {
//                         numbers.push(c[1..5].to_vec());
//                     } else {
//                         left.push(c.to_vec());
//                     }
//                 }
//             }
//             for n in numbers.iter() {
//                 let number = u32::from_str_radix(n.iter().collect::<String>().as_str(), 2).unwrap();
//                 println!("{:?} = {}", n, number);
//             }
//             let flat_left = left.into_iter().flatten().collect::<Vec<char>>();
//             println!("left:{} :: {:?}", flat_left.len(), flat_left);
//
//             let big_num: Vec<char> = numbers.into_iter().flatten().collect::<Vec<char>>();
//             let number = u32::from_str_radix(big_num.iter().collect::<String>().as_str(), 2).unwrap();
//             println!("{:?} = {}", big_num, number);
//             parse_to_decimal(&flat_left);
//             return number;
//         }
//         _ => {
//             // Operator
//             let length = if data[0] == '0' { 15 } else { 11 };
//             let sub_package = data.split_at(1).1[0..length].to_vec();
//             println!("{:?} ::: {}", sub_package,sub_package.len());
//             let number = u32::from_str_radix(sub_package.to_vec().iter().collect::<String>().as_str(), 2).unwrap();
//             println!("{}", number);
//             let pap = data.split_at(length+1).1.to_vec();
//             println!("{:?} :: {}", pap, pap.len());
//             if length == 15 {
//                 // sub_package is this long
//                 parse_to_decimal(&pap);
//             } else if length == 11{
//                 // number of sub packages contained by this package
//                 parse_to_decimal(&pap);
//             }
//         }
//     }
//
//     0
// }

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


fn part_b(input: &Vec<String>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_decimal_t() {
        let binary = "110100101111111000101000".chars().collect();
        let x: u32 = parse(&binary, None).0.to_vec().into_iter().sum();
        assert_eq!(2021, x)
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
}