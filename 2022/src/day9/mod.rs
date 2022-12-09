use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day9() {
    println!("== Day 9 ==");
    let input = "src/day9/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail.clone());

    let open = File::open(input).expect("Could not read file");
    // println!("Head at {:?}", head);
    for line in BufReader::new(open).lines() {
        let string = line.unwrap();
        let line = string.split(" ").collect::<Vec<&str>>();
        let amount = line[1].parse::<i32>().unwrap();
        match line[0] {
            "R" => for _ in 0..amount {
                head.1 += 1;
                if head.1 - tail.1 > 1 {
                    if tail.0 - head.0 > 0 {
                        tail.0 -= 1;
                    } else if head.0 - tail.0 > 0 {
                        tail.0 += 1;
                    }
                    tail.1 += 1;
                    visited.insert(tail.clone());
                }
            },
            "L" => for _ in 0..amount {
                head.1 -= 1;
                if tail.1 - head.1 > 1 {
                    if tail.0 - head.0 > 0 {
                        tail.0 -= 1;
                    } else if head.0 - tail.0 > 0 {
                        tail.0 += 1;
                    }
                    tail.1 -= 1;
                    visited.insert(tail.clone());
                }
            },
            "U" => for _ in 0..amount {
                head.0 += 1;
                if head.0 - tail.0 > 1 {
                    if head.1 - tail.1 > 0 {
                        tail.1 += 1;
                    } else if tail.1 - head.1 > 0 {
                        tail.1 -= 1;
                    }
                    tail.0 += 1;
                    visited.insert(tail.clone());
                }
            },
            "D" => for _ in 0..amount {
                head.0 -= 1;
                if tail.0 - head.0 > 1 {
                    if head.1 - tail.1 > 0 {
                        tail.1 += 1;
                    } else if tail.1 - head.1 > 0 {
                        tail.1 -= 1;
                    }
                    tail.0 -= 1;
                    visited.insert(tail.clone());
                }
            },
            _ => {}
        }
        // println!("Head at {:?}", head);
        // println!("Tail at {:?}", tail);
        // print(&visited);
    }

    // println!("{:?}", head);
    // println!("{:?}", tail);
    // println!("{:?}", visited);
    // print(&visited);

    visited.len()
}

