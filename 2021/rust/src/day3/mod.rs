use crate::util::{char_to_i32, lines_from_file, vec_to_string};

pub fn day3() {
    println!("== Day 3 ==");
    let input = lines_from_file("src/day3/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> i32 {
    let x: Vec<Vec<i32>> = input.iter()
        .map(|s| s.chars()
            .map(|c| char_to_i32(c))
            .collect())
        .collect();

    let mut sum_of_place = vec![0; x[0].len()];

    for line in x {
        // println!("{:?}",line);
        for (index, value) in line.iter().enumerate() {
            sum_of_place[index] += *value;
        }
    }
    // println!("{:?}",sum_of_place);
    let mut gamma: Vec<i32> = Vec::new();
    let mut sigma: Vec<i32> = Vec::new();

    let input_length: i32 = input.len() as i32;
    for value in sum_of_place {
        if value > input_length / 2 {
            gamma.push(1);
            sigma.push(0);
        } else {
            gamma.push(0);
            sigma.push(1);
        }
    }
    // println!("{:?}",gamma);
    // println!("{:?}",sigma);4139586

    let gamma_str = vec_to_string(gamma);
    let sigma_str = vec_to_string(sigma);

    let gamma_val = i32::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let sigma_val = i32::from_str_radix(sigma_str.as_str(), 2).unwrap();
    // println!("{}", gamma_val);


    return sigma_val * gamma_val;
}

fn part_b(input: &Vec<String>) -> i32 {
    let oxygen = reduce(input, 0, true);
    let carbon = reduce(input, 0, false);
    // println!("{:?}", oxygen);
    // println!("{:?}", carbon);

    let o = i32::from_str_radix(&oxygen[0], 2).unwrap();
    let c = i32::from_str_radix(&carbon[0], 2).unwrap();
    // println!("{:?}", o);
    // println!("{:?}", c);
    return c * o;
}

fn reduce(input: &Vec<String>, index: i32, most_or_least: bool) -> Vec<String> {
    // println!("At index {}  and most/least is {} and got this as input: {:?}", index, most_or_least,input);
    // Loop over this until you only have one row in the response
    if input.len() == 1 {
        return input.to_vec();
    }
    let mut every_row_at_index = Vec::new();
    for line in input {
        let c = line.chars().nth(index as usize).unwrap();
        every_row_at_index.push(c);
    }
    // println!("Every row at index {} is {:?}", index, every_row_at_index);
    let common: char;
    if most_or_least {
        common = most_common(every_row_at_index);
    } else {
        common = least_common(every_row_at_index);
    }
    // println!("Common was {} and {}", most_or_least, common);
    return reduce(&filter(input, common, index), index + 1, most_or_least);
}

fn filter(input: &Vec<String>, look_for: char, at_index: i32) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for line in input {
        if line.chars().nth(at_index as usize).unwrap() == look_for {
            ret.push(line.parse().unwrap());
        }
    }
    ret
}

fn least_common(input: Vec<char>) -> char {
    match most_common(input) {
        '1' => '0',
        '0' => '1',
        _ => '1'
    }
}

fn most_common(input: Vec<char>) -> char {
    let sum: i32 = input.iter()
        .map(|c| char_to_i32(*c))
        .sum();

    let in_len: f64 = (input.len() as f64) / 2 as f64;
    let usum = sum as f64;

    // println!("len {}, In_len {} and usum {}", input.len(), in_len, usum);

    if usum < in_len {
        return '0';
    }
    return '1';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day3/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(198, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day3/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(4139586, result);
    }

    #[test]
    fn most_common_test() {
        let vec1 = vec!['1', '0', '1'];
        let common = most_common(vec1);
        assert_eq!('1', common)
    }

    #[test]
    fn filter_lines() {
        let input: Vec<String> = vec![
            "101".to_string(),
            "001".to_string(),
            "111".to_string(),
            "000".to_string(),
        ];
        let expected = vec![
            "101".to_string(),
            "111".to_string(),
        ];
        let result = filter(&input, '1', 0);
        assert_eq!(expected, result)
    }

    #[test]
    fn reduce_lines() {
        let input: Vec<String> = vec![
            "101".to_string(),
            "000".to_string(),
        ];

        let result = reduce(&input, 0, true);
        let result2 = reduce(&input, 0, false);
        assert_eq!(vec!["101".to_string()], result);
        assert_eq!(vec!["000".to_string()], result2);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day3/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(230, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day3/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(1800151, result);
    }
}