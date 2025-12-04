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
    let (poop, _iter) = solve(&binary, 0);
    // println!("{:?}", poop);
    poop.versions.iter().sum()
}

fn part_b(input: &Vec<String>) -> u64 {
    let packet = input.get(0).unwrap();
    // println!("{}", packet);
    let binary = to_binary(packet);
    // println!("{} bits", binary.len());
    // println!("{:?}", binary);
    let (poop, _iter) = solve(&binary, 0);
    // println!("{:?}", poop);
    return *poop.numbers.get(0).unwrap()
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
#[derive(Debug)]
struct Poop {
    bits_drained: usize,
    numbers: Vec<u64>,
    versions: Vec<u32>,
}

fn solve(binary: &Vec<char>, iteration: u32) -> (Poop, u32) {
    let mut bits = binary.clone();
    let version = u32::from_str_radix(bits.drain(0..3).collect::<String>().as_str(), 2).unwrap();
    let type_id = u32::from_str_radix(bits.drain(0..3).collect::<String>().as_str(), 2).unwrap();
    // println!("Iteration: {} :: Version: {} :: Type_id: {} ::: Bin/bit {}/{}", iteration, version, type_id, binary.len() ,bits.len());
    let mut bits_drained = 6; // Version and type_id
    let mut versions = vec![version];

    return match type_id {
        4 => {
            let mut numbers: Vec<Vec<char>> = Vec::new();
            // println!("Bits: ({}) {:?}", bits.len(), bits);
            loop {
                let mut n = bits.drain(0..5).collect::<Vec<char>>();
                let head = n.drain(0..1).collect::<Vec<char>>()[0];
                numbers.push(n.clone());
                if head == '0' { break; }
            }
            bits_drained += numbers.len();
            let big_num: Vec<char> = numbers.into_iter().flatten().collect::<Vec<char>>();
            bits_drained += big_num.len();
            let number = u64::from_str_radix(big_num.iter().collect::<String>().as_str(), 2).unwrap();
            // println!("I found {} after draining {} from ({}){:?}", number, bits_drained,binary.len(), binary);
            // println!("bd: i {} {}", iteration, bits_drained);
            (Poop {
                bits_drained,
                numbers: vec![number],
                versions,
            }, iteration)
        }
        _ => {
            let length = if *bits.get(0).unwrap() == '0' { 15 } else { 11 };
            bits.drain(0..1);
            bits_drained += 1;

            // println!("Binary: {} {:?}",binary.len(), binary);
            // println!("Bits: {} {:?}", bits.len(), bits);
            let number = u32::from_str_radix(bits.drain(0..length).collect::<String>().as_str(), 2).unwrap();
            // println!("Number: {}", number);
            // println!("Length: {}", length);
            bits_drained += length;

            let mut numbers = Vec::new();
            if length == 15 {
                // If the length type ID is 0, then the next 15 bits are a
                // number that represents the total length in bits of the
                // sub-packets contained by this packet.
                let mut sub_pkg = bits.drain(0..number as usize).collect::<Vec<char>>();
                bits_drained += number as usize;
                // println!("L15 Solve for: ({}) {:?}",sub_pkg.len(), sub_pkg);
                let mut drained = 0;
                while drained < number as usize {
                    let (poop, _iter) = solve(&sub_pkg, iteration + 1);
                    // println!("L15 poop (i {}): {:?}", _iter, poop);
                    drained += poop.bits_drained;
                    sub_pkg.drain(0..poop.bits_drained);
                    numbers.append(&mut poop.numbers.clone());
                    versions.append(&mut poop.versions.clone());
                }
            } else {
                // Length 11:
                // If the length type ID is 1, then the next 11 bits are a
                // number that represents the number of sub-packets immediately
                // contained by this packet.
                let mut sub_pkg_done = 0;
                while sub_pkg_done < number {
                    // println!("L11 Solve for: ({}) {:?}", bits.len(), bits);
                    let (poop, _iter) = solve(&bits.clone(), iteration + 1);
                    // println!("L11 poo (i {})p: {:?} ::: ({}) {:?}", _iter, poop, bits.len(), bits);
                    numbers.append(&mut poop.numbers.clone());
                    versions.append(&mut poop.versions.clone());
                    bits.drain(0..poop.bits_drained);
                    bits_drained += poop.bits_drained;
                    sub_pkg_done += 1;
                }
            }
            // println!("bd: i {} {}", iteration, bits_drained);
            (Poop {
                bits_drained,
                numbers: vec![calc(numbers, type_id)],
                versions,
            }, iteration)
        }
    };
}


fn calc(values: Vec<u64>, type_id: u32) -> u64 {
    match type_id {
        0 => values.into_iter().sum(), // 0 = Sum
        1 => values.into_iter().product(), // 1 = Product
        2 => values.into_iter().min().unwrap(), // 2 = min
        3 => values.into_iter().max().unwrap(), // 3 = max
        5 => { if values[0] > values[1] { 1 } else { 0 } }, // 5 = packet[0] > packet[1] (greater than)
        6 => { if values[0] < values[1] { 1 } else { 0 } }, // 6 = packet[0] < packet[1] (less than)
        7 => { if values[0] == values[1] { 1 } else { 0 } }, // 7 = packet[0] == packet[1] (equal)
        _ => { panic!("Got {} as type_id!", type_id) }
    }
}

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
        let (poop, _iter) = solve(&binary, 0);
        assert_eq!(2021, *poop.numbers.get(0).unwrap())
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
        assert_eq!(54, result)
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day16/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_ne!(10760589785870465, result);
        assert_eq!(true, result < 10760589785870465);
        assert_ne!(680615, result);
        assert_eq!(true, result > 680615);
        assert_ne!(319751467161448, result);
        assert_eq!(true, 319751467161448 > result);
        assert_eq!(194435634456, result);
    }


    #[test]
    fn solve_t_1() {
        let input = "C200B40A82".to_string();
        let binary = to_binary(&input);
        // println!("{:?}", binary);
        let (result, _iter) = solve(&binary, 0);

        assert_eq!(3, *result.numbers.get(0).unwrap())
    }

    #[test]
    fn solve_t_2() {
        let input = "EE00D40C823060".to_string();
        let binary = to_binary(&input);
        // println!("{:?}", binary);
        let (result, _iter) = solve(&binary, 0);

        assert_eq!(3, *result.numbers.get(0).unwrap())
    }
}