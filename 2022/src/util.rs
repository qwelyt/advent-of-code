use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::time::Instant;

pub fn time(f: fn(&str) -> i32, input: &str, part: &str) {
    let start = Instant::now();
    let result = f(input);
    let end = Instant::now();
    println!("Part {}: {}, took {}ns", part, result, end.duration_since(start).as_nanos())
}

pub fn lines(filename: impl AsRef<Path>) -> Vec<String> {
    let open = File::open(filename).expect("Could not read file");
    let lines: Result<Vec<String>> = BufReader::new(open).lines().collect();
    return lines.expect("Could not read file");
}

pub fn lines_as_i32(filename: impl AsRef<Path>) -> Vec<Option<i32>> {
    let open = File::open(filename).expect("Could not read file");
    let a: Vec<Option<i32>> = BufReader::new(open)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.parse::<i32>().unwrap())
            }
        })
        .collect();
    return a
}

pub fn vecs(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut v: Vec<Vec<String>> = Vec::new();
    let mut w = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            v.push(w);
            w = Vec::new();
            continue;
        } else {
            w.push(line.to_string());
        }
    }
    v.push(w);
    v
}

pub fn vecs_i32(numbers: &Vec<Option<i32>>) -> Vec<Vec<i32>> {
    let mut v: Vec<Vec<i32>> = Vec::new();
    let mut w = Vec::new();
    for n in numbers.iter() {
        if n.is_none() {
            v.push(w);
            w = Vec::new();
        } else {
            w.push(n.unwrap());
        }
    }
    v.push(w);
    v
}

pub fn to_i32(strings: &Vec<String>) -> Vec<i32> {
    strings.iter().map(|s| s_to_i32(s)).collect()
}

pub fn cs_to_i32(chars: &Vec<char>) -> Vec<i32> {
    chars.iter().map(|s| c_to_i32(s)).collect()
}

pub fn s_to_i32(string: &str) -> i32 {
    string.parse::<i32>().unwrap()
}

pub fn c_to_i32(char: &char) -> i32 {
    char.to_digit(10).unwrap() as i32
}

pub fn i32_to_string(input: Vec<i32>) -> String {
    input.iter()
        .map(|i| i.to_string())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_test() {
        let lines: Vec<String> = lines("test-resources/string-input.txt");
        let expected = vec![
            "Hello",
            "This",
            "is",
            "a",
            "some",
            "",
            "lines",
        ];
        assert_eq!(expected, lines);
    }

    #[test]
    fn lines_as_i32_test() {
        let lines = lines_as_i32("test-resources/i32-input.txt");
        let expected = vec![
            Some(8),
            Some(92342),
            None,
            Some(34),
            Some(2),
            None,
            Some(3),
        ];
        assert_eq!(expected, lines);
    }

    #[test]
    fn vecs_test() {
        let lines = vec!["a".to_string(), "b".to_string(), "".to_string(), "c".to_string(), "d".to_string()];
        let vecs = vecs(&lines);
        assert_eq!(vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string(), "d".to_string()]], vecs);
    }

    #[test]
    fn to_i32_test() {
        let str: Vec<String> = vec!["23".to_string(), "99".to_string()];
        let nr: Vec<i32> = to_i32(&str);
        assert_eq!(vec![23, 99], nr)
    }

    #[test]
    fn c_to_i32_test() {
        let inp: Vec<char> = vec!['2', '9'];
        let nr: Vec<i32> = cs_to_i32(&inp);
        assert_eq!(vec![2, 9], nr)
    }

    #[test]
    fn to_string() {
        let inp: Vec<i32> = vec![12, 33, 29, 9];
        let result = i32_to_string(inp);
        assert_eq!("1233299", result);
    }
}