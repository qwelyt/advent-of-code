use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day4() {
    println!("== Day 4 ==");
    let input = "src/day4/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> i32 {
    let mut sum = 0;
    let open = File::open(input).expect("Could not read file");
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let game = line.split(": ").collect::<Vec<&str>>()[1];
        let win_have = game.split(" | ").collect::<Vec<&str>>();
        let winning_numbers: HashSet<&str> = HashSet::from_iter(win_have[0].split(" ").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>().iter().cloned());
        let have_numbers = HashSet::from_iter(win_have[1].split(" ").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>().iter().cloned());
        let intersection = have_numbers.intersection(&winning_numbers);//.collect::<HashSet<&str>>();

        // print!("{:?}",intersection);
        let points = intersection//.iter()
            .fold(0, |a, _b| {
                if a == 0 { a + 1 } else { a * 2 }
            });
        // println!(" -- {}",points);
        sum += points
    }
    sum
}

fn part_b(input: &str) -> usize {
    let mut sum = 0;
    let mut wins_by_card: HashMap<String, usize> = HashMap::new();
    let mut num_cards: HashMap<usize, usize> = HashMap::new();
    let open = File::open(input).expect("Could not read file");
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let game = line.split(": ").collect::<Vec<&str>>();
        let game_id = game[0].split(" ").collect::<Vec<&str>>().last().unwrap().parse::<usize>().unwrap();
        *num_cards.entry(game_id).or_insert(0) += 1;
        let win_have = game[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers: HashSet<&str> = HashSet::from_iter(win_have[0].split(" ").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>().iter().cloned());
        let have_numbers = HashSet::from_iter(win_have[1].split(" ").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>().iter().cloned());
        let intersection = have_numbers.intersection(&winning_numbers).copied().collect::<Vec<&str>>();
        wins_by_card.insert(game[0].to_string(), intersection.len());
        for i in game_id + 1..game_id + 1 + intersection.len() {
            let amount_to_add = num_cards.get(&game_id).unwrap_or(&1);
            // println!("Game {} adds {} to {}", game_id,amount_to_add,i);
            *num_cards.entry(i).or_insert(0) += *amount_to_add;
        }
    }
    for (_k, v) in num_cards.iter() {
        // println!("Game {}: {}", _k, v);
        sum += v
    }
    // println!("{:?}", wins_by_card);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day4();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day4/input.txt";
        assert_eq!(28538, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day4/input.txt";
        assert_eq!(9425061, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_a(input);
        assert_eq!(13, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_b(input);
        assert_eq!(30, result);
    }
}