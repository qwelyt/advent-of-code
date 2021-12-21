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
    println!("{:?}", positions);
    let result = play_deterministic(positions);
    println!("{:?}", result);
    let loser = u32::min(result.0, result.1);

    loser * result.2
}

fn part_b(input: &Vec<String>) -> u32 {
    todo!()
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
        assert_eq!(3351, result)
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day21/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(18989, result);
    }
}
