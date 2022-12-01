use crate::util::lines;

pub fn day___(){
    print!("== Day ___ ==");
    let input = lines("src/day___/input.txt");
    println!("Part A: {}", part_a(& input));
    println!("Part B: {}", part_b(& input));
}

fn part_a(input: &Vec<?>) -> ? {}
fn part_b(input: &Vec<?>) -> ? {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input(){
        let input = lines("src/day___/test-input.txt");
        let result = part_a(&lines);
        assert_eq!(0, result);
    }

    #[test]
    fn part_b_test_input(){
        let input = lines("src/day___/test-input.txt");
        let result = part_b(&lines);
        assert_eq!(0, result);
    }
}