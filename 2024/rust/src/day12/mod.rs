use crate::util::time;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 12 ==");
    let input = "src/day12/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    parse_plots(input).iter()
        .map(|(area, perimeter, _)| area * perimeter)
        .sum()
}
fn part_b(input: &str) -> usize {
    parse_plots(input).iter()
        .map(|(area, _, sides)| area * sides)
        .sum()
}

fn parse_plots(input: &str) -> Vec<(usize, usize, usize)> {
    let garden = File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .unwrap();

    let dr = [-1, 0, 1, 0];
    let dc = [0, -1, 0, 1];
    let mut seen = HashSet::new();
    let mut plots = Vec::new();
    for (r, row) in garden.iter().enumerate() {
        for (c, crop) in row.iter().enumerate() {
            if seen.contains(&(r, c)) { continue; }
            seen.insert((r, c));
            let mut plot = HashSet::new();
            plot.insert((r as i32, c as i32));

            let mut q = VecDeque::new();
            q.push_back((r as i32, c as i32));
            while let Some((curr_r, curr_c)) = q.pop_front() {
                for d in 0..dr.len() {
                    let nr = curr_r + dr[d];
                    let nc = curr_c + dc[d];

                    if nr < 0 || nc < 0 { continue; }
                    if nr as usize >= garden.len() || nc as usize >= garden[nr as usize].len() { continue; }
                    if garden[nr as usize][nc as usize] != *crop { continue; }
                    if plot.contains(&(nr, nc)) { continue; }
                    if seen.contains(&(nr as usize, nc as usize)) { continue; }

                    seen.insert((nr as usize, nc as usize));
                    plot.insert((nr, nc));
                    q.push_back((nr, nc));
                }
            }
            plots.push((plot.len(), perimeter(&plot), sides(&plot)));
        }
    }
    plots
}

fn perimeter(plot: &HashSet<(i32, i32)>) -> usize {
    let mut perim = 0;
    let dr = [-1,0,1,0];
    let dc = [0,-1,0,1];

    for (r,c) in plot.iter() {
        let mut free_sides = 4;
        for d in 0..dr.len() {
            let nr = *r+dr[d];
            let nc = *c+dc[d];
            if plot.contains(&(nr,nc)) {
                free_sides -= 1;
            }
        }
        perim += free_sides as usize;
    }

    perim
}
fn sides(plot: &HashSet<(i32, i32)>) -> usize {
    // println!("## Plot {:?} ##", plot);
    // _print_plot(&plot);
    if plot.len() == 1 { return 4; }

    let mut sides = 0;

    let dr = [-1,0,1,0];
    let dc = [0,-1,0,1];

    for d in 0..dr.len() {
        // println!("dir: {},{}", dr[d], dc[d]);
        let mut outside_squares = HashSet::new();
        for (r,c) in plot.iter() {
            let nr = *r+dr[d];
            let nc = *c+dc[d];
            let n = (nr,nc);
            if !plot.contains(&n) {
                outside_squares.insert(n);
            }
        }
        // println!("os {:?}", outside_squares);
        let mut actually_inside = HashSet::new();
        for side in outside_squares.iter() {
            let mut tmp = (side.0+dc[d], side.1+dr[d]);
            while outside_squares.contains(&tmp) {
                actually_inside.insert(tmp);
                tmp = (tmp.0+dc[d], tmp.1+dr[d]);
            }
        }
        // println!("ai {:?}", actually_inside);
        let actual_sides = outside_squares.difference(&actually_inside)
            .collect::<HashSet<&(i32, i32)>>();
        // println!("as {:?}", actual_sides);
        sides += actual_sides.len();

    }

    // println!();
    // _print_plot(&plot);
    // println!("Corners: {}", corners);
    // println!();

    sides
}

fn _print_plot(plot: &HashSet<(i32, i32)>) {
    let min_x = plot.iter().map(|(x,_)| *x).min().unwrap();
    let max_x = plot.iter().map(|(x,_)| *x).max().unwrap();
    let min_y = plot.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = plot.iter().map(|(_, y)| *y).max().unwrap();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if plot.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
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
        let input = "src/day12/input.txt";
        assert_eq!(1533644, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day12/input.txt";
        assert_eq!(936718, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day12/test-input.txt";
        let result = part_a(input);
        assert_eq!(1930, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day12/test-input.txt";
        let result = part_b(input);
        assert_eq!(1206, result);
    }
}
