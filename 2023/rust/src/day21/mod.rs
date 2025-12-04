use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day21() {
    println!("== Day 21 ==");
    let input = "src/day21/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    solve_a(input, 64)
}

fn part_b(input: &str) -> usize {
    solve_b(input, 26501365)
}

fn solve_a(input: &str, steps: u32) -> usize {
    let garden = Garden::parse(input);
    // println!("{:?}", garden);
    garden.possible_tiles(steps)
        .values()
        .filter(|distance| **distance % 2 == 0)
        .count()
}

fn solve_b(input: &str, steps: u32) -> usize {
    let garden = Garden::parse(input);
    garden.possible_tiles_big(steps)
}

type Pos = (i32, i32);

fn add(a: Pos, b: Pos) -> Pos {
    let y = a.0 + b.0;
    let x = a.1 + b.1;
    (y, x).into()
}

#[derive(Debug, Clone)]
struct Garden {
    map: Vec<Vec<char>>,
    start: Pos,
    size: Pos,
}

impl Garden {
    fn parse(input: &str) -> Self {
        let mut map = File::open(input)
            .map(|f| BufReader::new(f).lines().flatten()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
            )
            .unwrap();

        let mut start_pos = None;
        for (r, l) in map.iter().enumerate() {
            for (c, ch) in l.iter().enumerate() {
                if ch == &'S' {
                    start_pos = Some((r, c));
                }
            }
        }
        let start_pos = start_pos.unwrap();
        map[start_pos.0][start_pos.1] = '.';

        let start = (start_pos.0 as i32, start_pos.1 as i32).into();
        let size = (map.len() as i32, map[0].len() as i32).into();
        Self { map, start, size }
    }

    fn walkable(&self, pos: &Pos) -> bool {
        let y = pos.0;
        let x = pos.1;
        return if y < 0
            || x < 0
            || y as usize >= self.map.len()
            || x as usize >= self.map[y as usize].len() {
            false
        } else if self.map[y as usize][x as usize] == '.' {
            true
        } else {
            false
        };
    }

    fn possible_tiles(&self, steps: u32) -> HashMap<Pos, u32> {
        let mut reachable = HashMap::new();

        let delta = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut queue = VecDeque::<(Pos, u32)>::new();
        queue.push_back((self.start, 0));
        // reachable.insert(self.start, 0);

        // self._print(&reachable);

        let mut new_starts = HashSet::new();
        for _i in 0..=steps {
            new_starts.into_iter().for_each(|p| queue.push_back(p));
            new_starts = HashSet::new();
            if queue.len() == 0 {
                break;
            }
            while let Some((start, distance)) = queue.pop_front() {
                if reachable.contains_key(&start) {
                    continue;
                }
                reachable.insert(start, distance);
                for d in delta.iter() {
                    let new_pos = add(start, *d);
                    // print!("Going from {:?} to {:?}", start, new_pos);
                    if self.walkable(&new_pos) {
                        // println!(" -> Works! {:?}", self.map[new_pos.0 as usize][new_pos.1 as usize]);
                        new_starts.insert((new_pos, distance + 1));
                    } else {
                        // println!(" -> NO! walkable {:?}, reached: {:?}", self.walkable(&new_pos), reachable.contains(&new_pos));
                    }
                }
            }
            // println!("{:?} :: NS size: {:?}, reached: {:?}", i, new_starts.len(), reachable.len());
            // println!("=== {}", i);
            // self._print(&new_starts);
            // println!("===");
            // self._print(&reachable);
            // println!();
        }
        // let keys = reachable.keys().map(|k| *k).collect::<HashSet<Pos>>();
        // self._print(&keys);
        // println!("===");
        // println!("===");
        // self._print(&new_starts.iter().map(|t| t.0).collect());
        // for e in reachable.iter() {
        //     println!("{:?}", e);
        // }
        // new_starts.len()
        reachable
    }
    fn possible_tiles_big(&self, steps: u32) -> usize {
        let y_tiles_to_edge = (self.size.0 - self.start.0) as u32;
        let x_tiles_to_edge = (self.size.1 - self.start.1) as u32;
        assert_eq!(y_tiles_to_edge, x_tiles_to_edge);
        if steps <= y_tiles_to_edge {
            return self.possible_tiles(steps)
                .values()
                .filter(|distance| **distance % 2 == 0)
                .count();
        }
        let full_walk = self.possible_tiles(y_tiles_to_edge * 2);
        let even = full_walk.values()
            .filter(|distance| **distance % 2 == 0)
            .count();
        let odd = full_walk.values()
            .filter(|distance| **distance % 2 == 1)
            .count();
        println!("len:  {:?}", full_walk.len());
        println!("even: {:?}", even);
        println!("odd:  {:?}", odd);
        // self.possible_tiles()
        0
    }

