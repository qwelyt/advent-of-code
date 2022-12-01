use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn lines(filename: impl AsRef<Path>) -> Vec<String> {
    let open = File::open(filename).expect("Could not read file");
    let lines: Result<Vec<String>> = BufReader::new(open).lines().collect();
    return lines.expect("Could not read file")
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

pub fn i32_to_string(input: Vec<i32>) -> String{
    input.iter()
        .map(|i| i.to_string())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_test() {
        let lines: Vec<String> = lines(".gitignore");
        assert_eq!(vec!["/target"], lines);
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
    fn to_string(){
        let inp: Vec<i32> = vec![12,33,29,9];
        let result = i32_to_string(inp);
        assert_eq!("1233299", result);
    }

}