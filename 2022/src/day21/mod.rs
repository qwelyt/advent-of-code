use std::collections::HashMap;

use crate::util::time;

pub fn day21() {
    println!("== Day 21 ==");
    let input = "src/day21/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
enum Operation {
    Add,
    Sub,
    Div,
    Mul,
    Eql,
}

impl From<&str> for Operation {
    fn from(str: &str) -> Self {
        match str {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "/" => Operation::Div,
            "*" => Operation::Mul,
            "=" => Operation::Eql,
            &_ => panic!("Can't parse {}", str)
        }
    }
}

impl Operation {
    fn solve(&self, lhs: isize, rhs: isize) -> isize {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Sub => lhs - rhs,
            Operation::Div => lhs / rhs,
            Operation::Mul => lhs * rhs,
            Operation::Eql => (lhs == rhs) as isize,
        }
    }
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
enum MonkeyValue {
    Value(isize),
    Equation(MonkeyId, Operation, MonkeyId),
}

impl MonkeyValue {
    #[allow(dead_code)]
    fn is_equation(&self) -> bool {
        match self {
            MonkeyValue::Value(_) => false,
            MonkeyValue::Equation(_, _, _) => true,
        }
    }
    fn is_value(&self) -> bool {
        match self {
            MonkeyValue::Value(_) => true,
            MonkeyValue::Equation(_, _, _) => false,
        }
    }
    fn lhs(&self) -> MonkeyId {
        match self {
            MonkeyValue::Value(_) => panic!("Can't get id of value"),
            MonkeyValue::Equation(lhs, _, _) => *lhs,
        }
    }
    fn rhs(&self) -> MonkeyId {
        match self {
            MonkeyValue::Value(_) => panic!("Can't get id of value"),
            MonkeyValue::Equation(_, _, rhs) => *rhs,
        }
    }
    #[allow(dead_code)]
    fn contains(&self, monkey_id: &MonkeyId) -> bool {
        match self {
            MonkeyValue::Value(_) => false,
            MonkeyValue::Equation(lhs, _, rhs) => lhs == monkey_id || rhs == monkey_id
        }
    }
    fn equation(&self) -> Option<(MonkeyId, Operation, MonkeyId)> {
        match self {
            MonkeyValue::Value(_) => None,
            MonkeyValue::Equation(lhs, op, rhs) => Some((*lhs, *op, *rhs))
        }
    }
    fn value(&self) -> Option<isize> {
        match self {
            MonkeyValue::Value(i) => Some(*i),
            MonkeyValue::Equation(_, _, _) => None,
        }
    }
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
struct Monkey {
    id: MonkeyId,
    value: MonkeyValue,
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
struct MonkeyId {
    id: (char, char, char, char), // Because Rust and Strings a effing annoying.
}

impl From<&str> for MonkeyId {
    fn from(str: &str) -> Self {
        let chars = str.chars().collect::<Vec<char>>();
        Self {
            id: (chars[0], chars[1], chars[2], chars[3])
        }
    }
}

struct MonkeyMath {
    monkeys: HashMap<MonkeyId, Monkey>,
}

impl MonkeyMath {
    fn parse(input: &str) -> Self {
        let mut monkeys = HashMap::new();
        for line in input.lines() {
            let split = line.split(": ").collect::<Vec<&str>>();
            let id = MonkeyId::from(split[0]);
            let result = split[1].parse::<isize>();
            let value = match result {
                Ok(int) => { MonkeyValue::Value(int) }
                Err(_) => {
                    let spl = split[1].split(" ").collect::<Vec<&str>>();
                    MonkeyValue::Equation(MonkeyId::from(spl[0]), Operation::from(spl[1]), MonkeyId::from(spl[2]))
                }
            };
            let monkey = Monkey { id, value };
            monkeys.insert(id, monkey);
        }
        Self { monkeys }
    }

    fn solve(&self, start: &MonkeyId) -> isize {
        let monkeh = self.monkeys.get(start).unwrap();
        match monkeh.value {
            MonkeyValue::Value(v) => { v }
            MonkeyValue::Equation(lhs, op, rhs) => {
                op.solve(self.solve(&lhs), self.solve(&rhs))
            }
        }
    }
    #[allow(dead_code)]
    fn find_path_for(&self, path_for: &MonkeyId, top: &MonkeyId) -> Vec<MonkeyId> {
        let mut path: Vec<MonkeyId> = Vec::new();
        path.push(*path_for);
        let parents = self.monkeys.iter()
            .filter(|(_id, monkey)| monkey.value.is_equation())
            .filter(|(_id, monkey)| monkey.value.contains(&path_for))
            .map(|(id, _)| *id)
            .collect::<Vec<MonkeyId>>();
        if parents.len() > 1 {
            panic!("Need to handle more parents! {}", parents.len());
        }
        let parent = parents.first().unwrap();
        if parent == top {
            path.push(*parent);
        } else {
            let mut vec = self.find_path_for(parent, top);
            path.append(vec.as_mut());
        }
        path
    }

    fn solve_static(monkeys: &HashMap<MonkeyId, Monkey>, start: &MonkeyId) -> Option<isize> {
        let monkeh = monkeys.get(start);
        match monkeh {
            None => None,
            Some(val) => {
                match val.value {
                    MonkeyValue::Value(v) => { Some(v) }
                    MonkeyValue::Equation(lhs, op, rhs) => {
                        let lhs_o = MonkeyMath::solve_static(monkeys, &lhs);
                        let rhs_o = MonkeyMath::solve_static(monkeys, &rhs);
                        if lhs_o.is_some() && rhs_o.is_some() {
                            return Some(op.solve(lhs_o.unwrap(), rhs_o.unwrap()));
                        }
                        None
                    }
                }
            }
        }
    }

    fn reduce(monkeys: &HashMap<MonkeyId, Monkey>) -> HashMap<MonkeyId, Monkey> {
        let mut map = monkeys.clone();
        for (id, _m) in monkeys.iter() {
            let option = MonkeyMath::solve_static(&map, id);
            if option.is_some() {
                map.insert(*id, Monkey { id: *id, value: MonkeyValue::Value(option.unwrap()) });
            }
        }
        map
    }

    fn find_value_for(&self, should_equal: &MonkeyId, change: &MonkeyId) -> isize {
        let mut cmc = self.monkeys.clone();
        // Remove "humn" and change "root" to be equals
        cmc.remove(change);
        if let Some(m) = cmc.get_mut(should_equal) {
            m.value = MonkeyValue::Equation(m.value.lhs(), Operation::Eql, m.value.rhs());
        }
        // Solve as much as possible so we can get the actual numbers
        let reduced = MonkeyMath::reduce(&cmc);
        let numbers: HashMap<MonkeyId, MonkeyValue> = reduced.values()
            .filter(|v| v.value.is_value())
            .map(|v| (v.id, v.value))
            .collect();

        // Massive block of code, what does it do?
        // We start a loop where we want to find "change" (humn)
        // This value has been removed from the map so the equations that need that value
        // are unresolved. However, we have resolved "should_equal" (root). We know that
        // we need to find what ever value the other leg for "should_equal" has. This means
        // we have a target value. All our unresolved equations must result in that value.
        // We start backwards then. "find X for blabla". If the equation says we should do
        // addition, we do subtraction. If we should do subtraction of A-B, we do B-A.
        // If multiplication we divide.
        // If division is A / B we do B/A.
        // At the end we have the result that "change" must be for the unresolved equations to
        // produce the correct result.
        let mut search = *should_equal;
        let mut result = 0;
        while search != *change {
            let x = reduced.get(&search).unwrap();
            let (lhs, op, rhs) = x.value.equation().unwrap();
            (search, result) = match (op, numbers.get(&lhs), numbers.get(&rhs)) {
                (Operation::Eql, None, Some(m)) => (lhs, m.value().unwrap()),
                (Operation::Eql, Some(m), None) => (rhs, m.value().unwrap()),
                (Operation::Add, None, Some(m)) => (lhs, result - m.value().unwrap()),
                (Operation::Add, Some(m), None) => (rhs, result - m.value().unwrap()),
                (Operation::Sub, None, Some(m)) => (lhs, result + m.value().unwrap()),
                (Operation::Sub, Some(m), None) => (rhs, m.value().unwrap() - result),
                (Operation::Mul, None, Some(m)) => (lhs, result / m.value().unwrap()),
                (Operation::Mul, Some(m), None) => (rhs, result / m.value().unwrap()),
                (Operation::Div, None, Some(m)) => (lhs, result * m.value().unwrap()),
                (Operation::Div, Some(m), None) => (rhs, m.value().unwrap() / result),
                _ => panic!()
            }
        }
        result
    }
}

fn part_a(input: &str) -> isize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let math = MonkeyMath::parse(open.as_str());
    math.solve(&MonkeyId::from("root"))
}

fn part_b(input: &str) -> isize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let math = MonkeyMath::parse(open.as_str());
    math.find_value_for(&MonkeyId::from("root"), &MonkeyId::from("humn"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day21();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day21/input.txt";
        assert_eq!(379578518396784, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day21/input.txt";
        assert_eq!(3353687996514, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day21/test-input.txt";
        let result = part_a(input);
        assert_eq!(152, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day21/test-input.txt";
        let result = part_b(input);
        assert_eq!(301, result);
    }
}