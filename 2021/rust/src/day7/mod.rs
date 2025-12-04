use crate::util::lines_from_file;

pub fn day7() {
    println!("== Day 7 ==");
    let input = lines_from_file("src/day7/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> i32 {
    let split: Vec<&str> = input.get(0).map(|s| s.split(",").collect()).unwrap();
    let mut positions: Vec<i32> = split.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    positions.sort();
    let mut fuel = 0;
    let middle = *positions.get(positions.len() / 2).unwrap();
    for pos in &positions {
        fuel += (*pos - middle).abs();
    }

    fuel
}

fn part_b(input: &Vec<String>) -> i32 {
    let split: Vec<&str> = input.get(0).map(|s| s.split(",").collect()).unwrap();
    let mut positions: Vec<i32> = split.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    positions.sort();
    let mut fuel = i32::MAX;

    let biggest_position = *positions.iter().max().unwrap();
    for calculate_for_position in 0..biggest_position {
        let mut cost = 0;
        for pos in &positions {
            let distance = (*pos - calculate_for_position).abs();
            cost += fuel_cost(distance);
        }
        if cost < fuel {
            fuel = cost;
        }
    }


    fuel
}

fn fuel_cost(distance: i32) -> i32 {
    distance * (distance + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day7/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(37, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day7/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(364898, result);
    }

    #[test]
    fn fuel_cost_t() {
        assert_eq!(0, fuel_cost(0));
        assert_eq!(1, fuel_cost(1));
        assert_eq!(1 + 2, fuel_cost(2));
        assert_eq!(1 + 2 + 3, fuel_cost(3));
        assert_eq!(1 + 2 + 3 + 4, fuel_cost(4));
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day7/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(168, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day7/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(104149091, result);
    }
}
