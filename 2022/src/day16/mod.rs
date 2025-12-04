use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;

use crate::util::time;

pub fn day16() {
    println!("== Day 16 ==");
    let input = "src/day16/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    position: ValveId,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip to make min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    // Same as for ord
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
// #[display=(fmt = "{}{}", )]
struct ValveId(char, char);

impl ValveId {
    fn new(id: &str) -> Self {
        let chars = id.chars().collect::<Vec<char>>();
        Self(chars[0], chars[1])
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Valve {
    id: ValveId,
    flow_rate: u32,
    connected: Vec<ValveId>,
}

impl Valve {
    fn new(id: ValveId, flow_rate: u32, connected: Vec<ValveId>) -> Self {
        Self { id, flow_rate, connected }
    }
}

struct System {
    valves: HashMap<ValveId, Valve>,
    pressurized: HashSet<ValveId>,
    paths: HashMap<ValveId, HashMap<ValveId, u32>>,
}

impl System {
    fn parse(input: &str) -> Self {
        let mut valves = HashMap::new();
        let mut pressurized = HashSet::new();
        for line in input.lines() {
            let split = line.split("; ").collect::<Vec<&str>>();
            let s0 = split[0].split("=").collect::<Vec<&str>>();
            let this_valve = ValveId::new(s0[0].split(" ").collect::<Vec<&str>>()[1]);
            let flow_rate = s0[1].parse::<u32>().unwrap();

            let vec = split[1].split(" ").collect::<Vec<&str>>();
            let connected = vec.split_at(4).1.iter().map(|s| ValveId::new(s)).collect::<Vec<ValveId>>();

            valves.insert(this_valve, Valve::new(this_valve, flow_rate, connected));
            if flow_rate > 0 { pressurized.insert(this_valve); }
        }
        let paths = System::generate_paths(&valves);
        Self {
            valves,
            pressurized,
            paths,
        }
    }

    fn generate_paths(valves: &HashMap<ValveId, Valve>) -> HashMap<ValveId, HashMap<ValveId, u32>> {
        let mut paths = HashMap::new();
        for (valve, _connections) in valves.iter() {
            let mut p = HashMap::new();
            for (v, _c) in valves.iter() {
                if v == valve { continue; }
                if valves.get(v).unwrap().flow_rate == 0 { continue; }
                p.insert(*v, System::djikstra(valve, v, valves).unwrap());
            }
            paths.insert(*valve, p);
        }
        paths
    }

    fn djikstra(start: &ValveId, end: &ValveId, valves: &HashMap<ValveId, Valve>) -> Option<u32> {
        let mut dist: HashMap<ValveId, u32> = HashMap::from_iter(valves.iter().map(|(k, _v)| (*k, u32::MAX)));
        let mut heap = BinaryHeap::new();
        *dist.entry(*start).or_default() = 0;
        heap.push(State { cost: 0, position: *start });
        while let Some(State { cost, position }) = heap.pop() {
            if position == *end {
                return Some(cost);
            }
            if cost > *dist.get(&position).unwrap() {
                continue;
            }
            for valve in &valves.get(&position).unwrap().connected {
                let valve = *valve;
                let next = State { cost: cost + 1, position: valve };
                if next.cost < *dist.get(&next.position).unwrap() {
                    heap.push(next);
                    *dist.entry(next.position).or_default() = next.cost;
                }
            }
        }
        None
    }


    fn _find_path(&self, pos: &ValveId, time: u32, curr_time: u32, opened: HashSet<ValveId>) -> u32 {
        // println!("pos: {:?}, time: {}, curr_time: {:?}, opened: {:?}", pos, time, curr_time, opened);
        if opened.len() == self.pressurized.len() { // All valves opened
            return 0;
        }
        let mut pressure = 0;
        for valve_id in self.pressurized.iter() {
            if opened.contains(valve_id) {
                continue;
            }
            let release_time = curr_time + self.paths.get(pos).unwrap().get(valve_id).unwrap() + 1;
            if release_time <= time {
                let release_pressure = self.valves.get(valve_id).unwrap().flow_rate * (time - release_time);
                let mut open = opened.clone();
                open.insert(*valve_id);
                let sub = self._find_path(valve_id, time, release_time, open);

                pressure = max(pressure, release_pressure + sub);
            }
        }
        pressure
    }
    fn _find_path_with_elephant(&self, me_pos: &ValveId, el_pos: &ValveId, time: u32, me_time: u32, el_time: u32, opened: HashSet<ValveId>) -> u32 {
        // println!("pos: {:?}, time: {}, curr_time: {:?}, opened: {:?}", pos, time, curr_time, opened);
        if opened.len() == self.pressurized.len() { // All valves opened
            return 0;
        }
        let mut pressure = 0;
        for valve_id in self.pressurized.iter() {
            if opened.contains(valve_id) { continue; }
            let me_rt = me_time + self.paths.get(me_pos).unwrap().get(valve_id).unwrap() + 1;
            let el_rt = el_time + self.paths.get(el_pos).unwrap().get(valve_id).unwrap() + 1;
            if me_rt <= el_rt {
                // Me first!
                if me_rt <= time {
                    let release_pressure = self.valves.get(valve_id).unwrap().flow_rate * (time - me_rt);
                    let mut open = opened.clone();
                    open.insert(*valve_id);
                    let sub = self._find_path_with_elephant(valve_id, el_pos, time, me_rt, el_time, open);

                    pressure = max(pressure, release_pressure + sub);
                }
            } else {
                // Elephants turn
                if el_rt <= time {
                    let release_pressure = self.valves.get(valve_id).unwrap().flow_rate * (time - el_rt);
                    let mut open = opened.clone();
                    open.insert(*valve_id);
                    let sub = self._find_path_with_elephant(me_pos, valve_id, time, me_time, el_rt, open);

                    pressure = max(pressure, release_pressure + sub);
                }
            }
        }

        pressure
    }

    fn calc_paths(&self, curr_valve: &ValveId, time_left: u32, visited: HashMap<ValveId, u32>) -> (u32, Option<HashMap<ValveId, u32>>) {
        if time_left < 1 {
            return (0, Some(visited))
        }

        // self.valves.keys()//.iter()
        self.pressurized.iter()
            .filter(|target| *target != curr_valve)
            .filter_map(|target| {
                let distance = self.paths.get(curr_valve).unwrap().get(target);
                if distance.is_none() {
                    println!("No path from {:?} to {:?}", curr_valve, target);
                    println!("{:?}", self.paths);
                    panic!();
                    // return None;
                }
                let distance = *distance.unwrap();

                let time_left_after_open = time_left as i32 - distance as i32 - 1;
                if time_left_after_open < 0 {
                    return None;
                }
                let time_left_after_open = time_left_after_open as u32;

                let flow_rate = self.valves.get(target).unwrap().flow_rate;
                let valve_value = time_left_after_open * flow_rate;
                let current_target_value = *visited.get(target).unwrap_or(&0);
                if valve_value <= current_target_value {
                    return None
                }
                let mut visited = visited.to_owned();
                visited.insert(*target, valve_value);

                let mut result = self.calc_paths(
                    target,
                    time_left_after_open,
                    visited,
                );

                result.0 += valve_value - current_target_value;
                result.1 = result.1.to_owned();

                Some(result)
            })
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap_or((0, Some(visited)))
    }

    fn max_release(&self, start: &ValveId, time: u32) -> u32 {
        // self.find_path(start, time, 0, HashSet::new())
        let x = self.calc_paths(start, time, HashMap::new());
        x.0
    }
    fn max_release_with_elephant(&self, start: &ValveId, time: u32) -> u32 {
        // let valve_values = Vec::with_capacity(self.paths.len());
        // println!("{:?}", self.pressurized);
        let x = self.calc_paths(start, time, HashMap::new());
        // println!("{:?}", x);
        let y = self.calc_paths(start, time, x.1.unwrap());
        // println!("{:?}", y);
        x.0 + y.0
        // self.find_path_with_elephant(start, start, time, 0, 0, HashSet::new())
    }
}

impl fmt::Debug for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (id, _valve) in self.valves.iter() {
            writeln!(f, "Id: {}{} :: {:?}", id.0, id.1, self.paths.get(id).unwrap())?;
        }
        Ok(())
    }
}

fn part_a(input: &str) -> u32 {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let system = System::parse(open.as_str());
    // println!("{:?}", system);
    system.max_release(&ValveId::new("AA"), 30)
}

fn part_b(input: &str) -> u32 {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let system = System::parse(open.as_str());
    // println!("{:?}", system);
    system.max_release_with_elephant(&ValveId::new("AA"), 26)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day16();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day16/input.txt";
        assert_eq!(1728, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day16/input.txt";
        assert_eq!(2304, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day16/test-input.txt";
        let result = part_a(input);
        assert_eq!(1651, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day16/test-input.txt";
        let result = part_b(input);
        assert_eq!(1707, result);
    }
}