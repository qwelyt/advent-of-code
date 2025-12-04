use std::collections::HashSet;

use crate::util::lines_from_file;

pub fn day11() {
    println!("== Day 11 ==");
    let input = lines_from_file("src/day11/input.txt");
    let a = part_a(&input, 100);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    row: usize,
    col: usize,
}

fn part_a(input: &Vec<String>, steps: i32) -> usize {
    let mut current_state: Vec<Vec<u8>> = input.iter().map(|l| to_numbers(l)).collect();

    // println!("### Steps: {}  Start: {:?}", steps, current_state);
    let mut total_flashes: usize = 0;
    for _step in 1..=steps {
        current_state.iter_mut()
            .for_each(|l| l.iter_mut()
                .for_each(|n| *n += 1)
            );
        // println!("Step {}: {:?}", step, current_state);

        let mut flashed: Vec<Point> = flash(&current_state);
        total_flashes += flashed.len();
        // println!("Step {} flashes: {:?}", step, flashed);
        let mut has_flashed: HashSet<Point> = HashSet::new();
        flashed.iter().for_each(|p| { has_flashed.insert(p.clone()); });
        while flashed.len() > 0 {
            let adjacent_vec: Vec<HashSet<Point>> = adjacent(current_state.len() as i32, current_state.get(0).unwrap().len() as i32, &flashed);
            // println!("Adjacent: {:?}", adjacent_vec);
            // println!("Current state: {:?}", &current_state);
            // pretty_print(&current_state);

            adjacent_vec.iter()
                .for_each(|adjacent| {
                    current_state.iter_mut().enumerate()
                        .for_each(|(row, r)| r.iter_mut().enumerate()
                            .filter(|(col, _)| adjacent.contains(&Point { row, col: *col }))
                            .for_each(|(_col, c)| *c += 1)
                        );
                });
            // println!("Current state AA: {:?}", &current_state);
            // pretty_print(&current_state);
            let vec = flash(&current_state);
            // println!("vec: {:?}", vec);
            let new_flashes: Vec<Point> = vec.into_iter()
                .filter(|p| !has_flashed.contains(p))
                .collect();
            // println!("old flashes: {:?}", has_flashed);
            // println!("new flashes: {:?}", new_flashes);
            total_flashes += new_flashes.len();
            flashed = new_flashes;//new_flashes;
            flashed.iter().for_each(|p| { has_flashed.insert(p.clone()); });
        }


        current_state.iter_mut()
            .for_each(|r| r.iter_mut()
                .filter(|c| **c > 9)
                .for_each(|c| *c = 0));
    }

    total_flashes
}

fn part_b(input: &Vec<String>) -> i32 {
    let mut current_state: Vec<Vec<u8>> = input.iter().map(|l| to_numbers(l)).collect();

    let mut all_has_flashed = false;
    let mut step = 0;
    while !all_has_flashed {
        step += 1;
        current_state.iter_mut()
            .for_each(|l| l.iter_mut()
                .for_each(|n| *n += 1)
            );

        let mut flashed: Vec<Point> = flash(&current_state);
        let mut has_flashed: HashSet<Point> = HashSet::new();
        flashed.iter().for_each(|p| { has_flashed.insert(p.clone()); });
        while flashed.len() > 0 {
            let adjacent_vec: Vec<HashSet<Point>> = adjacent(current_state.len() as i32, current_state.get(0).unwrap().len() as i32, &flashed);

            adjacent_vec.iter()
                .for_each(|adjacent| {
                    current_state.iter_mut().enumerate()
                        .for_each(|(row, r)| r.iter_mut().enumerate()
                            .filter(|(col, _c)| adjacent.contains(&Point { row, col: *col }))
                            .for_each(|(_col, c)| *c += 1)
                        );
                });
            let vec = flash(&current_state);
            let new_flashes: Vec<Point> = vec.into_iter()
                .filter(|p| !has_flashed.contains(p))
                .collect();
            flashed = new_flashes;//new_flashes;
            flashed.iter().for_each(|p| { has_flashed.insert(p.clone()); });
        }
        current_state.iter_mut()
            .for_each(|r| r.iter_mut()
                .filter(|c| **c > 9)
                .for_each(|c| *c = 0));
        if all_equals(0, &current_state) {
            all_has_flashed = true;
        }
    }

    step
}

