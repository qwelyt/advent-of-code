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

pub fn string_to_i32(input: Vec<String>) -> Vec<i32> {
    input.iter().map(|x| -> i32{ x.parse().unwrap() }).collect()
}
