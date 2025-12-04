use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::util::lines_from_file;

pub fn day15() {
    println!("== Day 15 ==");
    let input = lines_from_file("src/day15/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Node {
    row: usize,
    col: usize,
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
struct Edge {
    node: Node,
    cost: u32,
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

fn part_a(input: &Vec<String>) -> u32 {
    let cavern: Vec<Vec<u32>> = input.iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|ca| ca.iter().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect();
    let edges = to_edges(&cavern, 1);
    let option = shortest_path(&edges, Node { row: 0, col: 0 }, Node { row: cavern.len() - 1, col: cavern.iter().next().unwrap().len() - 1 });
    option.unwrap()
}

fn part_b(input: &Vec<String>) -> u32 {
    let cavern: Vec<Vec<u32>> = input.iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|ca| ca.iter().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect();
    let edges = to_edges(&cavern, 5);
    let option = shortest_path(&edges, Node { row: 0, col: 0 }, Node { row: cavern.len() * 5 - 1, col: cavern.iter().next().unwrap().len() * 5 - 1 });
    option.unwrap()
}


fn to_edges(cavern: &Vec<Vec<u32>>, num_tile: u32) -> HashMap<Node, Vec<Edge>> {
    let rd = [-1, 0, 1, 0];
    let cd = [0, -1, 0, 1];
    let mut edges: HashMap<Node, Vec<Edge>> = HashMap::new();
    let orig_max_rows = cavern.len();
    let orig_max_cols = cavern.iter().next().unwrap().len();
    let max_rows = orig_max_rows * num_tile as usize;
    let max_cols = orig_max_cols * num_tile as usize;
    for r in 0..max_rows {
        for c in 0..max_cols {
            let this_node = Node { row: r, col: c };
            let mut adjacent: Vec<Edge> = Vec::new();
            for i in 0..rd.len() {
                let rr = r as i32 + rd[i];
                let cc = c as i32 + cd[i];
                if rr >= 0 && rr < max_rows as i32 && cc >= 0 && cc < max_cols as i32 {
                    let other_node = Node { row: rr as usize, col: cc as usize };
                    let rrr = rr as usize % orig_max_rows;
                    let ccc = cc as usize % orig_max_cols;
                    let orig_cost = cavern.get(rrr).unwrap().get(ccc).unwrap();
                    let add_for_row = (rr as usize / orig_max_rows) as u32;
                    let add_for_col = (cc as usize / orig_max_cols) as u32;
                    let mut cost = *orig_cost + add_for_row + add_for_col;
                    while cost > 9 {
                        cost -= 9;
                    }
                    adjacent.push(Edge { node: other_node, cost });
                }
            }
            edges.insert(this_node, adjacent);
        }
    }
    edges
}


fn shortest_path(edges: &HashMap<Node, Vec<Edge>>, start: Node, end: Node) -> Option<u32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day15/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(40, result)
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day15/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(790, result)
    }

    #[test]
    fn to_edges_t_1() {
        let input = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        let expected: HashMap<Node, Vec<Edge>> = HashMap::from([
            (Node { row: 0, col: 0 }, vec![
                Edge { node: Node { row: 0, col: 1 }, cost: 2 },
                Edge { node: Node { row: 1, col: 0 }, cost: 3 }]
            ),
            (Node { row: 0, col: 1 }, vec![
                Edge { node: Node { row: 0, col: 0 }, cost: 1 },
                Edge { node: Node { row: 1, col: 1 }, cost: 4 }]
            ),
            (Node { row: 1, col: 0 }, vec![
                Edge { node: Node { row: 0, col: 0 }, cost: 1 },
                Edge { node: Node { row: 1, col: 1 }, cost: 4 }]
            ),
            (Node { row: 1, col: 1 }, vec![
                Edge { node: Node { row: 0, col: 1 }, cost: 2 },
                Edge { node: Node { row: 1, col: 0 }, cost: 3 }]
            )
        ]);
        let result = to_edges(&input, 1);

        println!("{:?}", result);

        for (k, v) in expected.iter() {
            let rv = result.get(k).unwrap();
            println!("{:?}", v);
            println!("{:?}", rv);
            assert!(rv.iter().all(|item| v.contains(item)));
            assert!(v.iter().all(|item| rv.contains(item)));
        }
    }

    #[test]
    fn to_edges_t_2() {
        let input = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        let input_like = vec![
            vec![1, 2, 2, 3],
            vec![3, 4, 4, 5],
            vec![2, 3, 3, 4],
            vec![4, 5, 5, 6],
        ];

        let result = to_edges(&input, 2);
        let result_like = to_edges(&input_like, 1);

        for (k, v) in result_like.iter() {
            let rv = result.get(k).unwrap();
            assert!(rv.iter().all(|item| v.contains(item)));
            assert!(v.iter().all(|item| rv.contains(item)));
        }
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day15/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(315, result)
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day15/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(2998, result)
    }
}
