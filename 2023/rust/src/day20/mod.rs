use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day20() {
    println!("== Day 20 ==");
    let input = "src/day20/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let mut modules = get_modules(input);
    // _print_modules(&modules);
    let (mut sum_low, mut sum_high) = (0, 0);
    for _ in 0..1000 {
        let (low, high) = press_button(&mut modules);
        sum_low += low;
        sum_high += high;
        // println!("Pressed buttons: ({:?}, {:?})", sum_low, sum_high);
        // _print_modules(&modules);
        // println!("\n");
    }
    sum_low * sum_high
}

fn part_b(input: &str) -> usize {
    let modules = get_modules(input);
    let paths = paths_to("rx", &modules);
    let mut presses = Vec::new();
    for path in paths.iter() {
        let x = path.last().unwrap();
        presses.push(presses_for(x, &modules))
    }
    presses.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
}

fn lcm(a: usize, b: usize) -> usize {
    let mut tmp = a;
    while tmp % b != 0 {
        tmp += a;
    }
    return tmp;
}

fn _print_modules(m: &HashMap<String, Module>) {
    for (k, v) in m.iter() {
        println!("{:?}: {:?}", k, v);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ModuleType {
    FlipFlop,
    // %
    Conjunction,
    // &
    Broadcast,
    Button,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    kind: ModuleType,
    active: bool,
    memory: HashMap<String, bool>,
    output: Vec<String>,
}

impl Module {
    fn parse(string: &str) -> Self {
        // broadcaster -> a
        // %a -> inv, con
        // &inv -> b
        let (this, outputs) = string.split_once(" -> ").unwrap();
        let kind = match this.chars().next().unwrap() {
            'b' => match this.chars().nth(1).unwrap() {
                'r' => ModuleType::Broadcast,
                'u' => ModuleType::Button,
                _ => panic!()
            }
            '%' => ModuleType::FlipFlop,
            '&' => ModuleType::Conjunction,
            _ => panic!()
        };
        let name = if this.starts_with("b") {
            this.to_string()
        } else {
            let mut chars = this.chars();
            chars.next();
            chars.as_str().to_string()
        };

        let output = outputs.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        Self {
            name,
            kind,
            active: false,
            memory: HashMap::new(),
            output,
        }
    }

    fn pulse(&mut self, source: &str, high: bool) -> Vec<Pulse> {
        let mut pulses = Vec::new();
        match self.kind {
            ModuleType::Button => {}
            ModuleType::Broadcast => self.output.iter()
                .map(|m| Pulse::of(self.name.as_str(), m.as_str(), high))
                .for_each(|p| pulses.push(p)),
            ModuleType::FlipFlop => if !high {
                self.active = !self.active;
                self.output.iter()
                    .map(|m| Pulse::of(self.name.as_str(), m.as_str(), self.active))
                    .for_each(|p| pulses.push(p));
            },
            ModuleType::Conjunction => {
                self.memory.insert(source.to_string(), high);
                let high_low = self.memory.values().fold(true, |a, b| a && *b);
                self.output.iter()
                    .map(|m| Pulse::of(self.name.as_str(), m.as_str(), !high_low))
                    .for_each(|p| pulses.push(p))
            }
        }

        self.memory.insert(source.to_string(), high);
        pulses
    }
}


#[derive(Debug, Clone)]
struct Pulse {
    source: String,
    destination: String,
    high: bool,
}

impl Pulse {
    fn of(src: &str, dest: &str, high: bool) -> Self {
        Self {
            source: src.to_string(),
            destination: dest.to_string(),
            high,
        }
    }
}

fn press_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    // println!("Press button!");
    let (mut low, mut high) = (0, 0);
    let mut pulses = VecDeque::new();
    pulses.push_back(Pulse::of("button", "broadcaster", false));
    while let Some(pulse) = pulses.pop_front() {
        // println!("Pulse: {:?}", pulse);
        if pulse.high {
            high += 1;
        } else {
            low += 1;
        }
        if let Some(destination) = modules.get_mut(pulse.destination.as_str()) {
            // println!("== {:?}", destination);
            let new_pulses = destination.pulse(pulse.source.as_str(), pulse.high);
            // println!("== {:?}", destination);
            new_pulses.into_iter().for_each(|p| pulses.push_back(p));
        }
    }

    (low, high)
}

fn get_modules(input: &str) -> HashMap<String, Module> {
    let file = File::open(input).unwrap();
    let mut modules = HashMap::new();
    for line in BufReader::new(file).lines().flatten() {
        let module = Module::parse(line.as_str());
        modules.insert(module.name.clone(), module);
    }
    // Init memory
    for (k, v) in modules.clone().iter() {
        for other_module in v.output.iter() {
            if let Some(om) = modules.get_mut(other_module.as_str()) {
                om.memory.insert(k.clone(), false);
            }
        }
    }
    modules.insert("button".to_string(), Module::parse("button -> broadcast"));
    modules
}

fn paths_to(module: &str, modules: &HashMap<String, Module>) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let goal = modules.get(module).unwrap();
    for m in goal.memory.keys() {
        let mut path = Vec::new();
        if m == "broadcaster" {
            paths.push(path);
        }
    }

    paths
}

fn presses_for(module: &str, modules: &HashMap<String, Module>) -> usize {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day20();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day20/input.txt";
        assert_eq!(681194780, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day20/input.txt";
        assert_eq!(0, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day20/test-input.txt";
        let result = part_a(input);
        assert_eq!(11687500, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day20/test-input.txt";
        let result = part_b(input);
        assert_eq!(0, result);
    }
}