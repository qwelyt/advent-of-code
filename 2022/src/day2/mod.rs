use crate::util::lines;

pub fn day2() {
    print!("== Day 2 ==");
    let input = "src/day2/input.txt";
    println!("Part A: {}", part_a(input));
    println!("Part B: {}", part_b(input));
}

struct Value {
    rock: i32,
    paper: i32,
    scissors: i32,
}

const VALUE: Value = Value {
    rock: 1,
    paper: 2,
    scissors: 3,
};

struct Scores {
    loose: i32,
    draw: i32,
    win: i32,
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


fn part_a(file: &str) -> i32 {
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

fn part_b(file: &str) -> i32 {
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

#[cfg(test)]
mod tests {
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
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day2/input.txt";
        assert_eq!(16098, part_b(input));
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