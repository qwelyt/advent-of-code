use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::{lines, time};

pub fn day2() {
    println!("== Day 2 ==");
    let input = "src/day2/input.txt";
    // let input = "src/day2/aoc_2022_day02_large_input.txt";
    time(part_a_2, input, "A");
    time(part_b_2, input, "B");
}

struct Value {
    rock: usize,
    paper: usize,
    scissors: usize,
}

const VALUE: Value = Value {
    rock: 1,
    paper: 2,
    scissors: 3,
};

struct Scores {
    loose: usize,
    draw: usize,
    win: usize,
}

const SCORES: Scores = Scores {
    loose: 0,
    draw: 3,
    win: 6,
};

// A X = ROCK
// B Y = PAPER
// C Z = SCISSORS

enum Hand {
    ROCK,
    PAPER,
    SCISSORS,
}

enum End {
    LOOSE,
    DRAW,
    WIN,
}

fn map_hand(s: &str) -> Hand {
    match s {
        "A" => Hand::ROCK,
        "B" => Hand::PAPER,
        "C" => Hand::SCISSORS,
        "X" => Hand::ROCK,
        "Y" => Hand::PAPER,
        "Z" => Hand::SCISSORS,
        _ => { Hand::ROCK }
    }
}

fn map_to_end(s: &str) -> End {
    match s {
        "X" => End::LOOSE,
        "Y" => End::DRAW,
        "Z" => End::WIN,
        _ => { End::LOOSE }
    }
}


fn part_a(file: &str) -> usize {
    let lines = lines(file);
    let mut tot = 0;
    for line in lines.iter() {
        let split: Vec<&str> = line.split(" ").collect();
        let their = map_hand(split[0]);
        let my = map_hand(split[1]);
        let value = match their {
            Hand::ROCK => {
                match my {
                    Hand::ROCK => { VALUE.rock + SCORES.draw }
                    Hand::PAPER => { VALUE.paper + SCORES.win }
                    Hand::SCISSORS => { VALUE.scissors + SCORES.loose }
                }
            }
            Hand::PAPER => {
                match my {
                    Hand::ROCK => { VALUE.rock + SCORES.loose }
                    Hand::PAPER => { VALUE.paper + SCORES.draw }
                    Hand::SCISSORS => { VALUE.scissors + SCORES.win }
                }
            }
            Hand::SCISSORS => {
                match my {
                    Hand::ROCK => { VALUE.rock + SCORES.win }
                    Hand::PAPER => { VALUE.paper + SCORES.loose }
                    Hand::SCISSORS => { VALUE.scissors + SCORES.draw }
                }
            }
        };
        tot += value;
    }
    tot
}

fn part_b(file: &str) -> usize {
    let lines = lines(file);
    let mut tot = 0;
    for line in lines.iter() {
        let split: Vec<&str> = line.split(" ").collect();
        let their = map_hand(split[0]);
        let needed_end = map_to_end(split[1]);
        let value = match their {
            Hand::ROCK => {
                match needed_end {
                    End::LOOSE => { SCORES.loose + VALUE.scissors }
                    End::DRAW => { SCORES.draw + VALUE.rock }
                    End::WIN => { SCORES.win + VALUE.paper }
                }
            }
            Hand::PAPER => {
                match needed_end {
                    End::LOOSE => { SCORES.loose + VALUE.rock }
                    End::DRAW => { SCORES.draw + VALUE.paper }
                    End::WIN => { SCORES.win + VALUE.scissors }
                }
            }
            Hand::SCISSORS => {
                match needed_end {
                    End::LOOSE => { SCORES.loose + VALUE.paper }
                    End::DRAW => { SCORES.draw + VALUE.scissors }
                    End::WIN => { SCORES.win + VALUE.rock }
                }
            }
        };
        tot += value;
    }
    tot
}

// A X = ROCK
// B Y = PAPER
// C Z = SCISSORS
fn part_a_2(file: &str) -> i32 {
    let open = File::open(file).expect("Could not read file");
    let mut tot = 0;
    let x = 1;
    let y = 2;
    let z = 3;
    let loose = 0;
    let draw = 3;
    let win = 6;
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        tot += match line.as_str() {
            "A X" => x + draw,
            "A Y" => y + win,
            "A Z" => z + loose,
            "B X" => x + loose,
            "B Y" => y + draw,
            "B Z" => z + win,
            "C X" => x + win,
            "C Y" => y + loose,
            "C Z" => z + draw,
            &_ => 0,
        }
    }
    tot
}

// A = ROCK
// B = PAPER
// C = SCISSORS
// X = Loose
// Y = Draw
// Z = Win
fn part_b_2(file: &str) -> i32 {
    let open = File::open(file).expect("Could not read file");
    let mut tot = 0;
    let x = 0;
    let y = 3;
    let z = 6;
    let rock = 1;
    let paper = 2;
    let scissors = 3;
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        tot += match line.as_str() {
            "A X" => x + scissors,
            "A Y" => y + rock,
            "A Z" => z + paper,
            "B X" => x + rock,
            "B Y" => y + paper,
            "B Z" => z + scissors,
            "C X" => x + paper,
            "C Y" => y + scissors,
            "C Z" => z + rock,
            &_ => 0,
        }
    }
    tot
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day2();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day2/input.txt";
        assert_eq!(15572, part_a(input));
        assert_eq!(15572, part_a_2(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day2/input.txt";
        assert_eq!(16098, part_b(input));
        assert_eq!(16098, part_b_2(input));
    }

    #[ignore]
    #[test]
    fn big_a() {
        let input = "src/day2/aoc_2022_day02_large_input.txt";
        let start = Instant::now();
        let result = part_a_2(input);
        let end = Instant::now();
        println!("Part A: {} took {}ms", result, end.duration_since(start).as_millis());
        assert_eq!(499982425, result);
    }

    #[ignore]
    #[test]
    fn big_b() {
        let input = "src/day2/aoc_2022_day02_large_input.txt";
        let start = Instant::now();
        let result = part_b_2(input);
        let end = Instant::now();
        println!("Part B: {} took {}ms", result, end.duration_since(start).as_millis());
        assert_eq!(499971558, result);
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day2/test-input.txt";
        let result = part_a(input);
        assert_eq!(15, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day2/test-input.txt";
        let result = part_b(input);
        assert_eq!(12, result);
    }
}