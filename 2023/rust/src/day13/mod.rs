use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day13() {
    println!("== Day 13 ==");
    let input = "src/day13/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

struct Pattern {
    image: Vec<String>,
}

impl Pattern {
    fn of(image: Vec<String>) -> Self {
        Pattern { image }
    }

    fn find_reflections(&self) -> (usize, usize) {
        let rows = Pattern::find_ref(&self.image);
        let rotated = Pattern::rotate(&self.image);
        let cols = Pattern::find_ref(&rotated);
        // println!("Rows reflected: {}", rows);
        // println!("cols reflected: {}", cols);
        // println!("({}, {})",cols, rows);
        if cols > rows {
            (cols, 0)
        } else {
            (0, rows)
        }
    }
    fn find_fixed_reflections(&self) -> (usize, usize) {
        let rows = Pattern::find_fixed_ref(&self.image.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>());
        let rotated = Pattern::rotate(&self.image);
        let cols = Pattern::find_fixed_ref(&rotated.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>());
        // println!("Rows reflected: {}", rows);
        // println!("cols reflected: {}", cols);

        if cols > rows {
            (cols, 0)
        } else {
            (0, rows)
        }
    }
    fn find_ref(image: &Vec<String>) -> usize {
        for (r, row) in image.iter().enumerate() {
            if r + 1 == image.len() { continue; }
            if row.eq(&image[r + 1]) { // Possible reflection found
                // Go up and down to check if we have a full reflection
                for i in 0..image.len() {
                    let ru = r as isize - i as isize;
                    let rd = r + 1 + i;
                    if ru == -1 || rd == image.len() { // We have reached the end of the image
                        return r + 1;
                    }
                    if image[ru as usize] != image[rd] {
                        break;
                    }
                }
            }
        }
        0
    }
    fn find_fixed_ref(image: &Vec<Vec<char>>) -> usize {
        // println!("\n TESTING {:?} \n", image);
        for r in 0..image.len() - 1 {
            let mut smudges = 0;
            for rr in 0..image.len() {
                let up = r as isize - rr as isize;
                let down = r + 1 + rr;
                // println!("up {} down {} || {}", up, down, image.len());
                if 0 <= up && down < image.len() {
                    // println!("u/d {}/{}, len {}", up, down, image.get(r).unwrap().len());
                    // let up_row = image.get(up as usize).unwrap();
                    // let down_row = image.get(down).unwrap();
                    // println!("{:?}    up   {}", up_row, up_row.len());
                    // println!("{:?}    down {}", down_row, down_row.len());
                    for c in 0..image.get(r).unwrap().len() {
                        if image[up as usize][c] != image[down][c] {
                            smudges += 1;
                        }
                    }
                }
            }
            if smudges == 1 {
                return r + 1;
            }
        }
        0
    }
    fn _can_match(a: &String, b: &String) -> (bool, usize) {
        if a.eq(b) {
            return (true, 0);
        }
        let av = a.chars().collect::<Vec<char>>();
        let bv = b.chars().collect::<Vec<char>>();
        let mut different = Vec::new();
        for i in 0..av.len() - 1 {
            if av[i] != bv[i] {
                different.push(i);
            }
        }
        (different.len() < 2, different.len())
    }

    fn rotate(v: &Vec<String>) -> Vec<String> {
        // println!("Rotating {:?}", v.len());
        // for l in v.iter() {
        //     println!("{:?}      {}", l, l.len());
        // }
        let vv = v.iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut rotated = vec![vec!['.'; vv.len()]; vv[0].len()];
        for (y, row) in vv.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                rotated[x][y] = col;
            }
        }
        let strings = rotated.iter().map(|l| l.into_iter().collect()).collect::<Vec<String>>();
        // println!(" Rotated {:?}", strings.len());
        // for l in strings.iter() {
        //     println!("{:?}      {}", l, l.len());
        // }
        strings
    }
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    let file = File::open(input).unwrap();
    let mut patterns = Vec::new();
    let mut tmp = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            patterns.push(Pattern::of(tmp));
            tmp = Vec::new();
        } else {
            tmp.push(line);
        }
    }
    if !tmp.is_empty() {
        patterns.push(Pattern::of(tmp));
    }
    patterns
}

