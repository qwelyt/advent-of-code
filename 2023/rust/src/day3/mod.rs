use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

struct Schematic {
    layout: Vec<Vec<char>>,
}

pub fn day3() {
    println!("== Day 3 ==");
    let input = "src/day3/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let schematic = to_schematic(input);
    solve_schematic(schematic)
}

fn solve_schematic(schematic: Schematic) -> usize {
    let mut sum = 0;
    let yd = [-1, -1, -1, 0, 0, 1, 1, 1];
    let xd = [-1, 0, 1, -1, 1, -1, 0, 1];
    let y_m = schematic.layout.len() as i32;
    let x_m = schematic.layout.get(0).unwrap().len() as i32;
    for (y, v) in schematic.layout.iter().enumerate() {
        let mut curr_number = Vec::new();
        let mut should_add = false;
        for (x, c) in v.iter().enumerate() {
            if c.is_digit(10) {
                curr_number.push(*c);
                for i in 0..yd.len() {
                    let r = y as i32 + yd[i];
                    let c = x as i32 + xd[i];
                    if r >= 0 && r <= y_m && c >= 0 && c <= x_m {
                        // let t = schematic.layout.get(r as usize).unwrap().get(c as usize).unwrap();
                        let t = schematic.layout.get(r as usize)
                            .map(|row| row.get(c as usize))
                            .flatten()
                            .unwrap_or(&'.');
                        if *t != '.' && !t.is_digit(10) {
                            should_add = true;
                        }
                    }
                }
            } else {
                if should_add && !curr_number.is_empty() {
                    let nr: String = curr_number.iter().collect();
                    //println!("'{}'", nr);
                    sum += nr.parse::<usize>().unwrap();
                }
                curr_number.clear();
                should_add = false;
            }
            if x == v.len() - 1 {
                if should_add && !curr_number.is_empty() {
                    let nr: String = curr_number.iter().collect();
                    //println!("'{}'", nr);
                    sum += nr.parse::<usize>().unwrap();
                }
                curr_number.clear();
                should_add = false;
            }
        }
    }
    sum
}

fn part_b(input: &str) -> usize {
    let schematic = to_schematic(input);
    find_gear_sum(schematic)
}

fn find_gear_sum(schematic: Schematic) -> usize {
    let mut sum = 0;
    let mut valids: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let yd = [-1, -1, -1, 0, 0, 1, 1, 1];
    let xd = [-1, 0, 1, -1, 1, -1, 0, 1];
    let y_m = schematic.layout.len() as i32;
    let x_m = schematic.layout.get(0).unwrap().len() as i32;
    for (y, v) in schematic.layout.iter().enumerate() {
        let mut curr_number = Vec::new();
        let mut pos = None;
        for (x, c) in v.iter().enumerate() {
            if c.is_digit(10) {
                curr_number.push(*c);
                for i in 0..yd.len() {
                    let r = y as i32 + yd[i];
                    let c = x as i32 + xd[i];
                    if r >= 0 && r <= y_m && c >= 0 && c <= x_m {
                        // let t = schematic.layout.get(r as usize).unwrap().get(c as usize).unwrap();
                        let t = schematic.layout.get(r as usize)
                            .map(|row| row.get(c as usize))
                            .flatten()
                            .unwrap_or(&'.');
                        if *t == '*' {
                            pos = Some((r as usize, c as usize));
                        }
                    }
                }
            } else {
                if pos.is_some() {
                    //println!("Should insert {:?} into {:?}", curr_number, pos);
                    let nr: usize = curr_number.iter().collect::<String>().parse::<usize>().unwrap();
                    let mut x1 = valids.get(&pos.unwrap()).unwrap_or(&mut Vec::new()).clone();
                    x1.push(nr);
                    valids.insert(pos.unwrap(), x1);
                    pos = None;
                }
                curr_number.clear();
            }
            if x == v.len() - 1 {
                // println!("{:?} -- {:?}", pos, curr_number);
                if pos.is_some() {
                    let nr: usize = curr_number.iter().collect::<String>().parse::<usize>().unwrap();
                    let mut x1 = valids.get(&pos.unwrap()).unwrap_or(&mut Vec::new()).clone();
                    x1.push(nr);
                    valids.insert(pos.unwrap(), x1);
                    // println!("Added {} to valids {:?}", nr,valids);
                    pos = None;
                }
                curr_number.clear();
            }
            // println!("{:?}", valids);
        }
    }
    let mut vec = Vec::from_iter(valids.iter());
    vec.sort_by(|a, b| a.0.cmp(b.0));
    for (_k, v) in vec.iter() {
        if v.len() == 2 {
            let product = v.iter().product::<usize>();
            // println!("{:?} -- Multiply {:?} to get {}", _k, v, product);
            sum += product;
        }
    }
    sum
}

