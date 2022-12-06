use std::collections::{HashSet, VecDeque};

use crate::util::{lines, time};

pub fn day6() {
    println!("== Day 6 ==");
    let input = "src/day6/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    find_seq(lines(input)[0].as_str(), 4)
}

fn find_seq(input: &str, size: usize) -> usize {
    let mut seq = VecDeque::with_capacity(size);
    for (i, c) in input.chars().enumerate() {
        seq.push_back(c);
        if seq.len() == size {
            if seq.iter().collect::<HashSet<&char>>().len() == size {
                return i + 1;
            } else {
                seq.pop_front();
            }
        }
    }
    0
}

fn part_b(input: &str) -> usize {
    find_seq(lines(input)[0].as_str(), 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day6();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day6/input.txt";
        assert_eq!(1210, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day6/input.txt";
        assert_eq!(3476, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day6/test-input.txt";
        let result = part_a(input);
        assert_eq!(7, result);
    }

    #[test]
    fn find_seq_test() {
        assert_eq!(5, find_seq("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, find_seq("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, find_seq("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, find_seq("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn part_b_test_input() {
        assert_eq!(19, find_seq("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, find_seq("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, find_seq("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, find_seq("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, find_seq("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }
}