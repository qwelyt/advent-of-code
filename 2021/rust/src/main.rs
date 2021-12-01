use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};



fn main() {
    println!("Day 1");
    partA();
    partB();


}

fn partA(){
    println!("=== Part A===");
    //let lines = lines_from_file("src/test-input.txt").expect("Bork");
    let lines = lines_from_file("src/input.txt").expect("Bork");
    let numbers: Vec<i32> = lines.iter().map(|x|->i32{x.parse().unwrap()}).collect();

    let mut last: i32 = numbers[0];
    let mut increases: i32 = 0;
    println!("last {}",last);
    for (i,x) in numbers.iter().enumerate() {
        if i == 0 { 
            continue;
        }

        if x > &last {
            increases+=1;
        }
        last = *x;
    }

    println!("Increases: {}", increases);
}

struct Point(i32,i32,i32);

fn partB() {
    println!("=== Part B ===");
    //let lines = lines_from_file("src/test-input.txt").expect("Bork");
    let lines = lines_from_file("src/input.txt").expect("Bork");
    let numbers: Vec<i32> = lines.iter().map(|x|->i32{x.parse().unwrap()}).collect();

    let mut points: Vec<Point> = Vec::new();
    let mut addedPoints: Vec<i32> = Vec::new();

    for (i,x) in numbers.iter().enumerate() {
        if i + 2 > numbers.len()-1 {
            break;
        }
        points.push(Point(*x, numbers[i+1], numbers[i+2]));
        addedPoints.push(*x + numbers[i+1] + numbers[i+2]);
    }

    println!("Num points: {}", points.len());


    let mut last: i32 = addedPoints[0];
    let mut increases: i32 = 0;
    println!("last {}",last);
    for (i,x) in addedPoints.iter().enumerate() {
        if i == 0 { 
            continue;
        }

        if x > &last {
            increases+=1;
        }
        last = *x;
    }

    println!("Increases: {}", increases);
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}