fn part_b(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut tail: Vec<(i32, i32)> = vec![(0, 0); 10];
    // println!("{:?}", tail);
    visited.insert((0, 0));

    let open = File::open(input).expect("Could not read file");
    // println!("Head at {:?}", head);
    for line in BufReader::new(open).lines() {
        let string = line.unwrap();
        let line = string.split(" ").collect::<Vec<&str>>();
        let amount = line[1].parse::<i32>().unwrap();

        // println!("== {} ==", string);
        for _ in 0..amount {
            match line[0] {
                "R" => {
                    tail[0].1 += 1;
                    for i in 1..tail.len() {
                        if tail[i - 1].1 - tail[i].1 > 1 {
                            tail[i].1 += 1;
                            if tail[i].0 - tail[i - 1].0 > 0 {
                                tail[i].0 -= 1;
                            } else if tail[i - 1].0 - tail[i].0 > 0 {
                                tail[i].0 += 1;
                            }
                        } else if tail[i - 1].0 - tail[i].0 > 1 {
                            // println!("A");
                            tail[i].0 += 1;
                            // tail[i].1 += 1;
                            if tail[i].1 - tail[i - 1].1 > 0 {
                                tail[i].1 -= 1;
                            } else if tail[i - 1].1 - tail[i].1 > 0 {
                                tail[i].1 += 1;
                            }
                        } else if tail[i].0 - tail[i - 1].0 > 1 {
                            // println!("B");
                            tail[i].0 -= 1;
                            if tail[i].1 - tail[i - 1].1 > 0 {
                                tail[i].1 -= 1;
                            } else if tail[i - 1].1 - tail[i].1 > 0 {
                                tail[i].1 += 1;
                            }
                            // tail[i].1 += 1;
                        }
                        if i == tail.len() - 1 {
                            visited.insert(tail[i].clone());
                        }
                    }
                    // _print_positions(&tail);
                }
                "L" => {
                    tail[0].1 -= 1;
                    for i in 1..tail.len() {
                        if tail[i].1 - tail[i - 1].1 > 1 {
                            tail[i].1 -= 1;
                            if tail[i].0 - tail[i - 1].0 > 0 {
                                tail[i].0 -= 1;
                            } else if tail[i - 1].0 - tail[i].0 > 0 {
                                tail[i].0 += 1;
                            }
                        } else if tail[i - 1].0 - tail[i].0 > 1 {
                            // println!("A");
                            tail[i].0 += 1;
                            if tail[i].1 - tail[i - 1].1 > 0 {
                                tail[i].1 -= 1;
                            } else if tail[i - 1].1 - tail[i].1 > 0 {
                                tail[i].1 += 1;
                            }
                        } else if tail[i].0 - tail[i - 1].0 > 1 {
                            // println!("B");
                            tail[i].0 -= 1;
                            if tail[i].1 - tail[i - 1].1 > 0 {
                                tail[i].1 -= 1;
                            } else if tail[i - 1].1 - tail[i].1 > 0 {
                                tail[i].1 += 1;
                            }
                        }
                        if i == tail.len() - 1 {
                            visited.insert(tail[i].clone());
                        }
                    }
                }
                "U" => {
                    tail[0].0 += 1;
                    for i in 1..tail.len() {
                        if tail[i - 1].0 - tail[i].0 > 1 {
                            tail[i].0 += 1;
                            if tail[i - 1].1 - tail[i].1 > 0 {
                                tail[i].1 += 1;
                            } else if tail[i].1 - tail[i - 1].1 > 0 {
                                tail[i].1 -= 1;
                            }
                        } else if tail[i - 1].1 - tail[i].1 > 1 {
                            // println!("A");
                            tail[i].1 += 1;
                            if tail[i].0 - tail[i - 1].0 > 0 {
                                tail[i].0 -= 1;
                            } else if tail[i - 1].0 - tail[i].0 > 0 {
                                tail[i].0 += 1;
                            }
                        } else if tail[i].1 - tail[i - 1].1 > 1 {
                            // println!("B");
                            tail[i].1 -= 1;
                            if tail[i].0 - tail[i - 1].0 > 0 {
                                tail[i].0 -= 1;
                            } else if tail[i - 1].0 - tail[i].0 > 0 {
                                tail[i].0 += 1;
                            }
                        }
                        if i == tail.len() - 1 {
                            visited.insert(tail[i].clone());
                        }
                    }
                }
                "D" => {
                    tail[0].0 -= 1;
                    for i in 1..tail.len() {
                        if tail[i].0 - tail[i - 1].0 > 1 {
                            tail[i].0 -= 1;
                            if tail[i - 1].1 - tail[i].1 > 0 {
                                tail[i].1 += 1;
                            } else if tail[i].1 - tail[i - 1].1 > 0 {
                                tail[i].1 -= 1;
                            }
                        } else if tail[i - 1].1 - tail[i].1 > 1 {
                            // println!("A");
                            tail[i].1 += 1;
                            if tail[i].0 - tail[i - 1].0 > 0 {
                                tail[i].0 -= 1;
                            } else if tail[i - 1].0 - tail[i].0 > 0 {
                                tail[i].0 += 1;
                            }
                        } else if tail[i].1 - tail[i - 1].1 > 1 {
                            // println!("B");
                            tail[i].1 -= 1;
                            if tail[i].0 - tail[i - 1].0 > 0 {
                                tail[i].0 -= 1;
                            } else if tail[i - 1].0 - tail[i].0 > 0 {
                                tail[i].0 += 1;
                            }
                        }
                        if i == tail.len() - 1 {
                            visited.insert(tail[i].clone());
                        }
                    }
                }
                _ => {}
            }
            // println!("{:?}", tail);
            // _print_positions(&tail);
        }
        // println!("Head at {:?}", head);
        // println!("Tail at {:?}", tail);
        // _print(&visited);
        // _print_positions(&tail);
    }
    // _print(&visited);
    // println!("{:?}", tail);
    // println!("{:?}",visited);
    visited.len()
}