fn all_equals(should_equal: u8, state: &Vec<Vec<u8>>) -> bool {
    let mut ok = true;
    for r in state.iter() {
        for c in r.iter() {
            if *c != should_equal {
                ok = false;
            }
        }
    }

    ok
}

fn adjacent(rows: i32, cols: i32, points: &Vec<Point>) -> Vec<HashSet<Point>> {
    let mut vec: Vec<HashSet<Point>> = Vec::new();

    let rd = [-1, -1, -1, 0, 0, 1, 1, 1];
    let cd = [-1, 0, 1, -1, 1, -1, 0, 1];
    for point in points.iter() {
        let mut set: HashSet<Point> = HashSet::new();
        let row = point.row as i32;
        let col = point.col as i32;
        for i in 0..rd.len() {
            let rr = row + rd[i];
            let cc = col + cd[i];
            if rr >= 0 && rr < rows && cc >= 0 && cc < cols {
                set.insert(Point { row: rr as usize, col: cc as usize });
            }
        }
        vec.push(set);
    }

    vec
}

fn flash(state: &Vec<Vec<u8>>) -> Vec<Point> {
    state.iter().enumerate()
        .map(|(row, r)| r.iter().enumerate()
            .filter(|(_col, c)| **c > 9)
            .map(|(col, _c)| Point { row, col })
            .collect::<Vec<Point>>()
        ).flatten()
        .collect()
}

fn to_numbers(line: &String) -> Vec<u8> {
    line.split("").collect::<Vec<&str>>().iter().filter(|s| !s.is_empty()).map(|s| s.parse::<u8>().unwrap()).collect()
}

// fn pretty_print(state: &Vec<Vec<u8>>) {
//     for line in state.iter() {
//         println!("{:?}", line);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_t() {
        assert_eq!(vec![1, 2, 3], to_numbers(&"123".to_string()))
    }

    #[test]
    fn adjacent_t() {
        let expected = HashSet::from([
            Point { row: 0, col: 0 },
            Point { row: 0, col: 1 },
            Point { row: 0, col: 2 },
            Point { row: 1, col: 0 },
            Point { row: 1, col: 2 },
            Point { row: 2, col: 0 },
            Point { row: 2, col: 1 },
            Point { row: 2, col: 2 },
        ]);
        let result = adjacent(3, 3, &vec![Point { row: 1, col: 1 }]);
        assert_eq!(vec![expected], result)
    }

    #[test]
    fn adjacent_edge_t() {
        let expected = HashSet::from([
            Point { row: 0, col: 0 },
            Point { row: 0, col: 1 },
            Point { row: 1, col: 1 },
            Point { row: 2, col: 0 },
            Point { row: 2, col: 1 },
        ]);
        let result = adjacent(3, 3, &vec![Point { row: 1, col: 0 }]);
        assert_eq!(vec!(expected), result)
    }

    #[test]
    fn example_test() {
        let input: Vec<String> = Vec::from([
            "11111".to_string(),
            "19991".to_string(),
            "19191".to_string(),
            "19991".to_string(),
            "11111".to_string()
        ]);
        assert_eq!(9, part_a(&input, 2));
    }

    #[test]
    fn part_a_test_input_steps() {
        let filename = "src/day11/test-input.txt";
        let input = lines_from_file(filename);
        assert_eq!(0, part_a(&input, 1));
        assert_eq!(35, part_a(&input, 2));
        assert_eq!(204, part_a(&input, 10));
    }

    #[test]
    fn part_a_test_input() {
        let filename = "src/day11/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input, 100);
        assert_eq!(1656, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day11/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input, 100);
        assert_eq!(1659, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day11/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(195, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day11/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(227, result);
    }
}