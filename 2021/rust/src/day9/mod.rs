use std::collections::{HashSet, VecDeque};

use crate::util::lines_from_file;

pub fn day9() {
    println!("== Day 9 ==");
    let input = lines_from_file("src/day9/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> u32 {
    let height_map: Vec<Vec<u32>> = input.iter()
        .map(|s| s.chars().collect::<Vec<char>>().iter().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut lowest_points: Vec<u32> = Vec::new();
    for (row_index, row) in height_map.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            let left = if col_index == 0 { true } else { value < row.get(col_index - 1).unwrap_or(&u32::MAX) };
            let right = if col_index == row.len() - 1 { true } else { value < row.get(col_index + 1).unwrap_or(&u32::MAX) };
            let up = if row_index == 0 { true } else { value < height_map.get(row_index - 1).unwrap_or(&vec![u32::MAX; row.len()]).get(col_index).unwrap_or(&u32::MAX) };
            let down = if row_index == height_map.len() - 1 { true } else { value < height_map.get(row_index + 1).unwrap_or(&vec![u32::MAX; row.len()]).get(col_index).unwrap_or(&u32::MAX) };
            if left && right && up && down {
                lowest_points.push(*value);
            }
        }
    }

    lowest_points.iter().map(|i| i + 1).sum()
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct SPoint {
    row: usize,
    col: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct RC {
    r: usize,
    c: usize,
    v: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize,
    neighbours: Vec<SPoint>,
}

fn part_b(input: &Vec<String>) -> usize {
    let height_map: Vec<Vec<u32>> = input.iter()
        .map(|s| s.chars().collect::<Vec<char>>().iter().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut basins: Vec<HashSet<RC>> = Vec::new();
    let mut seen: HashSet<RC> = HashSet::new();
    let rd: Vec<i32> = vec![-1, 0, 1, 0];
    let cd: Vec<i32> = vec![0, -1, 0, 1];
    let rows = height_map.len() as i32;
    let cols = height_map.get(0).unwrap().len() as i32;
    for (row, row_vec) in height_map.iter().enumerate() {
        for (col, col_value) in row_vec.iter().enumerate() {
            let mut r = row;
            let mut c = col;
            let mut set: HashSet<RC> = HashSet::new();
            let mut q: VecDeque<RC> = VecDeque::new();
            q.push_back(RC { r, c, v: *col_value });
            while !q.is_empty() {
                let point = q.pop_front().unwrap();
                r = point.r;
                c = point.c;
                if seen.contains(&point) || point.v == 9 { // I have no idea why we are adding a point with value 9 when we check for it to *not* be that.
                    // println!("I've already seen {:?}",point);
                    continue;
                } else {
                    seen.insert(point);
                    set.insert(point.clone());
                    for i in 0..4 {
                        let rr = r as i32 + *rd.get(i).unwrap();
                        let cc = c as i32 + *cd.get(i).unwrap();
                        // println!("{} ({},{}) -- {:?} - ({},{})",i,r,c,point,rr,cc);
                        if rr >= 0 && rr < rows && cc >= 0 && cc < cols {
                            let option1 = height_map.get(rr as usize);
                            if option1.is_some() {
                                let option = option1.unwrap().get(cc as usize);
                                if option.is_some() && option.unwrap() != &9 {
                                    let rc = RC { r: rr as usize, c: cc as usize, v: *option.unwrap() };
                                    // println!("Found {:?} -- {}",rc,option.unwrap());
                                    q.push_back(rc);
                                }
                            }
                        }
                    }
                }
            }
            basins.push(set);
        }
    }

    let mut basin_sizes: Vec<usize> = basins.iter()
        .map(|b| b.len())
        .collect();

    // basins.sort_by(|a,b| a.len().cmp(&b.len()));
    // for basin in basins {
    //     println!("{:?}",basin);
    // }
    basin_sizes.sort();
    basin_sizes.reverse();
    // println!("{:?}",basin_sizes);
    basin_sizes.split_at(3).0.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day9/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(15, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day9/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(560, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day9/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(1134, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day9/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(959136, result);
    }
}