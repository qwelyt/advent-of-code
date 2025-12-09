use crate::util::time;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 9 ==");
    let input = "src/day9/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> isize {
    let coords = coords(input);
    let mut biggest = 0;
    for i in 0..coords.len() {
        let a = coords[i];
        for ii in i + 1..coords.len() {
            let b = coords[ii];
            let l = max(a.0, b.0) - min(a.0, b.0) + 1;
            let w = max(a.1, b.1) - min(a.1, b.1) + 1;
            let area = l * w;
            biggest = max(biggest, area);
        }
    }
    biggest
}

fn part_b(input: &str) -> isize {
    let coords = coords(input);
    let mut biggest = 0;
    for i in 0..coords.len() {
        let a = coords[i];
        for ii in i + 1..coords.len() {
            let b = coords[ii];
            let (x_min, x_max) = (min(a.0, b.0), max(a.0, b.0));
            let (y_min, y_max) = (min(a.1, b.1), max(a.1, b.1));
            let l = x_max - x_min + 1;
            let w = y_max - y_min + 1;
            let area = l * w;
            if within_bounds(&coords, x_min, x_max, y_min, y_max) {
                biggest = max(biggest, area);
            }
        }
    }
    biggest
}

fn within_bounds(
    coords: &Vec<(isize, isize)>,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
) -> bool {
    for i in 0..coords.len() {
        let k0 = coords[i];
        let k1 = coords[(i + 1) % coords.len()];

        if k0.0 == k1.0 {
            // Vertical
            if x_min < k0.0 && k0.0 < x_max {
                let seg_y_min = min(k0.1, k1.1);
                let seg_y_max = max(k0.1, k1.1);

                if seg_y_min < y_max && y_min < seg_y_max {
                    return false;
                }
            }
        } else if k0.1 == k1.1 {
            // Horizontal
            if y_min < k0.1 && k1.1 < y_max {
                let seg_x_min = min(k0.0, k1.0);
                let seg_x_max = max(k0.0, k1.0);

                if seg_x_min < x_max && x_min < seg_x_max {
                    return false;
                }
            }
        }
    }

    let l = x_max - x_min + 1;
    let w = y_max - y_min + 1;
    let test_point = (x_min + l / 2, y_min + w / 2);
    if !inside_polygon(coords, test_point) {
        return false;
    }
    true
}

fn inside_polygon(coords: &Vec<(isize, isize)>, point: (isize, isize)) -> bool {
    let mut intersections = 0;
    for i in 0..coords.len() {
        let p1 = coords[i];
        let p2 = coords[(i + 1) % coords.len()];

        if (p1.1 > point.1) != (p2.1 > point.1) {
            let intersect = p1.0 + (p2.0 - p1.0) * (point.1 - p1.1) / (p2.1 - p1.1);
            if point.0 < intersect {
                intersections += 1;
            }
        }
    }

    intersections % 2 != 0
}

fn coords(input: &str) -> Vec<(isize, isize)> {
    let coords = File::open(input)
        .map(BufReader::new)
        .map(|reader| {
            reader
                .lines()
                .flatten()
                .map(|line| {
                    let (x, y) = line.split_once(",").expect("cannot parse line");
                    let xn = x.parse::<isize>().expect("cannot parse number");
                    let yn = y.parse::<isize>().expect("cannot parse number");
                    (xn, yn)
                })
                .collect::<Vec<(isize, isize)>>()
        })
        .expect("Should have input");
    coords
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
        let input = "src/day9/input.txt";
        assert_eq!(4750297200, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day9/input.txt";
        assert_eq!(1578115935, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day9/test-input.txt";
        let result = part_a(input);
        assert_eq!(50, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day9/test-input.txt";
        let result = part_b(input);
        assert_eq!(24, result);
    }
}
