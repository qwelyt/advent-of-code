use crate::util::time;

pub fn day___() {
    println!("== Day ___ ==");
    let input = "src/day___/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> i32 { 0 }

fn part_b(input: &str) -> i32 { 0 }

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day___();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day___/input.txt";
        assert_eq!(0, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day___/input.txt";
        assert_eq!(0, part_b(input));
    }

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