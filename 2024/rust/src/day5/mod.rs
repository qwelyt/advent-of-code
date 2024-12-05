use crate::util::time;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 5 ==");
    let input = "src/day5/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn read_file(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut rules = HashMap::new();

    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut part_a = true;
    for line in File::open(input)
        .map(|f| BufReader::new(f).lines().flatten())
        .unwrap()
    {
        if line.is_empty() {
            part_a = false;
            continue;
        }
        match part_a {
            true => a.push(line),
            false => b.push(line),
        }
    }

    for line in a.iter() {
        let split = line.split("|").collect::<Vec<&str>>();
        let key = split.get(0).unwrap().trim().parse::<u32>().unwrap();
        let value = split.get(1).unwrap().trim().parse::<u32>().unwrap();
        rules.entry(key).or_insert_with(HashSet::new).insert(value);
    }

    let updates = b.iter()
        .map(|line| line.split(",")
            .filter(|&x| !x.is_empty())
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    (rules, updates)
}

fn part_a(input: &str) -> u32 {
    let (rules, updates) = read_file(input);

    // println!("{:?}", rules);
    // println!();
    // println!("{:?}", updates);

    let mut middle_pages: Vec<u32> = Vec::new();

    updates.iter().for_each(|update| {
        for (idx, page) in update.iter().enumerate() {
            if let Some(rule) = rules.get(page) {
                let already_printed: HashSet<u32> = HashSet::from_iter(update.split_at(idx).0.iter().cloned());
                if !already_printed.is_disjoint(rule) {
                    //println!("Break: idx:{:?} page:{:?}  {:?}  {:?}    {:?}", idx, page, x, rule, already_printed);
                    break; // A page has already been printed
                }
            }
            if idx == update.len() - 1 {
                let middle_page = update.get(update.len() / 2);
                middle_pages.push(*middle_page.unwrap());
            }
        }
    });
    // println!("{:?}", middle_pages);

    middle_pages.iter().sum()
}

fn part_b(input: &str) -> u32 {
    let (rules, updates) = read_file(input);

    let mut middle_pages: Vec<u32> = Vec::new();

    updates.iter().for_each(|update| {
        for (idx, page) in update.iter().enumerate() {
            if let Some(rule) = rules.get(page) {
                let set: HashSet<u32> = HashSet::from_iter(update.split_at(idx).0.iter().map(|v| *v));
                if !set.is_disjoint(rule) {
                    let corrected_update: Vec<u32> = correct_update(update, &rules);
                    let middle_page = corrected_update.get(corrected_update.len() / 2);
                    middle_pages.push(*middle_page.unwrap());
                    break; // A page has already been printed
                }
            }
        }
    });
    // println!("{:?}", middle_pages);

    middle_pages.iter().sum()
}

fn correct_update(update: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    // println!("Need to correct update: {:?}", update);
    let mut corrected : Vec<u32> = Vec::new();

    for page in update {
        if let Some(rule) = rules.get(page) {
            let broken_rules: Vec<_> = rule.iter().filter(|r| corrected.contains(r)).collect::<Vec<&u32>>();
            //println!("x={:?}, page={:?}, rule: {:?}, corrected: {:?}", broken_rules, page, rule, corrected);
            if broken_rules.is_empty() {
                corrected.push(*page);
            } else {
                if let Some(index) = broken_rules.iter().map(|v| corrected.iter().position(|a| a==*v)).flatten().min() {
                    corrected.insert(index, *page);
                } else {
                    corrected.push(*page);
                }
            }
        } else {
            corrected.push(*page);
        }
    }

    // println!("Corrected: {:?}", corrected);
    corrected
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
        let input = "src/day5/input.txt";
        assert_eq!(4766, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day5/input.txt";
        assert_eq!(6257, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day5/test-input.txt";
        let result = part_a(input);
        assert_eq!(143, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day5/test-input.txt";
        let result = part_b(input);
        assert_eq!(123, result);
    }
}
