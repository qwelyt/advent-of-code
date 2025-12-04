use std::collections::HashSet;

use crate::util::lines_from_file;

pub fn day13() {
    println!("== Day 13 ==");
    let input = lines_from_file("src/day13/input.txt");
    let a = part_a(&input, 1);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B:");
    print_paper(&b);
}


#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Data {
    marked: HashSet<Point>,
    folds: Vec<(char, usize)>,
}

fn part_a(input: &Vec<String>, folds: usize) -> usize {
    let data = to_data(input);
    // print_paper(&data.marked);
    fold_paper(folds, data).len()
}

fn part_b(input: &Vec<String>) -> HashSet<Point> {
    let data = to_data(input);
    // print_paper(&data.marked);
    fold_paper(data.folds.len(), data)
}

fn fold_paper(folds: usize, data: Data) -> HashSet<Point> {
    let mut marked_set = data.marked.clone();
    for fold in 0..folds {
        let mut marked: Vec<Point> = marked_set.clone().into_iter().collect::<Vec<Point>>();
        let (axis, place): &(char, usize) = data.folds.get(fold as usize).unwrap();
        if *axis == 'x' {
            marked.iter_mut()
                .for_each(|p| {
                    if p.x > *place {
                        // println!("sub x: {} :: {}", p.x, place);
                        let i = p.x.checked_sub(*place).unwrap();
                        let new_x = place.checked_sub(i).unwrap();
                        p.x = new_x;
                    }
                })
        } else if *axis == 'y' {
            marked.iter_mut()
                .for_each(|p| {
                    if p.y > *place {
                        let i = p.y.checked_sub(*place).unwrap();
                        let new_y = place.checked_sub(i).unwrap();
                        // println!("sub y: {} :: {} :: {} :: {}", p.y, place, i, new_y);
                        p.y = new_y;
                    }
                })
        }

        // println!("After step {}",fold);
        marked_set = HashSet::from_iter(marked.into_iter());
        // print_paper(&marked_set);
    }
    marked_set
}

fn print_paper(marked: &HashSet<Point>) {
    let mut x = 0;
    let mut y = 0;
    for point in marked.iter() {
        if point.x > x { x = point.x; }
        if point.y > y { y = point.y; }
    }

    let mut paper: Vec<Vec<char>> = Vec::new();
    for _ in 0..=y {
        paper.push(vec!['.'; (x + 1) as usize]);
    }
    for point in marked.iter() {
        let x1: &mut Vec<char> = paper.get_mut(point.y as usize).unwrap();
        let _ = std::mem::replace(&mut x1[point.x as usize], '#');
    }

    for l in paper.iter() {
        println!("{}", l.into_iter().collect::<String>());
    }
}

fn to_data(input: &Vec<String>) -> Data {
    let index_of_blank_line = input.iter().position(|l| l.is_empty()).unwrap();
    let (points, instructions) = input.split_at(index_of_blank_line);


    let folds: Vec<(char, usize)> = instructions.iter()
        .filter(|i| !i.is_empty())
        .map(|i| i.split_at("fold along ".len()).1)
        .map(|s| s.split("=").collect::<Vec<&str>>())
        .map(|v| (*v.get(0).unwrap(), *v.get(1).unwrap()))
        .map(|(s, i)| (*s.chars().collect::<Vec<char>>().get(0).unwrap(), i.parse::<usize>().unwrap()))
        .collect();

    let ps: Vec<Point> = points.iter()
        .map(|p| p.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .map(|pv| (*pv.get(0).unwrap(), *pv.get(1).unwrap()))
        .map(|t| Point { x: t.0, y: t.1 })
        .collect();


    Data { marked: HashSet::from_iter(ps.into_iter()), folds }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_data_t() {
        let filename = "src/day13/test-input.txt";
        let input = lines_from_file(filename);
        let result = to_data(&input);
        print_paper(&result.marked);
        assert_eq!(18, result.marked.len());
    }

    #[test]
    fn part_a_test_input() {
        let filename = "src/day13/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input, 1);
        assert_eq!(17, result);
    }

    #[test]
    fn part_a_test_input_2() {
        let filename = "src/day13/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input, 2);
        assert_eq!(16, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day13/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input, 1);
        assert_eq!(695, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day13/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        print_paper(&result);
        // GJZGLUPJ
        assert_eq!(89, result.len());
    }
}