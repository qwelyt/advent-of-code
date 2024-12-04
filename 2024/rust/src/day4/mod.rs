use crate::util::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 4 ==");
    let input = "src/day4/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> u32 {
    let puzzle: Vec<Vec<char>> = File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .unwrap();

    // for l in puzzle.iter() {
    //     println!("{:?}", l);
    // }
    // println!();

    let mut count = 0;
    for (r, row) in puzzle.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            match *ch {
                'X' => {
                    //Look for XMAS
                    count += check_position(&puzzle, ['X', 'M', 'A', 'S'], r, row.len(), c);
                }
                'S' => {
                    //Look for SAMX
                    count += check_position(&puzzle, ['S', 'A', 'M', 'X'], r, row.len(), c);
                }
                _ => {}
            }
        }
    }
    count
}

fn check_position(
    puzzle: &Vec<Vec<char>>,
    xmas: [char; 4],
    r: usize,
    row_len: usize,
    c: usize,
) -> u32 {
    let mut count = 0;
    if r < puzzle.len() - 3 {
        count += traverse([1, 2, 3], [0; 3], &puzzle, (r, c), xmas, "down");
        if c > 2 {
            count += traverse([1, 2, 3], [-1, -2, -3], &puzzle, (r, c), xmas, "down-left");
        }
        if c < row_len - 3 {
            count += traverse([1, 2, 3], [1, 2, 3], &puzzle, (r, c), xmas, "down-right")
        }
    }
    if c < row_len - 3 {
        count += traverse([0; 3], [1, 2, 3], &puzzle, (r, c), xmas, "right")
    }

    count
}

fn traverse(
    delta_r: [i32; 3],
    delta_c: [i32; 3],
    puzzle: &Vec<Vec<char>>,
    pos: (usize, usize),
    xmas: [char; 4],
    _str: &str,
) -> u32 {
    for i in 0..4 {
        let dr = (pos.0 as i32 + delta_r[i]) as usize;
        let dc = (pos.1 as i32 + delta_c[i]) as usize;
        if puzzle[dr][dc] != xmas[i + 1] {
            break;
        }
        if i == 2 {
            // println!("{}: {:?} :: {:?}", _str, pos, xmas);
            return 1;
        }
    }
    0
}

fn part_b(input: &str) -> i32 {
    let puzzle: Vec<Vec<char>> = File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .unwrap();
    // for l in puzzle.iter() {
    //     println!("{:?}", l);
    // }
    // println!();

    /*
       M . S    S . S
       . A .    . A .
       S . M    M . M
    */
    let mut count = 0;

    for (r, row) in puzzle.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            match *ch {
                'A' => {
                    if r < 1 || c < 1 || r > puzzle.len() - 2 || c > row.len() - 2 {
                        continue;
                    }
                    let dr = (r as i32 - 1) as usize;
                    let dc = (c as i32 + -1) as usize;

                    let u_l = puzzle[dr][dc];
                    let d_r = puzzle[r + 1][c + 1];

                    let u_r = puzzle[dr][c + 1];
                    let d_l = puzzle[r + 1][dc];

                    let v = [u_l, d_r, u_r, d_l];

                    let possible = [
                        ['M', 'S', 'M', 'S'],
                        ['M', 'S', 'S', 'M'],
                        ['S', 'M', 'M', 'S'],
                        ['S', 'M', 'S', 'M'],
                    ];
                    if possible.contains(&v) {
                        // println!("{:?}, {:?}", v, (r,c));
                        count += 1;
                    }
                }
                _ => {}
            }
        }
    }

    count
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
        let input = "src/day4/input.txt";
        assert_eq!(2462, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day4/input.txt";
        let i = part_b(input);
        assert_eq!(1877, i);
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_a(input);
        assert_eq!(18, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_b(input);
        assert_eq!(9, result);
    }
}
