use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Range, RangeInclusive};

use crate::util::time;

pub fn day22() {
    println!("== Day 22 ==");
    let input = "src/day22/input.txt";
    // time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let mut tower = Tower::parse(input);
    // println!("{:?}", tower);
    // tower._print();
    tower.settle(false);
    // tower._print();
    tower.count_desintigrateables()
}


fn part_b(input: &str) -> usize {
    let mut tower = Tower::parse(input);
    tower.settle(false);
    tower.desintigrate()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Size {
    min: usize,
    max: usize,
}

impl Size {
    fn range(&self) -> Range<usize> {
        self.min..self.max
    }
    fn range_inc(&self) -> RangeInclusive<usize> {
        self.min..=self.max
    }
}

#[derive(Debug, Copy, Clone)]
struct Brick {
    id: usize,
    size: [Size; 3],
    pos: [Size; 3],
}

impl Brick {
    fn parse(s: &str) -> Self {
        let (min, max) = s.split_once("~").unwrap();
        let min_split = min.split(",").collect::<Vec<&str>>();
        let mins = min_split.iter().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let maxs = max.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let maybe_id = min_split.iter().rev().map(|s| *s).collect::<String>();
        let id = maybe_id.parse::<usize>().unwrap();
        // println!("{:?} -> {:?} :: {:?}", min, maybe_id, id);

        let size = [
            Size { min: mins[0], max: maxs[0] }, // X
            Size { min: mins[1], max: maxs[1] }, // Y
            Size { min: mins[2], max: maxs[2] }, // Z
        ];

        Self {
            id,
            size,
            pos: size,
        }
    }

    fn lower_to(&self, z: usize) -> Self {
        let z_pos = self.pos[2];
        let diff = z_pos.min - z;
        let new_z = Size { min: z_pos.min - diff, max: z_pos.max - diff };
        Self {
            id: self.id,
            size: self.size,
            pos: [self.pos[0], self.pos[1], new_z],
        }
    }
}

#[derive(Debug)]
struct Tower {
    bricks: HashMap<usize, Brick>,
    layout: Vec<Vec<Vec<usize>>>,
}

impl Tower {
    fn parse(input: &str) -> Self {
        let bricks = File::open(input)
            .map(|f| BufReader::new(f).lines().flatten()
                .map(|l| Brick::parse(l.as_str()))
                .collect::<Vec<Brick>>()
            ).unwrap();

        let max_x = bricks.iter().map(|b| b.pos[0].max).max().unwrap_or(0);
        let max_y = bricks.iter().map(|b| b.pos[1].max).max().unwrap_or(0);
        let max_z = bricks.iter().map(|b| b.pos[2].max).max().unwrap_or(0);

        let mut layout = vec![vec![vec![0; max_x + 1]; max_y + 1]; max_z + 1];
        // println!("[{:?}][{:?}][{:?}]", max_x + 1, max_y + 1, max_z + 1);
        // for b in bricks.iter(){
        //     println!("{:?}", b);
        // }
        for brick in bricks.iter() {
            for z in brick.size[2].range_inc() {
                for y in brick.size[1].range_inc() {
                    for x in brick.size[0].range_inc() {
                        // println!("[{:?}][{:?}][{:?}] = {:?}", z,y,x, brick.id);
                        layout[z][y][x] = brick.id;
                    }
                }
            }
        }

        Self {
            bricks: HashMap::from_iter(bricks.into_iter().map(|c| (c.id, c))),
            layout,
        }
    }

    fn update_layout(&mut self) {
        let max_x = self.bricks.iter().map(|(_, b)| b.pos[0].max).max().unwrap_or(0);
        let max_y = self.bricks.iter().map(|(_, b)| b.pos[1].max).max().unwrap_or(0);
        let max_z = self.bricks.iter().map(|(_, b)| b.pos[2].max).max().unwrap_or(0);

        let mut layout = vec![vec![vec![0; max_x + 1]; max_y + 1]; max_z + 1];
        // println!("update: [{:?}][{:?}][{:?}]", max_x + 1, max_y + 1, max_z + 1);
        for brick in self.bricks.values() {
            for z in brick.pos[2].range_inc() {
                for y in brick.pos[1].range_inc() {
                    for x in brick.pos[0].range_inc() {
                        layout[z][y][x] = brick.id;
                    }
                }
            }
        }
        self.layout = layout;
    }

    fn settle(&mut self, quick_break: bool) -> usize {
        let mut moved = 0;
        let mut keys = self.bricks.keys().map(|k| *k).collect::<Vec<usize>>();
        keys.sort();
        // let mut keys = Vec::new();
        // for yx in self.layout.iter(){
        //     for x in yx.iter() {
        //         for id in x.iter(){
        //             if !keys.contains(id) && *id!=0{
        //                 keys.push(*id);
        //             }
        //         }
        //     }
        // }

        // println!("{:?}", keys);
        for key in keys {
            if !self.bricks.contains_key(&key) {
                panic!("Found key {:?} but not found in bricks", key);
            }
            let brick = self.bricks.get(&key).unwrap();
            let mut highest_brick_under = None;
            for x in brick.pos[0].range_inc() {
                for y in brick.pos[1].range_inc() {
                    // if brick.id == 2 {
                    //     println!("{:?} || {:?}", brick.id, brick.pos[2]);
                    // }
                    for z in (1..brick.pos[2].min).rev() {
                        // if brick.id == 2 {
                        //     println!("Checking [{:?}][{:?}][{:?}] == {:?}", z, y, x, self.layout[z][y][x]);
                        // }
                        if self.layout[z][y][x] != 0 {
                            highest_brick_under = if highest_brick_under.is_none() { Some(z) } else { highest_brick_under.map(|v| max(v, z)) };
                        }
                    }
                }
            }
            // println!("Can move {:?} to {:?}", brick.id, highest_brick_under.unwrap_or(0)+1);
            let updated_brick = brick.lower_to(highest_brick_under.unwrap_or(0) + 1);
            if updated_brick.pos != brick.pos {
                // println!("{:?} was moved {:?} -> {:?}", brick.id, brick.pos, updated_brick.pos);
                moved += 1;
                if quick_break {
                    return moved;
                }
            }
            self.bricks.insert(updated_brick.id, updated_brick);
            self.update_layout();
        }

        // // This _has_ to be done better .... what even is this...
        // for z in 1..self.layout.len() {
        //     // println!("Z {:?}: {:?}", z, self.layout[z]);
        //     if z == 1 { // 1 is the floor, can't move down
        //         continue;
        //     }
        //     for y in 0..self.layout[0].len() {
        //         for x in 0..self.layout[0][0].len() {
        //             let check = self.layout[z][y][x];
        //             if check != 0 { // there is a brick here
        //                 let brick = self.bricks.get(&check).unwrap();
        //                 let relevant_x_values = brick.x;
        //                 let relevant_y_values = brick.y;
        //                 let mut lowest_free_z = 0;
        //                 for my in relevant_y_values.min..=relevant_y_values.max {
        //                     for mx in relevant_x_values.min..=relevant_x_values.max {
        //                         for mz in 1..z - 1 {
        //                             if self.layout[mz][my][mx] != 0 {
        //                                 lowest_free_z = lowest_free_z.max(mz + 1);
        //                             }
        //                         }
        //                     }
        //                 }
        //
        //                 let z_diff = if lowest_free_z > 0 { brick.z.min - lowest_free_z } else { 0 };
        //                 // println!("Lowest free z {:?}, diff {:?}", lowest_free_z,z_diff);
        //                 if z_diff > 0 {
        //                     for mz in brick.z.min..=brick.z.max {
        //                         for my in brick.y.min..=brick.y.max {
        //                             for mx in brick.x.min..=brick.x.max {
        //                                 self.layout[mz][my][mx] = 0;
        //                             }
        //                         }
        //                     }
        //
        //                     let new_z = (brick.z.min - z_diff, brick.z.max - z_diff);
        //                     for mz in new_z.0..=new_z.1 {
        //                         for my in relevant_y_values.min..=relevant_y_values.max {
        //                             for mx in relevant_x_values.min..=relevant_x_values.max {
        //                                 self.layout[mz][my][mx] = brick.id;
        //                                 moved += 1;
        //                             }
        //                         }
        //                     }
        //                 }
        //                 // println!("Moving brick {:?}", brick.id);
        //                 // self._print();
        //                 continue;
        //             }
        //         }
        //     }
        // }
        // println!("{:?}", moved);
        moved
    }
    fn _print(&self) {
        println!("=====================================");
        for z in 1..self.layout.len() {
            println!("Z {:?}: {:?}", z, self.layout[z]);
        }
    }

    fn count_desintigrateables(&mut self) -> usize {
        let original_layout = self.layout.clone();
        let original_bricks = self.bricks.clone();
        let mut key: Vec<usize> = self.bricks.keys().map(|k| *k).collect();
        key.sort();
        let mut possible = 0;
        for k in key {
            // println!("Remove brick {:?} and settle", k);
            self.remove_brick(k);
            let moved = self.settle(true);
            if moved == 0 {
                // println!("{:?} can be removed!",k);
                possible += 1;
            } else {
                // println!("{:?} can not be removed",k);
            }
            // self._print();
            // println!("Resetting after testing {:?}, moved {:?}", k, moved);
            self.layout = original_layout.clone();
            self.bricks = original_bricks.clone();
        }
        possible
    }

    fn desintigrate(&mut self) -> usize {
        let original_layout = self.layout.clone();
        let original_bricks = self.bricks.clone();
        let mut key: Vec<usize> = self.bricks.keys().map(|k| *k).collect();
        key.sort();
        let mut sum_moved = 0;
        for k in key {
            self.remove_brick(k);
            let moved = self.settle(false);
            sum_moved += moved;
            self.layout = original_layout.clone();
            self.bricks = original_bricks.clone();
        }
        sum_moved
    }

    fn remove_brick(&mut self, id: usize) {
        self.bricks.remove(&id);
        self.update_layout();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day22();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day22/input.txt";
        assert_eq!(416, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day22/input.txt";
        assert_eq!(60963, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day22/test-input.txt";
        let result = part_a(input);
        assert_eq!(5, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day22/test-input.txt";
        let result = part_b(input);
        assert_eq!(7, result);
    }
}