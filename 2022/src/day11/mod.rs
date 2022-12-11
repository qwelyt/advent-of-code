use crate::util::{lines, time, vecs};

pub fn day11() {
    println!("== Day 11 ==");
    let input = "src/day11/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let monkeys = run_monkeys(input, 20, true);
    let mut inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn part_b(input: &str) -> usize {
    let monkeys = run_monkeys(input, 10_000, false);
    let mut inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    div_by: usize,
    test: Box<dyn Fn(usize) -> bool>,
    if_true: i32,
    if_false: i32,
    inspected: usize,
}

impl Monkey {
    fn push(&mut self, item: usize) {
        self.items.push(item);
    }
    fn clear(&mut self) {
        self.items.clear();
    }
    fn inspected(&mut self) {
        self.inspected += 1;
    }
}

fn revert_to_mokeh(input: &Vec<String>) -> Monkey {
    let mut items = Vec::new();
    let mut op: Box<dyn Fn(usize) -> usize> = Box::new(|i| i);
    let mut test: Box<dyn Fn(usize) -> bool> = Box::new(|_i| false);
    let mut div_by = 0;
    let mut if_true = 0;
    let mut if_false = 0;
    for line in input.iter() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        match parts[0].trim() {
            "Starting items" => {
                items = parts[1].split(", ").map(|s| s.parse::<usize>().unwrap()).collect();
            }
            "Test" => {
                let o = parts[1].split(" ").collect::<Vec<&str>>();
                let d = o.last().map(|s| s.parse::<usize>().unwrap()).unwrap();
                div_by = d;
                test = Box::new(move |old| old % div_by == 0);
            }
            "If true" => {
                let o = parts[1].split(" ").collect::<Vec<&str>>();
                if_true = o.last().map(|s| s.parse::<i32>().unwrap()).unwrap();
            }
            "If false" => {
                let o = parts[1].split(" ").collect::<Vec<&str>>();
                if_false = o.last().map(|s| s.parse::<i32>().unwrap()).unwrap();
            }
            "Operation" => {
                let o = parts[1].split(" = ").collect::<Vec<&str>>();
                let p = o[1].split(" ").collect::<Vec<&str>>();
                match p[1] {
                    "+" => {
                        if p[2].eq("old") {
                            op = Box::new(|old| old + old);
                        } else {
                            let value = p[2].parse::<usize>().unwrap();
                            op = Box::new(move |old| old + value);
                        }
                    }
                    "-" => {
                        if p[2].eq("old") {
                            op = Box::new(|old| old - old);
                        } else {
                            let value = p[2].parse::<usize>().unwrap();
                            op = Box::new(move |old| old - value);
                        }
                    }
                    "*" => {
                        if p[2].eq("old") {
                            op = Box::new(|old| old * old);
                        } else {
                            let value = p[2].parse::<usize>().unwrap();
                            op = Box::new(move |old| old * value);
                        }
                    }
                    "/" => {
                        if p[2].eq("old") {
                            op = Box::new(|old| old / old);
                        } else {
                            let value = p[2].parse::<usize>().unwrap();
                            op = Box::new(move |old| old / value);
                        }
                    }
                    &_ => {
                        println!("Not implemented: {}", parts[1]);
                    }
                }
            }
            &_ => {}
        }
    }

    return Monkey {
        items,
        operation: op,
        div_by,
        test,
        if_true,
        if_false,
        inspected: 0,
    };
}

fn monkehs(input: &str) -> Vec<Monkey> {
    let lines = lines(input);
    let vecs1 = vecs(&lines);
    vecs1.iter().map(|v| revert_to_mokeh(v)).collect::<Vec<Monkey>>()
}


fn run_monkeys(input: &str, rounds: i32, divide: bool) -> Vec<Monkey> {
    let mut monkeys = monkehs(input);
    let common_mod: usize = monkeys.iter().map(|m| m.div_by).product();
    for _i in 0..rounds {
        // println!("== Round {} ==", i);
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            // println!("== {} ==", i);
            // println!("Items: {:?}", monkey.items);
            // println!("True: {:?}", monkey.if_true);
            // println!("False: {:?}", monkey.if_false);
            let mut items_to_throw = Vec::new();
            for it in 0..monkey.items.len() {
                let item = monkey.items.get(it).unwrap();
                let x: usize = (monkey.operation)(*item).into();
                let y: usize = if divide { x / 3 } else { x % common_mod };
                let test: bool = (monkey.test)(y).into();
                let throw_to = if test { monkey.if_true } else { monkey.if_false };
                // println!("X: {:?} / 3 = {} :: {} -> {}", x, y, test, throw_to);
                items_to_throw.push((throw_to, y));
                monkey.inspected();
            }
            monkey.clear();
            // let _ = items_to_throw.iter().map(|t| monkeys.get_mut(t.0 as usize).unwrap().push(t.1));
            for tuple in items_to_throw.iter() {
                let m = monkeys.get_mut(tuple.0 as usize).unwrap();
                m.push(tuple.1);
            }
            // println!();
        }
        // if (_i+1)%20 == 0 || _i==0{
        //     println!("== Round {} ==", _i);
        //     for m in monkeys.iter() {
        //         println!("{} ::: {:?}", m.inspected, m.items);
        //     }
        //     println!();
        // }
    }
    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day11();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day11/input.txt";
        assert_eq!(64032, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day11/input.txt";
        assert_eq!(12729522272, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day11/test-input.txt";
        let result = part_a(input);
        assert_eq!(10605, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day11/test-input.txt";
        let result = part_b(input);
        assert_eq!(2713310158, result);
    }

    #[test]
    fn test_b_rounds() {
        let input = "src/day11/test-input.txt";
        assert_eq!(vec![2, 4, 3, 6], run_monkeys(input, 1, false).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![99, 97, 8, 103], run_monkeys(input, 20, false).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![5204, 4792, 199, 5192], run_monkeys(input, 1_000, false).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![26075, 23921, 974, 26000], run_monkeys(input, 5_000, false).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![46945, 43051, 1746, 46807], run_monkeys(input, 9_000, false).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![52166, 47830, 1938, 52013], run_monkeys(input, 10_000, false).iter().map(|m| m.inspected).collect::<Vec<usize>>());
    }
}