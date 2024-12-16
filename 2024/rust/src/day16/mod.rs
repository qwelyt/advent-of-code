use crate::util::time;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("== Day 16 ==");
    let input = "src/day16/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> u32 {
    let (map, start, end) = parse_input(input);

    // println!("{:?} UP", next_move(&map, (15,3), (-1,0)));
    // println!("{:?} RIGHT", next_move(&map, (15,3), (0,1)));
    // println!("{:?} DOWN", next_move(&map, (15,3), (1,0)));
    // println!("{:?} LEFT", next_move(&map, (15,3), (0,-1)));

    // println!("{:?} - {:?}", start, map[start.0][start.1]);
    // println!("{:?} - {:?}", end, map[end.0][end.1]);
    // for l in map.iter(){
    //     println!("{:?}", l);
    // }
    //
    dijkstra(&map, start, end).unwrap()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let map = File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .unwrap();
    // Assumptions
    let start = (map.len() - 2, 1);
    let end = (1, map[0].len() - 2);
    (map, start, end)
}

fn part_b(input: &str) -> usize {
    let (map, start, end) = parse_input(input);
    dijkstra_all_paths(&map, start, end).unwrap().len()
}

fn dijkstra(map: &Vec<Vec<char>>, start: Coord, end: Coord) -> Option<u32> {
    let mut dist: HashMap<Key, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let up = State {
        cost: 1000,
        pos: start,
        dir: (-1, 0),
        steps: 0,
    };
    let left = State {
        cost: 0,
        pos: start,
        dir: (0, 1),
        steps: 0,
    };
    heap.push(up);
    heap.push(left);
    dist.insert(up.into(), up.cost);
    dist.insert(left.into(), left.cost);
    while let Some(current) = heap.pop() {
        if current.pos == end {
            return Some(current.cost);
        }
        if *dist.get(&current.into()).unwrap_or(&u32::MAX) < current.cost {
            continue;
        }

        for (dir, pos, cost) in next_move(&map, current.pos, current.dir) {
            let next = State {
                cost: current.cost + cost,
                pos,
                dir,
                steps: current.steps + 1,
            };

            if *dist.get(&next.into()).unwrap_or(&u32::MAX) <= next.cost {
                continue;
            }

            heap.push(next);
            *dist.entry(next.into()).or_default() = next.cost;
        }
    }
    None
}
fn dijkstra_all_paths(
    map: &Vec<Vec<char>>,
    start: Coord,
    end: Coord,
) -> Option<HashSet<Coord>> {
    let mut dist: HashMap<Key, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let up = State::of(1000, start, (-1, 0), 0);
    let left = State::of(0, start, (0, 1), 0);
    heap.push(up);
    heap.push(left);
    dist.insert(up.into(), up.cost);
    dist.insert(left.into(), left.cost);

    let mut backtrack = HashMap::new();
    let mut best_cost = u32::MAX;
    let mut end_state = HashSet::new();

    while let Some(current) = heap.pop() {
        if current.pos == end {
            if current.cost > best_cost {
                break;
            }
            best_cost = current.cost;
            end_state.insert((current.pos, current.dir));
        }
        for (dir, pos, cost) in next_move(&map, current.pos, current.dir) {
            let next = State::of(current.cost + cost, pos, dir, current.steps+1);
            let lowest = *dist.get(&next.into()).unwrap_or(&u32::MAX);
            if lowest < next.cost {
                continue;
            }
            if lowest > next.cost {
                backtrack.insert((next.pos, next.dir), HashSet::new());
                *dist.entry(next.into()).or_default() = next.cost;
            }
            backtrack.entry((next.pos, next.dir)).or_default().insert((current.pos,current.dir));
            heap.push(next);
        }
    }

    // println!("{:?}", backtrack);
    // println!("{:?}", end_state);

    let mut q = VecDeque::new();
    let mut seen = end_state.clone();
    for s in seen.iter() {
        q.push_back(*s);
        // println!("BT {:?}", backtrack.get(s));
    }

    while let Some(next) = q.pop_front() {
        for last in backtrack.get(&next).unwrap_or(&HashSet::new()) {
            if seen.contains(last) {continue}
            seen.insert(*last);
            q.push_back(*last);
        }
    }
    // println!("{:?}", seen);
    // println!("{:?}", seen.len());
    let set = seen.iter().map(|(coord, _dir)| *coord).collect::<HashSet<Coord>>();
    // println!("{}, {:?}", set.len(), set);

    Some(set)
}

fn next_move(map: &Vec<Vec<char>>, pos: Coord, dir: Dir) -> Vec<(Dir, Coord, u32)> {
    let mut moves = Vec::new();
    let straight = new_pos(pos, dir);
    // println!("S: {:?} - {:?}", straight, dir);
    if map[straight.0][straight.1] != '#' {
        moves.push((dir, straight, 1));
    }
    let turn_right = (dir.1, dir.0 * -1);
    let right = new_pos(pos, turn_right);
    // println!("R: {:?} - {:?}", right, turn_right);
    if map[right.0][right.1] != '#' {
        moves.push((turn_right, right, 1000 + 1));
    }

    let turn_left = (dir.1 * -1, dir.0);
    let left = new_pos(pos, turn_left);
    // println!("L: {:?} - {:?}", left, turn_left);
    if map[left.0][left.1] != '#' {
        moves.push((turn_left, left, 1000 + 1));
    }

    moves
}
fn new_pos(pos: Coord, dir: Dir) -> Coord {
    (
        (pos.0 as isize + dir.0 as isize) as usize,
        (pos.1 as isize + dir.1 as isize) as usize,
    )
}
type Coord = (usize, usize);
type Dir = (i8, i8);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    pos: Coord,
    dir: Dir,
    steps: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl State {
    fn of(cost: u32, pos: Coord, dir: Dir, steps: u32) -> Self {
        Self {
            cost,
            pos,
            dir,
            steps,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Key {
    pos: Coord,
    dir: Dir,
}

impl From<State> for Key {
    fn from(state: State) -> Self {
        Self {
            pos: state.pos,
            dir: state.dir,
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
        let input = "src/day16/input.txt";
        assert_eq!(105496, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day16/input.txt";
        assert_eq!(524, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day16/test-input.txt";
        let result = part_a(input);
        assert_eq!(11048, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day16/test-input.txt";
        let result = part_b(input);
        assert_eq!(64, result);
    }
}