fn _print_positions(tail: &Vec<(i32, i32)>) {
    let tmp_x = tail.iter().map(|p| p.1).min().unwrap();
    let min_x: i32 = if tmp_x > 0 { 0 } else { tmp_x };
    let tmp_y = tail.iter().map(|p| p.0).min().unwrap();
    let min_y: i32 = if tmp_y > 0 { 0 } else { tmp_y };

    let tmp_x = tail.iter().map(|p| p.1).max().unwrap(); //+ min_x.abs();
    let max_x = if tmp_x < 0 { 0 } else { tmp_x };
    let tmp_y = tail.iter().map(|p| p.0).max().unwrap(); //+ min_y.abs();
    let max_y = if tmp_y < 0 { 0 } else { tmp_y };


    let mut map: Vec<Vec<String>> = Vec::new();
    for _ in (min_y..=max_y).rev() {
        let mut vec = Vec::new();
        for _ in min_x..=max_x {
            vec.push(".".to_string());
        }
        map.push(vec);
    }
    // map[(min_y.abs()) as usize][min_x.abs() as usize] = "s".to_string();
    // println!("{}, {}, {}", max_y, min_y, max_y-min_y.abs());
    // println!("{}", map.len());
    if map.len() > 0 {
        let o_y = map.len() - 1 - (min_y.abs() as usize);
        let o_x = min_x.abs() as usize;
        map[o_y][o_x] = "s".to_string();
        for (i, t) in tail.iter().enumerate() {
            let y = map.len() - 1 - (t.0 + min_y.abs()) as usize;
            let x = (t.1 + min_x.abs()) as usize;
            if map[y][x].eq(".") || map[y][x].eq("s") {
                if i == 0 {
                    map[y][x] = "H".to_string();
                } else {
                    map[y][x] = i.to_string();
                }
            }
        }
    }

    for x in map.iter() {
        for y in x.iter() {
            print!("{}", y);
        }
        println!();
    }
    println!();
}

fn _print(positions: &HashSet<(i32, i32)>) {
    let max_x = positions.iter().map(|p| p.1).max().unwrap() + 1;
    let min_x = positions.iter().map(|p| p.1).min().unwrap();
    let max_y = positions.iter().map(|p| p.0).max().unwrap();
    let min_y = positions.iter().map(|p| p.0).min().unwrap();

    println!();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if y == 0 && x == 0 {
                print!("s");
            } else if positions.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day9();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day9/input.txt";
        assert_eq!(6284, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day9/input.txt";
        let result = part_b(input);
        assert_eq!(2661, result);
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day9/test-input.txt";
        let result = part_a(input);
        assert_eq!(13, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day9/test-input.txt";
        let result = part_b(input);
        assert_eq!(1, result);
    }

    #[test]
    fn part_b_test_input_2() {
        let input = "src/day9/test-input2.txt";
        let result = part_b(input);
        assert_eq!(36, result);
    }

    #[test]
    fn part_b_test_input_3() {
        let input = "src/day9/test-input3.txt";
        let result = part_b(input);
        assert_eq!(36, result);
    }

    #[ignore]
    #[test]
    fn print_positions_test() {
        let positions = vec![(13, 0), (0, 13), (-1, 0), (0, -1)];
        _print_positions(&positions);
        let test = vec![(-1, 0)];
        _print_positions(&test)
    }
}