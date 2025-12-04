use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use crate::util::time;

pub fn solve() {
    println!("== Day 20 ==");
    let input = "src/day20/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    p1_smarter(input).iter()
        .filter(|(k,_)| **k > 101)
        .map(|(_,v)| v)
        .sum()
}

fn part_b(input: &str) -> usize {
    p2(input).iter()
        .filter(|((cost,radius),_)| *cost >= 100+*radius )
        .map(|((_,_),v)| v)
        .sum()
}

fn p1(input: &str) -> HashMap<usize,usize> {
    let (start,end,map,walls) = parse_map(input);
    // println!("Start: {:?}, end: {:?}",start,end);
    // for l in map.iter() {
    //     println!("{:?}", l);
    // }
    // println!();
    let no_cheats = dijkstra(&start.unwrap(), &end.unwrap(), &map, usize::MAX).expect("No path found");
    // println!("{:?}", no_cheats);
    let mut m: HashMap<usize,usize> = HashMap::new();
    let paths = dijkstra_with_cheats(&start.unwrap(), &end.unwrap(), &map, &walls, no_cheats);
    for p in paths.iter() {
        if *p < no_cheats {
            let key = no_cheats-*p;
            m.entry(key).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    m
}
fn p1_smarter(input: &str) -> HashMap<usize,usize> {
    let (start,end,map,_walls) = parse_map(input);
    let mut pos = start.unwrap().clone();
    let mut costs = vec![vec![-1; map[0].len()]; map.len()];
    costs[pos.uy()][pos.ux()] = 0;
    // println!("Start: {:?}, {:?}", costs[pos.uy()][pos.ux()], pos);
    // for c in costs.iter(){
    //     println!("{:?}", c);
    // }
    // println!();

    while pos != end.unwrap() {
        for npos in [pos+(-1, 0), pos+(1, 0),pos+(0, 1), pos+(0, -1)] {
            if npos.y < 0
                || npos.x < 0
                || npos.uy() >= map.len()
                || npos.ux() >= map[0].len()
            { continue; }
            if map[npos.uy()][npos.ux()] == '#'{ continue; }
            if costs[npos.uy()][npos.ux()] != -1 { continue; }
            costs[npos.uy()][npos.ux()] = costs[pos.uy()][pos.ux()] + 1;
            pos = npos;
        }
    }
    let mut m: HashMap<usize,usize> = HashMap::new();
    for y in 0..map.len(){
        for x in 0..map[y].len(){
            if map[y][x] == '#'{ continue; }
            let pos = Pos::new(x, y);
            //
            //  S-
            // /|\
            //
            for npos in [pos+(0,2),pos+(1,1), pos+(2,0), pos+(1,-1)] {
                if npos.y < 0
                    || npos.x < 0
                    || npos.uy() >= map.len()
                    || npos.ux() >= map[0].len()
                { continue; }
                if map[npos.uy()][npos.ux()] == '#'{ continue; }
                let i = (costs[y][x] - costs[npos.uy()][npos.ux()]) as isize;
                let cost = (i).abs() as usize;
                // println!("i: {}, cost: {}", i, cost);
                m.entry(cost).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }
    m
}
fn p2(input: &str) -> HashMap<(isize,isize),usize> {
    let (start,end,map,_walls) = parse_map(input);
    let mut pos = start.unwrap().clone();
    let mut costs = vec![vec![-1_isize; map[0].len()]; map.len()];
    costs[pos.uy()][pos.ux()] = 0;

    while pos != end.unwrap() {
        for npos in [pos+(-1, 0), pos+(1, 0),pos+(0, 1), pos+(0, -1)] {
            if npos.y < 0
                || npos.x < 0
                || npos.uy() >= map.len()
                || npos.ux() >= map[0].len()
            { continue; }
            if map[npos.uy()][npos.ux()] == '#'{ continue; }
            if costs[npos.uy()][npos.ux()] != -1 { continue; }
            costs[npos.uy()][npos.ux()] = costs[pos.uy()][pos.ux()] + 1;
            pos = npos;
        }
    }
    let mut m: HashMap<(isize,isize),usize> = HashMap::new();
    for y in 0..map.len(){
        for x in 0..map[y].len(){
            if map[y][x] == '#'{ continue; }
            let iy = y as isize;
            let ix = x as isize;
            for radius in 2..=20_isize {
                for ry in 0..=radius {
                    let rx = radius-ry;
                    let new_pos = HashSet::from([
                        (iy+ ry, ix+ rx),
                        (iy+ ry, ix- rx),
                        (iy- ry, ix+ rx),
                        (iy- ry, ix- rx),
                    ]);
                    for (ny,nx) in new_pos {
                        if ny < 0 || nx < 0
                            || ny as usize >= map.len() || nx as usize >= map[0].len()
                        { continue; }
                        if map[ny as usize][nx as usize] == '#'{ continue; }

                        let cost = costs[y][x] - costs[ny as usize][nx as usize];
                        // println!("i: {}, cost: {}", i, cost);
                        m.entry((cost,radius)).and_modify(|v| *v += 1).or_insert(1);
                    }

                }
            }
        }
    }
    m
}
fn parse_map(input: &str) -> (Option<Pos>, Option<Pos>, Vec<Vec<char>>, HashSet<Pos>) {
    let mut start = None;
    let mut end = None;
    let mut walls = HashSet::new();
    let map = File::open(input)
        .map(|f| {
            BufReader::new(f)
                .lines()
                .flatten()
                .enumerate()
                .map(|(p, s)| {
                    let vec = s.chars().collect::<Vec<char>>();
                    for (i,c) in vec.iter().enumerate() {
                        match c {
                            '#' => { walls.insert(Pos::new(i,p)); }
                            'S' => { start = Some(Pos::new(i,p)); }
                            'E' => { end = Some(Pos::new(i,p)); }
                            &_ => {}
                        }
                    }
                    vec
                })
                .collect::<Vec<Vec<char>>>()
        })
        .unwrap();
    (start, end, map, walls)
}

fn dijkstra(start: &Pos, end: &Pos, map: &Vec<Vec<char>>, max_cost: usize) -> Option<usize> {
    let mut dist: HashMap<Pos, usize> = HashMap::new();
    let mut heap : BinaryHeap<State>= BinaryHeap::new();
    heap.push(State::of(*start, 0));
    dist.insert(*start, 0);
    while let Some(current) = heap.pop() {
        if current.cost >= max_cost {
            return None;
        }
        if current.pos == *end {
            return Some(current.cost)
        }
        if *dist.get(&current.pos).unwrap_or(&usize::MAX) < current.cost {
            continue;
        }

        for dir in [(0, 1), (-1, 0), (1, 0), (0, -1)] {
            let npos =  current.pos + dir;
            if npos.y < 0
             || npos.x < 0
             || npos.uy() >= map.len()
             || npos.ux() >= map[npos.uy()].len()
             || map[npos.uy()][npos.ux()] == '#'
            {
                continue;
            }
            let next = State::of(npos, current.cost+1);
            if *dist.get(&next.pos).unwrap_or(&usize::MAX) <= next.cost {
                continue;
            }

            heap.push(next);
            *dist.entry(next.pos).or_default() = next.cost;
        }
    }
    None
}
fn dijkstra_with_cheats(start: &Pos, end: &Pos, map: &Vec<Vec<char>>, walls: &HashSet<Pos>, max_cost: usize) -> Vec<usize> {
    let mut steps = Vec::new();
    let mut m_map = map.clone();
    // let mut tried = HashSet::new();
    // let mut prints = 0;
    for wall1 in walls.iter() {
        m_map[wall1.uy()][wall1.ux()] = 'O';
        // for wall2 in walls.iter() {
        //     if wall1 == wall2  || tried.contains(&(wall1,wall2)){continue}
        //     m_map[wall2.uy()][wall2.ux()] = 'O';
            let option = dijkstra(start, end, &m_map, max_cost);
            if let Some(cost) = option {
                steps.push(cost);
                // if prints < 10 {
                //     println!("Cost: {}, saves {}", cost, max_cost-cost);
                //     for l in m_map.iter() {
                //         println!("{:?}", l.iter().collect::<String>());
                //     }
                //     prints += 1;
                // }
            }
            // m_map[wall2.uy()][wall2.ux()] = '#';
            // tried.insert((wall1,wall2));
            // tried.insert((wall2,wall1));
        // }
        m_map[wall1.uy()][wall1.ux()] = '#';
    }
    steps
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x: x as i32, y: y as i32 }
    }
    fn of(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn ux(&self) -> usize {
        self.x as usize
    }
    fn uy(&self) -> usize {
        self.y as usize
    }
}
impl Add<(i32, i32)> for Pos {
    type Output = Pos;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Pos::of(self.x + rhs.0, self.y + rhs.1)
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Pos,
}

impl State {
    fn of(pos: Pos, cost: usize) -> State {
        Self { cost, pos }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            //.then_with(|| self.pos.cmp(&other.pos))
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[ignore]
    #[test]
    fn run_day() {
        solve();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day20/input.txt";
        assert_eq!(1518, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day20/input.txt";
        let result = part_b(input);
        assert_eq!(1032257, result);
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day20/test-input.txt";
        let result = p1(input);
        // (picoseconds_saved, num_cheats)
        let cheats = HashMap::from([
            (2, 14) ,
            (4,14),
            (6,2),
            (8,4),
            (10,2),
            (12,3),
            (20,1),
            (36,1),
            (38,1),
            (40,1),
            (64,1),
        ]);
        assert_eq!(cheats, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day20/test-input.txt";
        let result = part_b(input);
        assert_eq!(0, result);
    }
}