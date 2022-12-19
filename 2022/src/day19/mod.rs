use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Sub};

use crate::util::time;

pub fn day19() {
    println!("== Day 19 ==");
    let input = "src/day19/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Ore {
    Iron,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Blueprint {
    id: u32,
    iron_drone_cost: OreMap,
    clay_drone_cost: OreMap,
    obsidian_drone_cost: OreMap,
    geode_drone_cost: OreMap,
}

impl Blueprint {
    fn max_needed(&self, ore: &Ore) -> u32 {
        let mut cost = 0;
        for drone_cost in vec![self.iron_drone_cost.clone(),
                               self.clay_drone_cost.clone(),
                               self.obsidian_drone_cost.clone(),
                               self.geode_drone_cost.clone(),
        ].iter() {
            cost = max(cost, drone_cost.get(ore));
        }
        cost
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct OreMap {
    iron: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl OreMap {
    fn new() -> Self {
        Self {
            iron: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn get(&self, ore: &Ore) -> u32 {
        match ore {
            Ore::Iron => self.iron,
            Ore::Clay => self.clay,
            Ore::Obsidian => self.obsidian,
            Ore::Geode => self.geode,
        }
    }

    fn iron(&self, amount: u32) -> OreMap {
        let mut other = OreMap::new();
        other.iron = amount;
        *self + other
    }
    fn clay(&self, amount: u32) -> OreMap {
        let mut other = OreMap::new();
        other.clay = amount;
        *self + other
    }
    fn obsidian(&self, amount: u32) -> OreMap {
        let mut other = OreMap::new();
        other.obsidian = amount;
        *self + other
    }
    fn geode(&self, amount: u32) -> OreMap {
        let mut other = OreMap::new();
        other.geode = amount;
        *self + other
    }

    fn affords(&self, cost: &OreMap) -> bool {
        // println!("Checking if {:?}", self);
        // println!("affords     {:?}", cost);
        self.iron >= cost.iron
            && self.clay >= cost.clay
            && self.obsidian >= cost.obsidian
            && self.geode >= cost.geode
    }
}

impl Add for OreMap {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            iron: self.iron + other.iron,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl Sub for OreMap {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            iron: self.iron - other.iron,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Inventory {
    drones: OreMap,
    ores: OreMap,
}

impl Inventory {
    fn new() -> Self {
        let mut drones = OreMap::new();
        drones.iron = 1;
        Self {
            drones,
            ores: OreMap::new(),
        }
    }

    fn from(ores: OreMap, drones: OreMap) -> Self {
        Self {
            ores,
            drones,
        }
    }


    fn gather(self) -> Inventory {
        Inventory {
            drones: self.drones,
            ores: self.ores + self.drones,
        }
    }
    fn geodes(self) -> u32 {
        self.ores.geode
    }
}

#[derive(Debug)]
struct RobotFactory {
    blueprint: Blueprint,
    inventory: Inventory,
}

impl RobotFactory {
    fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            inventory: Inventory::new(),
        }
    }

    fn work(&self, turns: u32) -> u32 {
        let max_iron_needed = self.blueprint.max_needed(&Ore::Iron);
        let max_clay_needed = self.blueprint.max_needed(&Ore::Clay);
        let max_obsidian_needed = self.blueprint.max_needed(&Ore::Obsidian);

        // println!("Max needs: I: {}   C: {}   O: {}", max_iron_needed, max_clay_needed, max_obsidian_needed);

        // Even a slower start might go faster at the end
        // But if you are 2 after, you will probably never catch up
        // So end the route early
        let max_fall_off = 2;

        let start = self.inventory.clone();
        let mut queue: VecDeque<(Inventory, u32)> = VecDeque::new();
        queue.push_back((start, turns));

        let mut cache: HashMap<u32, u32> = HashMap::new(); // Cache with key = turn and value = geodes
        let mut seen: HashSet<Inventory> = HashSet::new();
        while let Some((state, turns_left)) = queue.pop_front() {
            let prev_best = *cache.get(&turns_left).unwrap_or(&0);
            // println!("{} Looking at: {:?}", turns_left, state);
            // println!("Prev best was {}", prev_best);
            // println!();
            if state.geodes() + max_fall_off < prev_best {
                // Early kill of the route, it's too far behind
                // println!("Early kill {} + {} < {}", state.geodes(), max_fall_off, prev_best);
                continue;
            }

            cache.insert(turns_left, max(prev_best, state.geodes()));

            if turns_left == 0 {
                continue; // End of the road
            }

            // We've already done this. Do something else
            if seen.contains(&state) {
                continue;
            }
            seen.insert(state);

            let new_state = state.gather();
            let next_best = *cache.get(&(turns_left - 1)).unwrap_or(&0);
            if new_state.geodes() + max_fall_off < next_best {
                continue; // Same as above. early kill
            }


            if state.drones.iron < max_iron_needed && state.ores.affords(&self.blueprint.iron_drone_cost) {
                let new_ores = new_state.ores - self.blueprint.iron_drone_cost;
                let new_drones = new_state.drones + OreMap::new().iron(1);
                let new_inventory = Inventory::from(new_ores, new_drones);
                queue.push_back((new_inventory, turns_left - 1));
            }
            if state.drones.clay < max_clay_needed && state.ores.affords(&self.blueprint.clay_drone_cost) {
                let new_ores = new_state.ores - self.blueprint.clay_drone_cost;
                let new_drones = new_state.drones + OreMap::new().clay(1);
                let new_inventory = Inventory::from(new_ores, new_drones);
                queue.push_back((new_inventory, turns_left - 1));
            }
            if state.drones.obsidian < max_obsidian_needed && state.ores.affords(&self.blueprint.obsidian_drone_cost) {
                let new_ores = new_state.ores - self.blueprint.obsidian_drone_cost;
                let new_drones = new_state.drones + OreMap::new().obsidian(1);
                let new_inventory = Inventory::from(new_ores, new_drones);
                queue.push_back((new_inventory, turns_left - 1));
            }
            if state.ores.affords(&self.blueprint.geode_drone_cost) {
                let new_ores = new_state.ores - self.blueprint.geode_drone_cost;
                let new_drones = new_state.drones + OreMap::new().geode(1);
                let new_inventory = Inventory::from(new_ores, new_drones);
                queue.push_back((new_inventory, turns_left - 1));
            } else {
                // println!("Pushing: {:?}", new_state);
                queue.push_back((new_state, turns_left - 1));
            }
        }
        *cache.get(&0).unwrap()
    }
}

fn parse_blueprints(input: &str) -> HashMap<u32, Blueprint> {
    let mut blueprints = HashMap::new();
    for bp in input.lines() {
        let s = bp.split(": ").collect::<Vec<&str>>();
        let id = s[0].split(" ").last().unwrap().parse::<u32>().unwrap();
        let costs = s[1].split(" ").collect::<Vec<&str>>();
        blueprints.insert(
            id,
            Blueprint {
                id,
                iron_drone_cost: OreMap::new().iron(costs[4].parse::<u32>().unwrap()),
                clay_drone_cost: OreMap::new().iron(costs[10].parse::<u32>().unwrap()),
                obsidian_drone_cost: OreMap::new().iron(costs[16].parse::<u32>().unwrap()).clay(costs[19].parse::<u32>().unwrap()),
                geode_drone_cost: OreMap::new().iron(costs[25].parse::<u32>().unwrap()).obsidian(costs[28].parse::<u32>().unwrap()),

            },
        );
    }
    blueprints
}

fn part_a(input: &str) -> u32 {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let blueprints = parse_blueprints(open.as_str());

    blueprints.values()
        .map(|bp| RobotFactory::new(bp.clone()).work(24) * bp.id)
        .sum()
}

fn part_b(input: &str) -> u32 {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let blueprints = parse_blueprints(open.as_str());
    let reduced = blueprints.values().filter(|bp| bp.id < 4).collect::<Vec<&Blueprint>>();

    reduced.iter()
        .map(|bp| RobotFactory::new((*bp).clone()).work(32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day19();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day19/input.txt";
        assert_eq!(1150, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day19/input.txt";
        assert_eq!(37367, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day19/test-input.txt";
        let result = part_a(input);
        assert_eq!(33, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day19/test-input.txt";
        let result = part_b(input);
        assert_eq!(56 * 62, result);
    }

    #[test]
    fn ore_map_affords() {
        {
            let ores = OreMap::new();
            let cost = OreMap::new();
            assert_eq!(true, ores.affords(&cost))
        }
        {
            let ores = OreMap::new();
            let cost = OreMap::new().iron(1);
            assert_eq!(false, ores.affords(&cost))
        }
        {
            let ores = OreMap::new().iron(1);
            let cost = OreMap::new().iron(1);
            assert_eq!(true, ores.affords(&cost))
        }
        {
            let ores = OreMap::new().iron(1);
            let cost = OreMap::new().iron(1).geode(1);
            assert_eq!(false, ores.affords(&cost))
        }
    }

    #[test]
    fn bluep() {
        let blueprint = Blueprint {
            id: 0,
            iron_drone_cost: OreMap::new().iron(1),
            clay_drone_cost: OreMap::new().iron(2),
            obsidian_drone_cost: OreMap::new().iron(2).clay(3),
            geode_drone_cost: OreMap::new().iron(3).obsidian(14),
        };
        assert_eq!(3, blueprint.max_needed(&Ore::Iron));
        assert_eq!(3, blueprint.max_needed(&Ore::Clay));
        assert_eq!(14, blueprint.max_needed(&Ore::Obsidian));
        assert_eq!(0, blueprint.max_needed(&Ore::Geode));
    }
}