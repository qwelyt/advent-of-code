use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day8() {
    println!("== Day 8 ==");
    let input = "src/day8/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let matrix: Vec<Vec<u32>> = parse(input);
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    let mut row_len = 0;
    // Up -> down
    for (x, row) in matrix.iter().enumerate() {
        let mut highest = 0;
        if row.len() > row_len { row_len = row.len(); }

        // Left -> Right
        for (y, height) in row.iter().enumerate() {
            if *height > highest {
                visible.insert((x, y));
                highest = *height;
            } else if x == 0 || x == matrix.len() - 1 {
                visible.insert((x, y));
            }
        }
        highest = 0;
        // Right -> Left
        for (y, height) in row.iter().enumerate().rev() {
            if *height > highest {
                visible.insert((x, y));
                highest = *height;
            } else if x == 0 || x == matrix.len() - 1 {
                visible.insert((x, y));
            }
        }
    }
    // Left -> Right
    for y in 0..row_len {
        let mut highest = 0;
        // Up -> Down
        for (x, row) in matrix.iter().enumerate() {
            if row[y] > highest {
                visible.insert((x, y));
                highest = row[y];
            } else if y == 0 || y == row.len() - 1 {
                visible.insert((x, y));
            }
        }
        // Down -> up
        highest = 0;
        for (x, row) in matrix.iter().enumerate().rev() {
            if row[y] > highest {
                visible.insert((x, y));
                highest = row[y];
            } else if y == 0 || y == row.len() - 1 {
                visible.insert((x, y));
            }
        }
    }
    // println!("{:?}", visible);
    // let mut vec = visible.iter().map(|e| *e).collect::<Vec<(usize, usize)>>();
    // vec.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    // println!("{:?}", vec);
    // print_matrix(matrix);
    // println!();
    // for row in matrix.iter().enumerate() {
    //     println!();
    //     for col in row.1.iter().enumerate() {
    //         let v = if visible.contains(&(row.0, col.0)) { 1 } else { 0 };
    //         print!("{} ", v);
    //     }
    // }
    // println!();
    visible.len()
}

fn part_b(input: &str) -> u32 {
    let matrix = parse(input);
    let mut best = 0;
    for (x, row) in matrix.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            // if tree == highest_tree {
            let scenic_score = find_scenic_score(&matrix, x, y, *tree);
            if scenic_score > best {
                best = scenic_score;
            }
            // }
        }
    }
    // print_matrix(matrix);
    best
}

fn find_scenic_score(matrix: &Vec<Vec<u32>>, start_x: usize, start_y: usize, height: u32) -> u32 {
    // Go down
    let mut down = 0;
    for x in start_x..matrix.len() {
        if x == start_x { continue; }
        if matrix[x][start_y] >= height {
            down += 1;
            break;
        } else {
            down += 1;
        }
    }
    // Go up
    let mut up = 0;
    for x in (0..start_x).rev() {
        if x == start_x { continue; }
        if matrix[x][start_y] >= height {
            up += 1;
            break;
        } else {
            up += 1;
        }
    }
    // Go right
    let mut right = 0;
    for y in start_y..matrix[start_x].len() {
        if y == start_y { continue; }
        if matrix[start_x][y] >= height {
            right += 1;
            break;
        } else {
            right += 1;
        }
    }
    // Go left
    let mut left = 0;
    for y in (0..start_y).rev() {
        if y == start_y { continue; }
        if matrix[start_x][y] >= height {
            left += 1;
            break;
        } else {
            left += 1;
        }
    }

    // println!("Find scenic for {},{} ({})|| up {}, down {}, right {}, left {}", start_x, start_y, height,up,down,right,left);
    up * down * right * left
}

fn _print_matrix(matrix: Vec<Vec<u32>>) {
    for row in matrix.iter() {
        println!();
        for col in row.iter() {
            print!("{} ", col);
        }
    }
    println!();
}


fn parse(input: &str) -> Vec<Vec<u32>> {
    let open = File::open(input).expect("Could not read file");
    BufReader::new(open)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day8();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day8/input.txt";
        assert_eq!(1546, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day8/input.txt";
        assert_eq!(519064, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day8/test-input.txt";
        let result = part_a(input);
        assert_eq!(21, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day8/test-input.txt";
        let result = part_b(input);
        assert_eq!(8, result);
    }
}