fn to_schematic(input: &str) -> Schematic {
    let open = File::open(input).expect("Could not read file");
    let mut sc = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let vc = line.chars().collect::<Vec<char>>();
        sc.push(vc);
    }
    Schematic {
        layout: sc,

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let input = "src/day3/test-input.txt";
        let result = part_a(input);
        assert_eq!(4361, result);
    }

    #[test]
    fn solver_test_simple() {
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
            ];
            assert_eq!(0, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['.', '1', '.'],
                vec!['.', '.', '.'],
            ];
            assert_eq!(0, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['#', '.', '.'],
                vec!['.', '1', '.'],
                vec!['.', '.', '.'],
            ];
            assert_eq!(1, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '#', '.'],
                vec!['.', '1', '.'],
                vec!['.', '.', '.'],
            ];
            assert_eq!(1, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '#'],
                vec!['.', '1', '.'],
                vec!['.', '.', '.'],
            ];
            assert_eq!(1, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['#', '1', '.'],
                vec!['.', '.', '.'],
            ];
            assert_eq!(1, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['.', '1', '#'],
                vec!['.', '.', '.'],
            ];
            assert_eq!(1, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['.', '1', '.'],
                vec!['#', '.', '.'],
            ];
            assert_eq!(1, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['.', '1', '.'],
                vec!['.', '#', '.'],
            ];
            assert_eq!(1, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['.', '1', '.'],
                vec!['.', '.', '#'],
            ];
            assert_eq!(1, solve_schematic(Schematic { layout }))
        }
    }

    #[test]
    fn solver_test_bigger() {
        {
            let layout = vec![
                vec!['#', '.', '.', '.'],
                vec!['.', '.', '1', '.'],
                vec!['3', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
            ];
            assert_eq!(0, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.', '.'],
                vec!['#', '.', '1', '.'],
                vec!['3', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
            ];
            assert_eq!(3, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '1', '.'],
                vec!['3', '.', '.', '.'],
                vec!['#', '.', '.', '.'],
            ];
            assert_eq!(3, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '1', '.'],
                vec!['3', '#', '.', '.'],
                vec!['#', '.', '.', '.'],
            ];
            assert_eq!(4, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.', '.'],
                vec!['3', '.', '1', '.'],
                vec!['3', '#', '.', '.'],
                vec!['#', '.', '.', '.'],
            ];
            assert_eq!(7, solve_schematic(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '1', '1'],
                vec!['.', '#', '.', '.'],
                vec!['#', '.', '.', '.'],
            ];
            assert_eq!(11, solve_schematic(Schematic { layout }))
        }
    }

    #[test]
    fn edge_case() {
        {
            let layout = vec![
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '1', '1'],
                vec!['.', '#', '.', '.'],
                vec!['#', '.', '.', '.'],
            ];
            assert_eq!(11, solve_schematic(Schematic { layout }))
        }
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day3/input.txt";
        assert_eq!(true, part_a(input) > 530105);
        assert_eq!(532331, part_a(input));
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day3/test-input.txt";
        let result = part_b(input);
        assert_eq!(467835, result);
    }

    #[test]
    fn gear_sum_test() {
        {
            let layout = vec![
                vec!['.', '.', '.', '.'],
                vec!['2', '.', '1', '1'],
                vec!['.', '*', '.', '.'],
                vec!['.', '.', '.', '.'],
            ];
            assert_eq!(22, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.', '.'],
                vec!['2', '*', '1', '1'],
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
            ];
            assert_eq!(22, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '*', '.', '.'],
                vec!['2', '-', '1', '1'],
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
            ];
            assert_eq!(22, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.', '.'],
                vec!['2', '*', '.', '.'],
                vec!['2', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
            ];
            assert_eq!(4, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['2', '.', '.', '.'],
                vec!['.', '*', '.', '1'],
                vec!['.', '1', '1', '.'],
                vec!['.', '.', '.', '.'],
            ];
            assert_eq!(22, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '2', '.', '.'],
                vec!['.', '*', '.', '1'],
                vec!['1', '1', '.', '.'],
                vec!['.', '.', '.', '.'],
            ];
            assert_eq!(22, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '7', '6', '9', '.', '.', '.', '.'],
                vec!['@', '.', '.', '.', '*', '.', '.', '.', '.'],
                vec!['1', '3', '7', '.', '.', '4', '3', '2', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ];
            assert_eq!(332208, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['2', '.', '.'],
                vec!['.', '*', '.'],
                vec!['.', '.', '4'],
            ];
            assert_eq!(8, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '2', '.'],
                vec!['.', '*', '.'],
                vec!['.', '.', '4'],
            ];
            assert_eq!(8, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '2'],
                vec!['.', '*', '.'],
                vec!['.', '.', '4'],
            ];
            assert_eq!(8, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['2', '*', '.'],
                vec!['.', '.', '4'],
            ];
            assert_eq!(8, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['.', '*', '2'],
                vec!['.', '.', '4'],
            ];
            assert_eq!(8, find_gear_sum(Schematic { layout }))
        }
        {
            let layout = vec![
                vec!['.', '.', '.'],
                vec!['.', '*', '.'],
                vec!['2', '.', '4'],
            ];
            assert_eq!(8, find_gear_sum(Schematic { layout }))
        }
    }

    #[test]
    fn gear_edge_case() {
        {
            let layout = vec![
                vec!['.', '.', '2'],
                vec!['.', '*', '.'],
                vec!['.', '.', '4'],
            ];
            assert_eq!(8, find_gear_sum(Schematic { layout }))
        }
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day3/input.txt";
        assert_eq!(true, 81553820 < part_b(input));
        assert_eq!(82301120, part_b(input));
    }


    #[ignore]
    #[test]
    fn runday() {
        day3()
    }
}