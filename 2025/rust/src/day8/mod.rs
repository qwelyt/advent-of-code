use crate::util::time;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 8 ==");
    let input = "src/day8/input.txt";
    time(part_a_real, input, "A");
    time(part_b, input, "B");
}

fn part_a_real(input: &str) -> usize {
    part_a(input, 1000)
}

fn part_a(input: &str, connections: usize) -> usize {
    let (coords, edges) = coords_and_edges(input);
    let mut coord_group: HashMap<Coord, usize> = HashMap::new();
    let mut group_number = 0;
    for edge in edges.iter().take(connections) {
        let ag = coord_group.get(&edge.a);
        let bg = coord_group.get(&edge.b);
        if ag.is_none() && bg.is_none() {
            coord_group.insert(edge.a, group_number);
            coord_group.insert(edge.b, group_number);
            group_number += 1;
        } else if ag.is_some() && bg.is_none() {
            let g = *ag.unwrap();
            coord_group.insert(edge.b, g);
        } else if ag.is_none() && bg.is_some() {
            let g = *bg.unwrap();
            coord_group.insert(edge.a, g);
        } else if ag.is_some() && bg.is_some() {
            // Merge the two groups into one
            let a_group = *ag.unwrap();
            let b_group = *bg.unwrap();
            for (_, v) in coord_group.iter_mut() {
                if *v == b_group {
                    *v = a_group;
                }
            }
        }
    }
    let mut groups: HashMap<usize, Vec<Coord>> = HashMap::new();
    for (k, v) in coord_group.iter() {
        if !groups.contains_key(v) {
            groups.insert(*v, Vec::new());
        }
        groups.get_mut(v).unwrap().push(k.clone());
    }

    let mut grps = groups
        .iter()
        .map(|e| (e.1.len(), e.1))
        .collect::<Vec<(usize, &Vec<Coord>)>>();
    grps.sort_by(|a, b| usize::cmp(&a.0, &b.0));

    grps.iter().rev().take(3).map(|e| e.0).product()
}

fn part_b(input: &str) -> isize {
    let (coords, edges) = coords_and_edges(input);
    let mut coord_group: HashMap<Coord, usize> = HashMap::new();
    let mut group_number = 0;
    for edge in edges.iter() {
        let ag = coord_group.get(&edge.a);
        let bg = coord_group.get(&edge.b);
        if ag.is_none() && bg.is_none() {
            coord_group.insert(edge.a, group_number);
            coord_group.insert(edge.b, group_number);
            group_number += 1;
        } else if ag.is_some() && bg.is_none() {
            let g = *ag.unwrap();
            coord_group.insert(edge.b, g);
        } else if ag.is_none() && bg.is_some() {
            let g = *bg.unwrap();
            coord_group.insert(edge.a, g);
        } else if ag.is_some() && bg.is_some() {
            // Merge the two groups into one
            let a_group = *ag.unwrap();
            let b_group = *bg.unwrap();
            for (_, v) in coord_group.iter_mut() {
                if *v == b_group {
                    *v = a_group;
                }
            }
        }
        if coord_group.len() == coords.len() {
            let mut groups: HashMap<usize, Vec<Coord>> = HashMap::new();
            for (k, v) in coord_group.iter() {
                if !groups.contains_key(v) {
                    groups.insert(*v, Vec::new());
                }
                groups.get_mut(v).unwrap().push(k.clone());
            }
            if groups.len() == 1 {
                return edge.a.x * edge.b.x;
            }
        }
    }
    0
}

fn coords_and_edges(input: &str) -> (Vec<Coord>, Vec<Edge>) {
    let coords = File::open(input)
        .map(BufReader::new)
        .map(|reader| {
            reader
                .lines()
                .flatten()
                .map(Coord::parse)
                .collect::<Vec<Coord>>()
        })
        .expect("Should have input");

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..coords.len() {
        for ii in i + 1..coords.len() {
            edges.push(Edge::calculate(coords[i], coords[ii]))
        }
    }
    edges.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    (coords, edges)
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn parse(str: String) -> Coord {
        let v = str
            .split(",")
            .map(|s| s.parse::<isize>())
            .flatten()
            .collect::<Vec<isize>>();
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
    fn distance(&self, other: Coord) -> f64 {
        let dx = (other.x - self.x).pow(2);
        let dy = (other.y - self.y).pow(2);
        let dz = (other.z - self.z).pow(2);
        ((dx + dy + dz) as f64).sqrt()
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)?;
        Ok(())
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct Edge {
    distance: f64,
    a: Coord,
    b: Coord,
}
impl Edge {
    fn calculate(a: Coord, b: Coord) -> Edge {
        Edge {
            distance: a.distance(b),
            a,
            b,
        }
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
        let input = "src/day8/input.txt";
        assert_eq!(103488, part_a_real(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day8/input.txt";
        assert_eq!(8759985540, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day8/test-input.txt";
        let result = part_a(input, 10);
        assert_eq!(40, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day8/test-input.txt";
        let result = part_b(input);
        assert_eq!(25272, result);
    }
}
