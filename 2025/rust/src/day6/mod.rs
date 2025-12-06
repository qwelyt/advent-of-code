use crate::util::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 6 ==");
    let input = "src/day6/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> u64 {
    parse_columns(input)
        .iter()
        .map(|column| Column::new(column))
        .map(|column| column.calculate())
        .sum()
}

fn part_b(input: &str) -> u64 {
    parse_columns_as_groups(input)
        .iter()
        .map(|column| transform(column))
        .map(|column| Column::new(&column))
        .map(|column| column.calculate())
        .sum()
}

fn parse_columns(input: &str) -> Vec<Vec<String>> {
    let mut columns: Vec<Vec<String>> = Vec::new();
    let open = File::open(input).expect("Could not read file");
    for (i, line) in BufReader::new(open).lines().flatten().enumerate() {
        let curr = line.split_whitespace().collect::<Vec<&str>>();
        if i == 0 {
            for _ in 0..curr.len() {
                columns.push(Vec::new())
            }
        }
        for (ii, c) in curr.iter().enumerate() {
            columns[ii].push(c.to_string());
        }
    }
    columns
}

fn parse_columns_as_groups(input: &str) -> Vec<Vec<String>> {
    let lines = File::open(input)
        .map(|file| {
            BufReader::new(file)
                .lines()
                .flatten()
                .collect::<Vec<String>>()
        })
        .expect("Could not read lines");

    let mut column_start: Vec<usize> = Vec::new();
    let (last_line, _) = lines.split_last().expect("Should be a last line");
    for (i, c) in last_line.chars().enumerate() {
        match c {
            '+' | '*' => {
                column_start.push(i);
            }
            _ => {}
        }
    }

    let mut columns: Vec<Vec<String>> = vec![Vec::new(); column_start.len()];
    for line in lines.iter() {
        for i in 0..column_start.len() {
            let start = column_start[i];
            let end = if i + 1 >= column_start.len() {
                line.len()
            } else {
                column_start[i + 1] - 1
            };
            let sub = &line[start..end];
            columns[i].push(sub.to_string());
        }
    }

    let last_column = columns.last().expect("Should be a last column");

    let column_length = last_column
        .iter()
        .map(|s| s.len())
        .max()
        .expect("Should have a length");
    let fixed_last = last_column
        .iter()
        .map(|s| {
            let len = s.len();
            s.clone() + " ".repeat(column_length - len).as_str()
        })
        .collect::<Vec<String>>();

    columns.pop();
    columns.push(fixed_last);

    columns
}
fn transform(input: &Vec<String>) -> Vec<String> {
    // input: [123, 4, 46, *]

    // (*, [123, 4, 46])
    // 123
    //   4
    //  46
    let (operator, numbers) = input.split_last().expect("Empty input");

    // [[], [], []] - 123.len() == 3
    let max_length = numbers
        .iter()
        .map(|s| s.len())
        .max()
        .expect("Max is missing");
    let mut v: Vec<Vec<char>> = vec![Vec::new(); max_length];

    for number in numbers.iter() {
        for i in 0..max_length {
            v[i].push(number.chars().nth(i).unwrap());
        }
    }

    // [346, 24, 1]
    let mut new_numbers = v
        .iter()
        .map(|s| s.iter().collect::<String>())
        .collect::<Vec<String>>();

    new_numbers.push(operator.to_string());

    new_numbers
}

struct Column {
    numbers: Vec<u64>,
    operator: char,
}

impl Column {
    fn new(input: &Vec<String>) -> Column {
        let (op, numbers) = input.split_last().unwrap();
        let operator = op.chars().next().unwrap();
        let numbers = numbers
            .iter()
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect();
        Self { numbers, operator }
    }

    fn calculate(self) -> u64 {
        match self.operator {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => {
                panic!()
            }
        }
    }
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
        let input = "src/day6/input.txt";
        assert_eq!(4719804927602, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day6/input.txt";
        assert_eq!(9608327000261, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day6/test-input.txt";
        let result = part_a(input);
        assert_eq!(4277556, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day6/test-input.txt";
        let result = part_b(input);
        assert_eq!(3263827, result);
    }

    #[test]
    fn parse_columns_as_groups_test() {
        let input = "src/day6/test-input.txt";
        let result = parse_columns_as_groups(input);
        let expected: Vec<Vec<String>> = vec![
            vec!["123", " 45", "  6", "*  "],
            vec!["328", "64 ", "98 ", "+  "],
            vec![" 51", "387", "215", "*  "],
            vec!["64 ", "23 ", "314", "+  "],
        ]
        .iter()
        .map(|s| s.iter().map(|ss| ss.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
        assert_eq!(expected, result);
    }

    #[test]
    fn transform_test() {
        let input = vec!["123", " 45", "  6", "*  "]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = transform(&input);
        let expected = vec!["1  ", "24 ", "356", "*  "]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(expected, result);

        let input = vec!["328", "64 ", "98 ", "+  "]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = transform(&input);
        let expected = vec!["369", "248", "8  ", "+  "]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(expected, result);
    }
}
