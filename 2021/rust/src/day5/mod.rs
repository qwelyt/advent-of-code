use std::collections::HashMap;

use crate::util::{lines_from_file, string_to_i32};

pub fn day5() {
    println!("== Day 5 ==");
    let input = lines_from_file("src/day5/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Debug)]
struct Line {
    from: Point,
    to: Point,
}


fn part_a(input: &Vec<String>) -> usize {
    let mut map: HashMap<Point, i32> = HashMap::new();
    let lines: Vec<Line> = to_line_points(input);
    for line in lines {
        // println!("Working on {:?}",line);
        if line.from.x != line.to.x && line.from.y != line.to.y {
            // Diagonal line
            // println!("skipping");
            continue;
        }
        let xf = i32::min(line.from.x, line.to.x);
        let xt = i32::max(line.from.x, line.to.x);
        let yf = i32::min(line.from.y, line.to.y);
        let yt = i32::max(line.from.y, line.to.y);

        for x in xf..=xt {
            for y in yf..=yt {
                let point = Point { x, y };
                // println!("{:?}", point);
                let value = *map.get(&point).unwrap_or(&0);
                map.insert(point, value + 1);
            }
        }
    }
    // println!("{:?}", map);


    map.iter()
        .filter(|(_, value)| **value > 1)
        .count()
}

fn part_b(input: &Vec<String>) -> usize {
    let mut map: HashMap<Point, i32> = HashMap::new();
    let lines: Vec<Line> = to_line_points(input);
    for line in lines {
        if line.from.x != line.to.x && line.from.y != line.to.y {
            // Diagonal
            let mut y = line.from.y;
            let mut x = line.from.x;
            if line.from.x < line.to.x {
                // println!("\\ {:?}", line);
                if line.from.y < line.to.y {
                    // println!("down");
                    while x <= line.to.x && y <= line.to.y {
                        let point = Point { x, y };
                        // println!("{:?}", point);
                        let value = *map.get(&point).unwrap_or(&0);
                        map.insert(point, value + 1);
                        x += 1;
                        y += 1;
                    }
                } else {
                    // println!("up");
                    while x <= line.to.x && y >= line.to.y {
                        let point = Point { x, y };
                        // println!("{:?}", point);
                        let value = *map.get(&point).unwrap_or(&0);
                        map.insert(point, value + 1);
                        x += 1;
                        y -= 1;
                    }
                }
            } else {
                // println!("/ {:?}", line);
                if line.from.y < line.to.y {
                    // println!("down");
                    while x >= line.to.x && y <= line.to.y {
                        let point = Point { x, y };
                        // println!("{:?}", point);
                        let value = *map.get(&point).unwrap_or(&0);
                        map.insert(point, value + 1);
                        x -= 1;
                        y += 1;
                    }
                } else {
                    // println!("up");
                    while x >= line.to.x && y >= line.to.y {
                        let point = Point { x, y };
                        // println!("{:?}", point);
                        let value = *map.get(&point).unwrap_or(&0);
                        map.insert(point, value + 1);
                        x -= 1;
                        y -= 1;
                    }
                }
            }
        } else {
            // Horizontal / vertical
            // println!("+ {:?}", line);
            let xf = i32::min(line.from.x, line.to.x);
            let xt = i32::max(line.from.x, line.to.x);
            let yf = i32::min(line.from.y, line.to.y);
            let yt = i32::max(line.from.y, line.to.y);

            for x in xf..=xt {
                for y in yf..=yt {
                    let point = Point { x, y };
                    // println!("{:?}", point);
                    let value = *map.get(&point).unwrap_or(&0);
                    map.insert(point, value + 1);
                }
            }
        }
    }

    map.iter()
        .filter(|(_, value)| **value > 1)
        .count()
}


fn to_line_points(input: &Vec<String>) -> Vec<Line> {
    input.iter()
        .map(|s| to_line_point(s.trim()))
        .collect()
}

fn to_line_point(input: &str) -> Line {
    let points: Vec<&str> = input.split(" -> ").into_iter().map(|s| s.trim()).collect();
    let from = to_point(points.get(0).unwrap());
    let to = to_point(points.get(1).unwrap());

    Line {
        from,
        to,
    }
}

fn to_point(points: &str) -> Point {
    let is: Vec<i32> = points.split(",")
        .into_iter()
        .map(|s| s.trim())
        .map(|s| string_to_i32(s))
        .collect();

    Point {
        x: *is.get(0).unwrap(),
        y: *is.get(1).unwrap(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_point_t() {
        let str = "1,2";
        let expected = Point { x: 1, y: 2 };
        let result = to_point(str);
        assert_eq!(expected, result);
    }

    #[test]
    fn to_line_point_t() {
        let str = "1,2 -> 1,9";
        let expected = Line {
            from: Point { x: 1, y: 2 },
            to: Point { x: 1, y: 9 },
        };
        let result = to_line_point(str);
        assert_eq!(expected, result)
    }

    #[test]
    fn to_line_points_t() {
        let str = vec![
            "1,2 -> 1,9".to_string(),
            "3,8 -> 4,7".to_string(),
        ];
        let expected = vec![
            Line {
                from: Point { x: 1, y: 2 },
                to: Point { x: 1, y: 9 },
            },
            Line {
                from: Point { x: 3, y: 8 },
                to: Point { x: 4, y: 7 },
            },
        ];
        let result = to_line_points(&str);
        assert_eq!(expected, result)
    }

    #[test]
    fn part_a_test_input() {
        let filename = "src/day5/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(5, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day5/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(7674, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day5/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(12, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day5/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(20898, result);
    }
}
