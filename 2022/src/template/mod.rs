use crate::util::lines;

pub fn day___() {
    println!("== Day ___ ==");
    let input = "src/day___/input.txt";
    println!("Part A: {}", part_a(input));
    println!("Part B: {}", part_b(input));
}

fn part_a(input: &str) -> i32 { 0 }

fn part_b(input: &str) -> i32 { 0 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let input = "src/day___/test-input.txt";
        let result = part_a(input);
        assert_eq!(0, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day___/test-input.txt";
        let result = part_b(input);
        assert_eq!(0, result);
    }
}