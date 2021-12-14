use std::collections::{HashMap, VecDeque};

use crate::util::lines_from_file;

pub fn day14() {
    println!("== Day 14 ==");
    let input = lines_from_file("src/day14/input.txt");
    let a = part_a(&input, 10);
    println!("Part A: {}", a);
    let b = part_b(&input, 40);
    println!("Part B: {}", b);
}

struct Data {
    template: String,
    rules: HashMap<(char, char), char>,
}

fn part_a(input: &Vec<String>, steps: i32) -> usize {
    let data = to_data(input);
    let mut template = data.template.chars().collect::<Vec<char>>();
    for _ in 0..steps {
        let mut insertions: VecDeque<(usize, char)> = VecDeque::new();
        for (index, c) in template.iter().enumerate() {
            if index + 1 == template.len() {
                continue;
            }
            let next = *template.get(index + 1).unwrap();
            let insert = data.rules.get(&(*c, next)).unwrap();
            insertions.push_back((index + 1, *insert));
        }

        while !insertions.is_empty() {
            let (index, c) = insertions.pop_back().unwrap();
            template.insert(index, c);
        }
    }
    let mut count: HashMap<char, usize> = HashMap::new();
    for c in template.iter() {
        *count.entry(*c).or_default() += 1;
    }

    let max = count.iter()
        .max_by_key(|(_, v)| *v)
        .unwrap();
    let min = count.iter()
        .min_by_key(|(_, v)| *v)
        .unwrap();

    // println!("{:?}", template);
    // println!("{:?}", count);

    max.1 - min.1
}

fn part_b(input: &Vec<String>, steps: usize) -> usize {
    let data = to_data(input);
    let mut pairs: HashMap<(char, char), usize> = HashMap::from_iter(data.rules.iter().map(|(k, _v)| (*k, 0)));
    let mut counter: HashMap<char, usize> = HashMap::new();

    let template = data.template.chars().collect::<Vec<char>>();
    for (index, c) in template.iter().enumerate() {
        *counter.entry(*c).or_default() += 1;
        if index + 1 == template.len() {
            continue;
        }
        let next = *template.get(index + 1).unwrap();
        *pairs.entry((*c, next)).or_default() += 1;
    }
    // println!("{:?}", pairs);
    // println!("{:?}",counter);

    for _step in 0..steps {
        let hits: Vec<(char, char)> = pairs.iter()
            .filter(|(_k, v)| **v > 0)
            .map(|(k, _v)| *k)
            .collect();
        // println!("Step: {}", step);
        // for pV in hits.iter(){
        //     println!("{:?}", p);
        // }
        let pairs_save = pairs.clone();
        for hit in hits.iter() {
            let rule = data.rules.get(hit).unwrap();
            let times = *pairs_save.get(hit).unwrap();
            let a = (hit.0, *rule);
            let b = (*rule, hit.1);
            // println!("{:?} becomes {:?} and {:?} based on {:?}   {}", hit, a, b, rule, times);
            *counter.entry(*rule).or_default() += times;
            *pairs.entry(a).or_default() += times;
            *pairs.entry(b).or_default() += times;
            *pairs.entry(*hit).or_default() -= times;
        }
    }

    // println!("{:?}", counter);

    let max = counter.iter()
        .max_by_key(|(_, v)| *v)
        .unwrap();
    let min = counter.iter()
        .min_by_key(|(_, v)| *v)
        .unwrap();

    max.1 - min.1
}

fn to_data(input: &Vec<String>) -> Data {
    let template = input.get(0).unwrap().clone();

    let rules = HashMap::from_iter(input.split_at(2).1.iter()
        .filter(|l| !l.is_empty())
        .map(|r| r.split(" -> ").collect::<Vec<&str>>())
        .map(|v| (v.get(0).unwrap().chars().collect::<Vec<char>>(), v.get(1).unwrap().chars().collect::<Vec<char>>()))
        .map(|(k, v)| ((*k.get(0).unwrap(), *k.get(1).unwrap()), *v.get(0).unwrap()))
    );

    Data { template, rules }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_data_t() {
        let filename = "src/day14/test-input.txt";
        let input = lines_from_file(filename);
        let data = to_data(&input);
        let expected_rules = HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C')
        ]);
        assert_eq!("NNCB", data.template);
        assert_eq!(expected_rules, data.rules);
    }

    #[test]
    fn part_a_test_input() {
        let filename = "src/day14/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input, 10);
        assert_eq!(1588, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day14/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input, 10);
        assert_eq!(3284, result);
    }

    #[test]
    fn part_b_test_input_steps() {
        let filename = "src/day14/test-input.txt";
        let input = lines_from_file(filename);
        assert_eq!(1, part_a(&input, 1));
        assert_eq!(1, part_b(&input, 1));
        assert_eq!(5, part_a(&input, 2));
        assert_eq!(5, part_b(&input, 2));
        assert_eq!(7, part_a(&input, 3));
        assert_eq!(7, part_b(&input, 3));
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day14/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input, 40);
        assert_eq!(2188189693529, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day14/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input, 40);
        assert_eq!(4302675529689, result);
    }
}
