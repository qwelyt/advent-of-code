use std::collections::HashMap;
use std::str::FromStr;

use crate::util::lines_from_file;

pub fn day21() {
    println!("== Day 21 ==");
    let input = lines_from_file("src/day21/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> u32 {
    let i = input.iter()
        .map(|l| l.split(": ").collect::<Vec<&str>>())
        .map(|v| *v.get(1).unwrap())
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<u32>>();
    let positions = (i.get(0).unwrap(), i.get(1).unwrap());
    // println!("{:?}", positions);
    let result = play_deterministic(positions);
    // println!("{:?}", result);
    let loser = u32::min(result.0, result.1);

    loser * result.2
}

fn part_b(input: &Vec<String>) -> u64 {
    let i = input.iter()
        .map(|l| l.split(": ").collect::<Vec<&str>>())
        .map(|v| *v.get(1).unwrap())
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<u32>>();
    let positions = (i.get(0).unwrap(), i.get(1).unwrap());
    // println!("{:?}", positions);
    let (p1, p2) = play_dirac(positions);
    // println!("({}, {})", p1, p2);

    u64::max(p1, p2)
}

// -> (p1_score, p2_score, dice_throws)
fn play_deterministic(players: (&u32, &u32)) -> (u32, u32, u32) {
    let mut throws: u32 = 0;
    let p1: (u32, u32) = (*players.0, 0);
    let p2: (u32, u32) = (*players.1, 0);

    let mut p = [p1, p2];

    let mut turn: usize = 0;
    let max_score = 1000;
    while p[0].1 < max_score && p[1].1 < max_score {
        let mut steps_vec = Vec::new();
        for i in throws + 1..=throws + 3 {
            steps_vec.push(i % 100);
            throws += 1;
        }
        let steps: u32 = steps_vec.iter().sum();

        let player = turn % 2;
        let (old_position, old_score) = p[player];
        let mut new_position = old_position + steps;
        while new_position > 10 {
            new_position -= 10;
        }
        let new_score = old_score + new_position;
        // println!("p{}, o({},{}) n({},{}), t {}, s {} {:?}", player, old_position, old_score, new_position, new_score, throws, steps, steps_vec);

        p[player] = (new_position, new_score);

        turn += 1;
    }

    (p[0].1, p[1].1, throws)
}

fn play_dirac(players: (&u32, &u32)) -> (u64, u64) {
    let mut frequency_of_dice_outcome: HashMap<u32, u64> = HashMap::new();
    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                let sum_of_throws = d1 + d2 + d3;
                *frequency_of_dice_outcome.entry(sum_of_throws).or_insert(0) += 1;
            }
        }
    }
    // println!("{:?}", frequency_of_dice_outcome);
    let mut player = true;
    // If p1 has a score of 16 and is on tile 2, we don't really care how p1 got
    // there. We just care about *in how many worlds* did p1 get there.
    let mut worlds_per_outcome = HashMap::from([(((*players.0, 0), (*players.1, 0)), 1u64)]);
    let mut wins: (u64, u64) = (0, 0);
    loop {
        let mut new_outcomes = HashMap::new();

        for (p, worlds) in worlds_per_outcome {
            for (dice_roll, freq) in &frequency_of_dice_outcome {
                let (p_pos, p_score) = if player { p.0 } else { p.1 };
                let mut new_pos = p_pos + *dice_roll;
                while new_pos > 10 {
                    new_pos -= 10;
                }
                let new_score = p_score + new_pos;
                if new_score > 20 {
                    let new_wins = worlds * *freq;
                    if player { wins.0 += new_wins; } else { wins.1 += new_wins }
                } else {
                    let key = if player { ((new_pos, new_score), p.1) } else { (p.0, (new_pos, new_score)) };
                    *new_outcomes.entry(key).or_insert(0) += worlds * *freq;
                }
            }
        }

        if new_outcomes.is_empty() {
            break;
        }
        worlds_per_outcome = new_outcomes;
        player = !player;
    }

    wins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day21/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(739785, result)
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day21/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(576600, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day21/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(444356092776315, result)
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day21/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(131888061854776, result);
    }
}
