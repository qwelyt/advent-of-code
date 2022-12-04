use crate::util::{lines, time};

pub fn day4() {
    println!("== Day 4 ==");
    let input = "src/day4/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> i32 {
    let mut tot = 0;
    for line in lines(input).iter() {
        // println!("{}", line);
        let v: Vec<&str> = line.split(",").collect();
        let a = area_bounds(v[0]);
        let b = area_bounds(v[1]);
        // println!("a {:?}, b {:?}", a, b);
        if contains(a, b) || contains(b, a) {
            tot += 1;
            // println!("true");
        }
    }
    tot
}

fn contains(a: (i32, i32), b: (i32, i32)) -> bool {
    if a.0 <= b.0 {
        if a.1 >= b.1 {
            return true;
        }
    }
    false
}

fn area_bounds(s: &str) -> (i32, i32) {
    let v: Vec<i32> = s.split("-").map(|d| d.parse::<i32>().unwrap()).collect();
    (v[0], v[1])
}

fn part_b(input: &str) -> i32 {
    let mut tot = 0;
    for line in lines(input).iter() {
        // println!("{}", line);
        let v: Vec<&str> = line.split(",").collect();
        let a = area_bounds(v[0]);
        let b = area_bounds(v[1]);
        if overlap(a, b) || overlap(b, a) {
            // println!("a {:?}, b {:?}", a, b);
            tot += 1;
            // println!("true");
        }
    }
    tot
}

fn overlap(a: (i32, i32), b: (i32, i32)) -> bool {
    if contains(a, b) { return true; }
    if a.1 >= b.0 && a.0 < b.1 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_a(input);
        assert_eq!(2, result);
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day4/input.txt";
        let result = part_a(input);
        assert_eq!(433, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day4/test-input.txt";
        let result = part_b(input);
        assert_eq!(4, result);
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day4/input.txt";
        let result = part_b(input);
        assert_eq!(852, result);
    }
}