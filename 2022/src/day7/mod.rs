use std::collections::HashMap;

use crate::util::{lines, time};

pub fn day7() {
    println!("== Day 7 ==");
    let input = "src/day7/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}


fn part_a(input: &str) -> usize {
    let lines = lines(input);
    let dir_sizes = dir_sizes(lines);
    // for (a,b) in dir_sizes.iter(){
    //     println!("{}: {}", a,b);
    // }
    dir_sizes.iter().filter(|e| *e.1 < 100_000).map(|e| e.1).sum()
}

fn part_b(input: &str) -> usize {
    let lines = lines(input);
    let dir_sizes = dir_sizes(lines);
    let free = 70_000_000 - dir_sizes.get("").unwrap();
    let needed = 30_000_000 - free;
    // println!("{} :: {}", free, needed);
    // for (a,b) in dir_sizes.iter(){
    //     println!("{}: {}", a,b);
    // }
    *dir_sizes.iter().filter(|e| *e.1 >= needed).map(|e| e.1).min().unwrap()
}


fn dir_sizes(lines: Vec<String>) -> HashMap<String, usize> {
    let mut path = Vec::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    for line in lines.iter() {
        let l = line.split(" ").collect::<Vec<&str>>();
        if l[0].starts_with("$") { // Command
            if l[1].eq("cd") { // Move
                if l[2].eq("..") {
                    path.pop();
                    // println!("Go up to {:?}", path);
                } else {
                    if l[2].eq("/") {
                        path.clear();
                        path.push("");
                    } else {
                        // println!("Go to {}", l[2]);
                        path.push(l[2]);
                    }
                }
            } else { // ls
                // println!("Do ls on {:?}", path);
            }
        } else { // ls
            // println!("Found {}", line);
            if l[0].eq("dir") { // Dir
            } else { // File
                let file_size = l[0].parse::<usize>().unwrap();
                // For all the directories that we have
                // Add the size of this file to them
                // If we are at /a/b/c and find file d.txt with size 10
                // The we add 10 to:
                //      /
                //      /a
                //      /a/b
                //      /a/b/c
                // All these get an added 10 to their sizes.
                for i in 0..path.len() {
                    let dir = path[0..=i].join("/");
                    dir_sizes.entry(dir)
                        .and_modify(|size| *size += file_size)
                        .or_insert(file_size);
                }
            }
        }
    }
    dir_sizes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day7();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day7/input.txt";
        assert_eq!(1453349, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day7/input.txt";
        assert_eq!(2948823, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day7/test-input.txt";
        let result = part_a(input);
        assert_eq!(95437, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day7/test-input.txt";
        let result = part_b(input);
        assert_eq!(24933642, result);
    }
}