fn part_a(input: &str) -> usize {
    let patterns = parse_patterns(input);
    let sums = patterns.iter()
        .map(Pattern::find_reflections)
        // .map(|a| {
        //     println!("{:?}", a);
        //     a
        // })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    // println!("{:?}", patterns.len());
    sums.0 + (100 * sums.1)
}

fn part_b(input: &str) -> usize {
    let patterns = parse_patterns(input);
    let sums = patterns.iter()
        .map(Pattern::find_fixed_reflections)
        // .map(|a| {
        //     println!("{:?}", a);
        //     a
        // })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    // println!("{:?}", patterns.len());
    sums.0 + (100 * sums.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day13();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day13/input.txt";
        assert_eq!(26957, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day13/input.txt";
        assert_eq!(42695, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day13/test-input.txt";
        let result = part_a(input);
        assert_eq!(405, result);
    }

    #[test]
    fn test_reflect() {
        {
            let pattern = Pattern::of(vec![
                "##.#.##",
                "#.##...",
                ".######",
                "##....#",
                "#.##.#.",
                ".#.####",
                ".#.####",
                "#.##.#.",
                "##....#",
                ".######",
                "#.##..#",
                "##.#.##",
                "..###..",
                "..###..",
                "##.#.##",
                "#.##..#",
                ".######",
            ].iter().map(|s| s.to_string()).collect());
            let result = pattern.find_reflections();
            assert_eq!((0, 13), result);
            let result = pattern.find_fixed_reflections();
            assert_eq!((0, 6), result);
        }
        {
            let pattern = Pattern::of(vec![
                "...##.##.##....",
                ".##.######.##..",
                ".##.#....#.##..",
                "###.######.####",
                "#.##......##.##",
                "..###.##.###...",
                "#####....######",
                "####..##..#####",
                ".###.#....###..",
                "#####....######",
                "..#........#...",
            ].iter().map(|s| s.to_string()).collect());
            let result = pattern.find_reflections();
            assert_eq!((14, 0), result);
            let result = pattern.find_fixed_reflections();
            assert_eq!((7, 0), result);
        }
        {
            let pattern = Pattern::of(vec![
                "..####.....",
                "##....####.",
                "..#..#....#",
                "#.####.##..",
                "#.####.##..",
                "#.####.#.##",
                ".#....#...#",
                "#......#.##",
                "#.####.##..",
                "..####..##.",
                "..#..#..#.#",
                ".#.##.#.##.",
                ".#.##.#.##.",
                "..#..#..###",
                "..####..##.",
            ].iter().map(|s| s.to_string()).collect());
            let result = pattern.find_reflections();
            assert_eq!((4, 0), result);
            let result = pattern.find_fixed_reflections();
            assert_eq!((0, 12), result);
        }
        {
            let pattern = Pattern::of(vec![
                "####...####",
                ".###.#.....",
                ".###.#.....",
                "####...####",
                ".####.##..#",
                ".#..#######",
                "#####...#.#",
                "#.####.##..",
                "#.#...#.##.",
                "#.#####....",
                "...###.#.##",
                "#.#.#.#...#",
                "##.#####.##",
                "#...##.####",
                "#...#..####",
            ].iter().map(|s| s.to_string()).collect());
            let result = pattern.find_reflections();
            assert_eq!((0, 2), result);
            let result = pattern.find_fixed_reflections();
            assert_eq!((0, 14), result);
        }
    }

    #[test]
    fn test_rotate() {
        {
            let orig = vec![
                "#.#",
                ".#.",
                "#.#",
            ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
            let expected = vec![
                "#.#",
                ".#.",
                "#.#",
            ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
            let result = Pattern::rotate(&orig);
            assert_eq!(expected, result);
        }
        {
            let orig = vec![
                "#.#",
                ".#.",
            ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
            let expected = vec![
                "#.",
                ".#",
                "#.",
            ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
            let result = Pattern::rotate(&orig);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day13/test-input.txt";
        let result = part_b(input);
        assert_eq!(400, result);
    }
}