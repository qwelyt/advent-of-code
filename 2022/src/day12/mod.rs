use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day12() {
    println!("== Day 12 ==");
    let input = "src/day12/input.txt";
    time(part_a, input, "A");
    time(part_b_2, input, "B");
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Node {
    row: usize,
    col: usize,
    height: char,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Edge {
    node: Node,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.row.cmp(&other.row)
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    position: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip to make min-heap
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    // Same as for ord
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_a(input: &str) -> u32 {
    let height_map = height_map(input);
    let edges = to_edges(&height_map);

    // println!("{:?}", height_map);
    // println!("{:?}", edges);
    // for e in edges.iter() {
    //     println!("{:?} :: {}", e.0, e.1.len());
    //     for edge in e.1.iter() {
    //         println!("   {}", edge.node.height);
    //     }
    // }
    let start = edges.iter().find(|n| n.0.height == 'S').map(|n| n.0).unwrap();
    let end = edges.iter().find(|n| n.0.height == 'E').map(|n| n.0).unwrap();
    let cost = djikstra(&edges, *start, *end);
    cost.unwrap_or(0)
}

fn part_b(input: &str) -> u32 {
    let height_map = height_map(input);
    let edges = to_edges(&height_map);
    let end = edges.iter().find(|n| n.0.height == 'E').map(|n| n.0).unwrap();
    let starts = edges.iter().filter(|n| n.0.height == 'S' || n.0.height == 'a').map(|n| n.0).collect::<Vec<&Node>>();
    let mut costs = Vec::new();
    for start in starts.iter() {
        let cost = djikstra(&edges, **start, *end);
        costs.push(cost);
    }
    // println!("Starts: {} -> Valid paths: {}", starts.len(), costs.len());
    costs.iter().map(|c| c.unwrap_or(u32::MAX)).min().unwrap_or(u32::MAX)
}

fn part_b_2(input: &str) -> u32 {
    let height_map = height_map(input);
    let edges = to_edges(&height_map);
    let end = edges.iter().find(|n| n.0.height == 'E').map(|n| n.0).unwrap();
    let starts = edges.iter().filter(|n| n.0.height == 'S' || n.0.height == 'a').map(|n| n.0).collect::<Vec<&Node>>();
    // println!("Starts: {}", starts.len());
    let mut cost = u32::MAX;
    for start in starts.iter() {
        let c = djikstra_shortcircuit(&edges, **start, *end, cost);
        if c.is_some() && c.unwrap() < cost {
            cost = c.unwrap();
        }
    }
    cost
}


fn height_map(input: &str) -> Vec<Vec<char>> {
    let mut height_map: Vec<Vec<char>> = Vec::new();
    let open = File::open(input).expect("Could not read file");
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let chars = line.chars().collect::<Vec<char>>();
        height_map.push(chars);
    }
    height_map
}

fn to_edges(map: &Vec<Vec<char>>) -> HashMap<Node, Vec<Edge>> {
    let rd = [-1, 0, 1, 0];
    let cd = [0, -1, 0, 1];
    let mut edges: HashMap<Node, Vec<Edge>> = HashMap::new();
    let max_rows = map.len();
    let max_cols = map.iter().next().unwrap().len();
    for r in 0..max_rows {
        for c in 0..max_cols {
            let this_height = map.get(r).unwrap().get(c).unwrap();
            let this_node = Node { row: r, col: c, height: *this_height };
            let mut adjacent: Vec<Edge> = Vec::new();
            for i in 0..rd.len() {
                let rr = r as i32 + rd[i];
                let cc = c as i32 + cd[i];

                if rr >= 0 && rr < max_rows as i32
                    && cc >= 0 && cc < max_cols as i32 {
                    let other_height = map.get(rr as usize).unwrap().get(cc as usize).unwrap();
                    let other_node = Node { row: rr as usize, col: cc as usize, height: *other_height };
                    if *other_height != 'E'
                        && ((((*other_height as u32) as i32 - (*this_height as u32) as i32) < 2)
                        || *this_height == 'S'
                        || *other_height == 'S') {
                        adjacent.push(Edge { node: other_node, cost: 1 })
                    } else if *this_height == 'z' && *other_height == 'E' {
                        adjacent.push(Edge { node: other_node, cost: 1 })
                    }
                }
            }
            edges.insert(this_node, adjacent);
        }
    }
    edges
}


fn djikstra(edges: &HashMap<Node, Vec<Edge>>, start: Node, end: Node) -> Option<u32> {
    let mut dist: HashMap<Node, u32> = HashMap::from_iter(edges.iter().map(|(k, _v)| (*k, u32::MAX)));
    let mut heap = BinaryHeap::new();
    *dist.entry(start).or_default() = 0;
    heap.push(State { cost: 0, position: start });
    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }
        if cost > *dist.get(&position).unwrap() {
            continue;
        }
        for edge in edges.get(&position).unwrap() {
            let next = State { cost: cost + edge.cost, position: edge.node };
            if next.cost < *dist.get(&next.position).unwrap() {
                heap.push(next);
                *dist.entry(next.position).or_default() = next.cost;
            }
        }
    }
    None
}

fn djikstra_shortcircuit(edges: &HashMap<Node, Vec<Edge>>, start: Node, end: Node, cost_limit: u32) -> Option<u32> {
    let mut dist: HashMap<Node, u32> = HashMap::from_iter(edges.iter().map(|(k, _v)| (*k, u32::MAX)));
    let mut heap = BinaryHeap::new();
    *dist.entry(start).or_default() = 0;
    heap.push(State { cost: 0, position: start });
    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }
        if cost > *dist.get(&position).unwrap() {
            continue;
        }
        for edge in edges.get(&position).unwrap() {
            let next = State { cost: cost + edge.cost, position: edge.node };
            if next.cost > cost_limit {
                return None;
            }
            if next.cost < *dist.get(&next.position).unwrap() {
                heap.push(next);
                *dist.entry(next.position).or_default() = next.cost;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day12();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day12/input.txt";
        assert_eq!(456, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day12/input.txt";
        assert_eq!(454, part_b(input));
    }

    #[ignore]
    #[test]
    fn real_b2() {
        let input = "src/day12/input.txt";
        assert_eq!(454, part_b_2(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day12/test-input.txt";
        let result = part_a(input);
        assert_eq!(31, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day12/test-input.txt";
        let result = part_b(input);
        assert_eq!(29, result);
    }

    #[test]
    fn part_b2_test_input() {
        let input = "src/day12/test-input.txt";
        let result = part_b_2(input);
        assert_eq!(29, result);
    }
}