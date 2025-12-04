use crate::util::time;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn solve() {
    println!("== Day 19 ==");
    let input = "src/day19/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> u32 {
    let mut linen = Linen::parse(input);
    // println!("towels: {:?}", linen.towels);
    // println!("patterns: {:?}", linen.patterns);
    linen.possible_pattens()
}

fn part_b(input: &str) -> usize {
    let mut linen = Linen::parse(input);
    linen.all_possible_arrangements()
}

struct Linen {
    towels: Vec<String>,
    patterns: Vec<String>,
    cache: HashMap<String, bool>,
    num_cache: HashMap<String, usize>,
}

impl Linen {
    fn parse(input: &str) -> Self {
        let string = read_to_string(input.to_string()).expect("failed to read input");
        let (towels_str, pattens_str) = string.split_once("\n\n")
            .or_else(|| string.split_once("\r\n"))
            .unwrap();
        let towels = towels_str
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let patterns = pattens_str.lines().map(|s| s.to_string()).collect();
        Self {
            towels,
            patterns,
            cache: HashMap::new(),
            num_cache: HashMap::new(),
        }
    }

    fn possible_pattens(&mut self) -> u32 {
        let mut count = 0;
        for pattern in self.patterns.clone().iter() {
            if self.possible(pattern.as_str()) {
                count += 1;
            }
        }
        count
    }

    fn possible(&mut self, pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }
        if self.cache.contains_key(pattern) {
            return *(self.cache.get(pattern).unwrap());
        }
        for towel in self.towels.clone().iter() {
            if pattern.starts_with(towel) {
                if self.possible(pattern.strip_prefix(towel).unwrap()) {
                    self.cache.insert(pattern.to_string(), true);
                    return true;
                }
            }
        }
        false
    }

    fn all_possible_arrangements(&mut self) -> usize {
        let mut count = 0;
        for pattern in self.patterns.clone().iter() {
            count += self.possibilities(pattern.as_str());
        }
        count
    }

    fn possibilities(&mut self, pattern: &str) -> usize {
        if pattern.is_empty() { return 1; }
        if self.num_cache.contains_key(pattern) {
            return *self.num_cache.get(pattern).unwrap();
        }
        let mut count = 0;
        for towel in self.towels.clone().iter() {
            if pattern.starts_with(towel) {
                let i = self.possibilities(pattern.strip_prefix(towel).unwrap());
                count += i;
                self.num_cache.insert(pattern.to_string(), count);
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn run_day() {
        solve();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day19/input.txt";
        assert_eq!(228, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day19/input.txt";
        assert_eq!(584553405070389, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day19/test-input.txt";
        let result = part_a(input);
        assert_eq!(6, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day19/test-input.txt";
        let result = part_b(input);
        assert_eq!(16, result);
    }
}
