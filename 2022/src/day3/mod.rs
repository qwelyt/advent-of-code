use std::collections::HashSet;

use crate::util::{lines, time};

pub fn day3() {
    println!("== Day 3 ==");
    let input = "src/day3/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> i32 {
    lines(input).iter()
        .map(|line| find_duplicate(line))
        .map(|c| map_to_value(c))
        .sum()
}

fn find_duplicate(line: &String) -> char {
    let (a, b) = line.split_at(line.len() / 2);
    let a: HashSet<char> = a.chars().collect();
    let b: HashSet<char> = b.chars().collect();
    for c in a.intersection(&b) {
        return *c;
    }
    ' '
}

fn map_to_value(c: char) -> i32 {
    if c.is_uppercase() {
        return (c as u32 - 'A' as u32) as i32 + 27;
    }
    (c as u32 - 'a' as u32) as i32 + 1
}


fn part_b(input: &str) -> i32 {
    let mut tot = 0;
    let mut group = Vec::new();
    let lines = lines(input);
    for line in lines.iter() {
        group.push(line.as_str());
        if group.len() == 3 {
            let duplicate = find_group_duplicate(&group);
            tot += map_to_value(duplicate);
            group.clear();
        }
    }
    tot
}

fn find_group_duplicate(group: &Vec<&str>) -> char {
    let chars: Vec<HashSet<char>> = group.iter()
        .map(|s| *s)
        .map(|s| s.chars().collect::<HashSet<char>>())
        .collect();

    let mut s = chars[0].clone();
    for set in chars {
        s = s.intersection(&set).cloned().collect();
    }
    for c in s { return c; }
    ' '
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day3();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day3/input.txt";
        assert_eq!(7727, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day3/input.txt";
        assert_eq!(2609, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day3/test-input.txt";
        let result = part_a(input);
        assert_eq!(157, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day3/test-input.txt";
        let result = part_b(input);
        assert_eq!(70, result);
    }
}