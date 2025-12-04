use crate::util::{lines, time, vecs};

pub fn day11_2() {
    println!("== Day 11 - 2 ==");
    let input = "src/day11/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let monkehs = monkehs(input);
    // let worry_reducer: Box<dyn Fn(i128) -> i128> = Box::new(|x| x / 3);
    let worry_reducer = |x: i128| x / 3;
    let monkeys = run_monkehs(20, &monkehs, &worry_reducer);
    let mut inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn part_b(input: &str) -> usize {
    let monkeys = monkehs(input);
    let common_mod: i128 = monkeys.iter().map(|m| m.div_by).product();
    let worry_reducer = |x: i128| x % common_mod;
    let monkeys = run_monkehs(10_000, &monkeys, &worry_reducer);
    let mut inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Operation {
    Add,
    Mul,
    Div,
    Sub,
    Mod,
}

impl Operation {
    fn parse(str: &str) -> Operation {
        match str {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            "%" => Operation::Mod,
            &_ => {
                panic!("Not an operation: {}", str)
            }
        }
    }
    fn run(&self, lhs: &i128, rhs: &i128) -> i128 {
        match self {
            Operation::Add => { lhs + rhs }
            Operation::Mul => { lhs * rhs }
            Operation::Div => { lhs / rhs }
            Operation::Sub => { lhs - rhs }
            Operation::Mod => { lhs % rhs }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Monkey {
    items: Vec<i128>,
    lhs: Option<i128>,
    operation: Operation,
    div_by: i128,
    test: Operation,
    if_true: i32,
    if_false: i32,
    inspected: usize,
}

impl Monkey {
    fn push(&mut self, item: i128) {
        self.items.push(item);
    }
    fn clear(&mut self) {
        self.items.clear();
    }
    fn inspected(&mut self, items: usize) {
        self.inspected += items;
    }
    fn operation(&self, rhs: &i128) -> i128 {
        if self.lhs.is_some() {
            return self.operation.run(rhs, &self.lhs.unwrap());
        }
        self.operation.run(rhs, rhs)
    }
    fn test(&self, old: &i128) -> bool {
        self.test.run(old, &self.div_by) == 0
    }
    fn throw_to(&self, value: &i128) -> i32 {
        if self.test(value) {
            return self.if_true;
        }
        self.if_false
    }

    fn inspect_items(&mut self, worry_reducer: &dyn Fn(i128) -> i128) -> Vec<(i32, i128)> {
        let items_to_throw: Vec<(i32, i128)> = self.items.iter()
            .map(|i| self.operation(i))
            .map(|i| {
                let a: i128 = worry_reducer(i).into();
                a
            })
            .map(|i| (self.throw_to(&i), i))
            .collect();
        self.inspected(items_to_throw.len());
        self.clear();
        items_to_throw
    }
}

fn return_to_mokeh(input: &Vec<String>) -> Monkey {
    let mut items = Vec::new();
    let mut operation: Operation = Operation::Add;
    let mut lhs: Option<i128> = None;
    let mut test: Operation = Operation::Div;
    let mut div_by = 0;
    let mut if_true = 0;
    let mut if_false = 0;
    for line in input.iter() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        match parts[0].trim() {
            "Starting items" => {
                items = parts[1].split(", ").map(|s| s.parse::<i128>().unwrap()).collect();
            }
            "Test" => {
                let o = parts[1].split(" ").collect::<Vec<&str>>();
                let d = o.last().map(|s| s.parse::<i128>().unwrap()).unwrap();
                div_by = d;
                test = Operation::Mod;
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
                operation = Operation::parse(p[1]);
                lhs = if p[2].eq("old") { None } else { p[2].parse::<i128>().ok() };
            }
            &_ => {}
        }
    }

    return Monkey {
        items,
        lhs,
        operation,
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
    vecs1.iter().map(|v| return_to_mokeh(v)).collect::<Vec<Monkey>>()
}

fn run_monkehs(rounds: i32, monkeys: &Vec<Monkey>, worry_reducer: &dyn Fn(i128) -> i128) -> Vec<Monkey> {
    let mut monkeys = monkeys.clone();
    for _i in 0..rounds {
        // println!("== Round {} ==", i);
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let items_to_throw: Vec<(i32, i128)> = monkey.inspect_items(worry_reducer);
            for tuple in items_to_throw.iter() {
                let m = monkeys.get_mut(tuple.0 as usize).unwrap();
                m.push(tuple.1);
            }
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
        day11_2();
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
        let monkehs = monkehs(input);
        let common_mod: i128 = monkehs.iter().map(|m| m.div_by).product();
        let worry_reducer = |x: i128| x % common_mod;
        assert_eq!(vec![2, 4, 3, 6], run_monkehs(1, &monkehs, &worry_reducer).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![99, 97, 8, 103], run_monkehs(20, &monkehs, &worry_reducer).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![5204, 4792, 199, 5192], run_monkehs(1_000, &monkehs, &worry_reducer).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![26075, 23921, 974, 26000], run_monkehs(5_000, &monkehs, &worry_reducer).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![46945, 43051, 1746, 46807], run_monkehs(9_000, &monkehs, &worry_reducer).iter().map(|m| m.inspected).collect::<Vec<usize>>());
        assert_eq!(vec![52166, 47830, 1938, 52013], run_monkehs(10_000, &monkehs, &worry_reducer).iter().map(|m| m.inspected).collect::<Vec<usize>>());
    }
}