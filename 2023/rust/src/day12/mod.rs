use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day12() {
    println!("== Day 12 ==");
    let input = "src/day12/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let file = File::open(input).unwrap();
    BufReader::new(file).lines()
        .flatten()
        .map(|l| combos_for_line(l.as_str(), 1))
        .sum()
}

fn part_b(input: &str) -> usize { 0 }


fn is_valid(combo: &Vec<char>, blocks: &Vec<usize>) -> bool {
    let mut current = 0;
    let mut seen = Vec::new();
    for &c in combo.iter() {
        if c == '.' {
            if current > 0 {
                seen.push(current);
            }
            current = 0; // Block of damaged springs has ended.
        } else if c == '#' {
            current += 1; // We found another damaged spring
        }
    }
    if current > 0 {
        seen.push(current); // If we end on damaged springs we need to make sure we count them
    }
    seen.eq(blocks)
}

fn find_combos(combo: &Vec<char>, blocks: &Vec<usize>, num: usize) -> usize {
    if num == combo.len() {
        return if is_valid(combo, blocks) {
            1
        } else {
            0
        };
    }
    if combo[num] == '?' {
        let mut cloned = combo.clone();
        cloned[num] = '#';
        let with_damaged = find_combos(&cloned, blocks, num + 1);
        cloned[num] = '.';
        let with_working = find_combos(&cloned, blocks, num + 1);
        return with_damaged + with_working;
    }
    find_combos(combo, blocks, num + 1)
}

fn combos_for_line(line: &str, repeats: usize) -> usize {
    let (springs, numbers) = line.split_once(" ").unwrap();
    let springs = springs.chars().collect::<Vec<char>>();
    let numbers = numbers.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut combo = Vec::new();
    let mut blocks = Vec::new();
    for i in 0..repeats {
        if i > 0 {
            combo.push('?');
        }
        combo.extend(&springs);
        blocks.extend(&numbers);
    }
    find_combos(&combo, &blocks, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day12();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day12/input.txt";
        assert_eq!(7718, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day12/input.txt";
        assert_eq!(0, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day12/test-input.txt";
        let result = part_a(input);
        assert_eq!(21, result);
    }


    #[test]
    fn test_validity() {
        assert_eq!(true, is_valid(&"#.#.###".chars().collect::<Vec<char>>(), &vec![1, 1, 3]));
        assert_eq!(true, is_valid(&"###....##.#".chars().collect::<Vec<char>>(), &vec![3, 2, 1]));
        assert_eq!(true, is_valid(&".###.##....#".chars().collect::<Vec<char>>(), &vec![3, 2, 1]));
        assert_eq!(false, is_valid(&"####.##....#".chars().collect::<Vec<char>>(), &vec![3, 2, 1]));
    }

    #[test]
    fn combos_test() {
        {
            let line = "???.### 1,1,3";
            let result = combos_for_line(line, 1);
            assert_eq!(1, result)
        }
        {
            let line = ".??..??...?##. 1,1,3";
            let result = combos_for_line(line, 1);
            assert_eq!(4, result)
        }
        {
            let line = "?###???????? 3,2,1";
            let result = combos_for_line(line, 1);
            assert_eq!(10, result)
        }
        {
            let line = "???.### 1,1,3";
            let result = combos_for_line(line, 5);
            assert_eq!(1, result)
        }
        {
            let line = "?###???????? 3,2,1";
            let result = combos_for_line(line, 5);
            assert_eq!(506250, result)
        }
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day12/test-input.txt";
        let result = part_b(input);
        assert_eq!(525152, result);
    }
}