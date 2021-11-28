use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let lines = lines_from_file("src/data.txt").expect("Bork");
    for line in lines {
        println!("{}", line)
    }
}

// fn read_file(file: String) -> Vec<String> {
//     println!("In file {}",file);
//
//     let contents = fs::read_to_string(file)
//         .expect("Something went wrong reading the file");
//
//     contents.lines().collect()
// }
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
