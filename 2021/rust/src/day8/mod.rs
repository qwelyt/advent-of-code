use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::util::lines_from_file;

pub fn day8() {
    println!("== Day 8 ==");
    let input = lines_from_file("src/day8/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> i32 {
    let lines: Vec<Vec<&str>> = input.iter().map(|s| split_line(s)).collect();
    let values: Vec<&str> = lines.iter().map(|l| *l.get(1).unwrap()).collect();
    let numbers: Vec<i32> = values.iter().map(|s| count_known(*s)).collect();
    numbers.iter().sum()
}

fn split_line(line: &String) -> Vec<&str> {
    let vec = line.split(" | ").collect();
    vec
}

fn count_known(value_line: &str) -> i32 {
    let x: Vec<&str> = value_line.split(" ").filter(|s| proper_length(*s)).collect();
    x.len() as i32
}

fn proper_length(str: &str) -> bool {
    match str.len() {
        2 => true, // 1
        3 => true, // 7
        4 => true, // 4
        7 => true, // 8
        _ => false
    }
}

fn part_b(input: &Vec<String>) -> i32 {
    input.iter()
        .map(|l| decode_line(l.as_str()))
        .sum()
}


fn decode_line(line: &str) -> i32 {
    let line_split: Vec<&str> = (*line).split(" | ").collect();
    let wires = *line_split.get(0).unwrap();
    let digits = *line_split.get(1).unwrap();

    let wire_mapping: HashMap<String, i32> = map_wires(wires);

    decode_digits(wire_mapping, digits)
}

fn map_wires(input: &str) -> HashMap<String, i32> {
    let wires: Vec<&str> = input.split(" ").collect();
    let mut mapping: HashMap<String, i32> = HashMap::new();
    let mut mapping_inverse: HashMap<i32, &str> = HashMap::new();


    for w in wires.iter() {
        let wire = *w;
        match wire.len() {
            2 => {
                mapping.insert(wire.to_string(), 1);
                mapping_inverse.insert(1, wire);
            }
            3 => {
                mapping.insert(wire.to_string(), 7);
                mapping_inverse.insert(7, wire);
            }
            4 => {
                mapping.insert(wire.to_string(), 4);
                mapping_inverse.insert(4, wire);
            }
            7 => {
                mapping.insert(wire.to_string(), 8);
                mapping_inverse.insert(8, wire);
            }
            _ => {}
        }
    }


    let three_sub: String = subtract_segments(mapping_inverse.get(&8).unwrap(), mapping_inverse.get(&4).unwrap());
    let five_sub: String = subtract_segments(mapping_inverse.get(&4).unwrap(), mapping_inverse.get(&1).unwrap());

    mapping.insert(find_wire(&wires, three_sub.as_str(), 2), 2);
    let five: String = find_wire(&wires, five_sub.as_str(), 3);

    let segments_for_9 = add_segments(mapping_inverse.get(&1).unwrap(), &five.as_str());
    mapping.insert(find_correct_wire(&wires, segments_for_9), 9);

    let eight_minus_one = subtract_segments(mapping_inverse.get(&8).unwrap(), mapping_inverse.get(&1).unwrap());
    let segment_for_6 = add_segments(eight_minus_one.as_str(), &five.as_str());
    mapping.insert(find_correct_wire(&wires, segment_for_6), 6);
    mapping.insert(five, 5);

    for wire in wires {
        if !mapping.contains_key(wire) {
            match wire.len() {
                6 => mapping.insert(String::from(wire), 0),
                5 => mapping.insert(wire.to_string(), 3),
                _ => None
            };
        }
    }

    mapping
}

fn find_correct_wire(wires: &Vec<&str>, segments: String) -> String {
    let mut sc: Vec<char> = segments.chars().collect();
    sc.sort();

    for wire in wires {
        let mut chars: Vec<char> = (**wire).chars().collect();
        chars.sort();
        if chars == sc {
            return wire.to_string();
        }
    }
    panic!("Can't find correct wire for {} :: {:?}", segments, wires);
}


fn find_wire(wires: &Vec<&str>, sub_segments: &str, should_equal_len: usize) -> String {
    for wire in wires {
        if wire.len() == 5 {
            let segments = subtract_segments(*wire, sub_segments);
            // println!("{} :::: {}", wire ,segments);
            if segments.len() == should_equal_len {
                return String::from(*wire);
            }
        }
    }
    panic!("Could not find wire for length {}! {:?} :: {:?}", should_equal_len, wires, sub_segments)
}

fn subtract_segments(a: &str, b: &str) -> String {
    let av: Vec<char> = a.chars().collect();
    let bv: Vec<char> = b.chars().collect();
    let result: Vec<char> = av.iter().filter(|c| !bv.contains(*c)).map(|c| *c).collect();
    String::from_iter(result)
}

fn add_segments(a: &str, b: &str) -> String {
    let mut ao = a.to_string().to_owned();
    ao.push_str(b);
    let mut uniques = HashSet::new();
    ao.retain(|c| uniques.insert(c.clone()));

    ao
}

fn decode_digits(wire_mapping: HashMap<String, i32>, digits: &str) -> i32 {
    // println!("wm = {:?} ::: digits: {}",wire_mapping, digits);
    let keys = get_keys(wire_mapping.clone());
    let wires = keys.iter().map(|s| &**s).collect();
    let as_numbers: Vec<String> = (*digits).split(" ")
        .into_iter()
        // .map(|d| sort_string(d))
        .map(|s| find_correct_wire(&wires, s.to_string()))
        .map(|s| wire_mapping.get(s.as_str()).unwrap_or(&-1))
        .map(|i| i32::to_string(i))
        .collect();

    let number_string = String::from_iter(as_numbers);
    // println!("{}", number_string);
    i32::from_str(number_string.as_str()).unwrap()
}

fn get_keys(map: HashMap<String, i32>) -> Vec<String> {
    let mut ret = Vec::new();
    for (k, _) in map {
        ret.push(k);
    }
    ret
}

fn sort_string(str: &str) -> String {
    let mut x: Vec<char> = (*str).chars().collect::<Vec<char>>();
    x.sort();
    String::from_iter(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day8/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(26, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day8/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(247, result);
    }

    #[test]
    fn sort_string_t() {
        assert_eq!("abc", sort_string("cba"))
    }

    #[test]
    fn decode_digit_t() {
        let wire_mapping = HashMap::from([
            ("acb".to_string(), 7),
            ("cb".to_string(), 1),
            ("gfabdec".to_string(), 8),
            ("dcba".to_string(), 4),
            ("gfabd".to_string(), 5)
        ]);
        let digits = "acb dcba gfabdec cb";
        assert_eq!(7481, decode_digits(wire_mapping, digits))
    }

    #[test]
    fn subtract_segment_t() {
        let a = "fbcad";
        let b = "cdg";
        assert_eq!("fba", subtract_segments(a, b))
    }

    #[test]
    fn add_segment_t() {
        let a = "abc";
        let b = "adgcf";
        assert_eq!("abcdgf", add_segments(a, b))
    }

    #[test]
    fn find_correct_wire_t() {
        let strings = vec![
            "abc",
            "fdga",
            "cdbaf",
        ];

        assert_eq!("fdga", find_correct_wire(&strings, "adgf".to_string()))
    }

    #[test]
    fn map_wires_t() {
        let expected = HashMap::from([
            ("acedgfb".to_string(), 8),
            ("cdfbe".to_string(), 5),
            ("gcdfa".to_string(), 2),
            ("fbcad".to_string(), 3),
            ("dab".to_string(), 7),
            ("cefabd".to_string(), 9),
            ("cdfgeb".to_string(), 6),
            ("eafb".to_string(), 4),
            ("cagedb".to_string(), 0),
            ("ab".to_string(), 1)
        ]);
        let wires = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        assert_eq!(expected, map_wires(wires))
    }

    #[test]
    fn decode_line_t() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let result = decode_line(input);
        assert_eq!(5353, result)
    }


    #[test]
    fn part_b_test_input() {
        let filename = "src/day8/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(61229, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day8/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(933305, result);
    }
}