    fn _print(&self, positions: &HashSet<Pos>) {
        println!("{:?}: {:?}", positions.len(), positions);
        for (y, l) in self.map.iter().enumerate() {
            let mut v = Vec::new();
            for (x, c) in l.iter().enumerate() {
                if positions.contains(&(y as i32, x as i32).into()) {
                    v.push('O');
                } else {
                    v.push(*c);
                }
            }
            println!("{:?}", v.iter().collect::<String>());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day21();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day21/input.txt";
        assert_eq!(3729, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day21/input.txt";
        assert_eq!(0, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day21/test-input.txt";
        let result = solve_a(input, 6);
        assert_eq!(16, result);
    }

    #[test]
    fn garden_test() {
        let input = "src/day21/input.txt";
        let garden = Garden::parse(input);
        let take_steps = 26501365;
        println!("size {:?}", garden.size);
        println!("start {:?}", garden.start);
        assert_eq!(garden.size.0, garden.size.1);
        println!("Total: {:?}", garden.size.0 as usize * garden.size.1 as usize);
        let max_possible_steps_until_edge = garden.size.0 - garden.start.0;
        println!("{:?}", max_possible_steps_until_edge);
        println!("Num gardens in straight line {:?}", (take_steps - garden.start.1) / garden.size.1);
        println!("Even garden? {:?} -- {:?}", garden.size.0 % 2 == 0, (garden.size.0 * garden.size.1) % 2 == 0);
        let full_walk = garden.possible_tiles(take_steps as u32);
        // Basically proves that 21.1 is walking to the edge
        // So walking outside the edge would be like walking inside, kinda
        let edge_walk = full_walk.values()
            .filter(|v| **v < max_possible_steps_until_edge as u32)
            .filter(|v| **v % 2 == 0)
            .count();
        assert_eq!(3729, edge_walk);

        let corners = full_walk.values()
            .filter(|v| **v >= max_possible_steps_until_edge as u32)
            .collect::<Vec<&u32>>();
        let corners_odd = full_walk.values()
            .filter(|v| **v >= max_possible_steps_until_edge as u32)
            .filter(|v| **v % 2 == 1)
            .count();
        let corners_even = full_walk.values()
            .filter(|v| **v >= max_possible_steps_until_edge as u32)
            .filter(|v| **v % 2 == 0)
            .count();

        println!("Corners: {:?}, odd {:?}, even {:?}", corners.len(), corners_odd, corners_even);
        println!("{:?}", corners);

        let total = full_walk.values().collect::<Vec<&u32>>();
        let even = full_walk.values()
            .filter(|v| **v % 2 == 0)
            .count();
        let odd = full_walk.values()
            .filter(|v| **v % 2 == 1)
            .count();
        println!("Total: {:?}, odd {:?}, even {:?}", total.len(), odd, even);
        println!("{:?}", total);

        /*
                 /\
                /  \
               /    \
               \    /
                \  /
                 \/    Full gardens will be cut off....
         */
        let total_gardens = ((take_steps - garden.start.1) / garden.size.1) as usize;
        let all_odds = (total_gardens + 1) * total_gardens * odd;
        let all_evens = total_gardens * total_gardens * even;
        let all_corners = (total_gardens + 1) * corners_odd + total_gardens * corners_even;

        let ans = all_odds + all_evens - all_corners;
        println!("{:?} WRONG", ans);


        // let all_corners = ((total_gardens+1)*corners_odd) - total_gardens*corners_even;
        let all_odd_corners = (total_gardens + 1) * corners_odd;
        let all_even_corners = total_gardens * corners_even;
        let ans = all_odds + all_evens - all_odd_corners + all_even_corners;
        println!("{:?} WRONG TOO LOW", ans);

        let total_gardens = ((take_steps - garden.start.1) / garden.size.1) as usize;
        let extra_garden = total_gardens + 1;
        let odd_garden = extra_garden * extra_garden;
        let even_garden = total_gardens * total_gardens;
        let all_odds = odd * odd_garden;
        let all_evens = even * even_garden;
        let odd_corners = extra_garden * corners_odd;
        let even_corners = total_gardens * corners_even;
        let ans = all_odds + all_evens + even_corners - odd_corners;
        println!("{:?}", ans);
        assert_eq!(621289922886149, ans);

        // for steps in 0..garden.size.0 as u8 +10 {
        //     let walk = garden.possible_tiles(steps);
        //     let even = walk.values()
        //         .filter(|distance| **distance % 2 == 0)
        //         .count();
        //     let odd = walk.values()
        //         .filter(|distance| **distance % 2 == 1)
        //         .count();
        //     println!("Steps: {:?}: odd {:?}, even {:?}", steps, odd, even);
        //     // println!("len:  {:?}", walk.len());
        //     // println!("even: {:?}", even);
        //     // println!("odd:  {:?}", odd);
        //     // println!("=====================================================");
        // }
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day21/test-input.txt";
        assert_eq!(16, solve_b(input, 6));
        assert_eq!(50, solve_b(input, 10));
        // assert_eq!(1594, solve_b(input, 50));
        // assert_eq!(6536, solve_b(input, 100));
        // assert_eq!(16733044, solve_b(input, 5000));
    }
}