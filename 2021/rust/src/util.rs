use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    lines(filename).expect("Could not read file")
}

fn lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn strings_to_i32(input: Vec<String>) -> Vec<i32> {
    input.iter().map(|x| string_to_i32(x)).collect()
}

pub fn string_to_i32(string: &str) -> i32 {
    string.parse::<i32>().unwrap()
}

pub fn char_to_i32(c: char) -> i32 {
    c.to_digit(10).unwrap() as i32
}

pub fn vec_to_string(input: Vec<i32>) -> String {
    input.into_iter().map(|i| i.to_string()).collect::<String>()
}
