use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day19() {
    println!("== Day 19 ==");
    let input = "src/day19/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Debug, Copy, Clone)]
enum Op {
    GT,
    LT,
}

impl Op {
    fn of(c: &char) -> Self {
        match c {
            &'>' => Op::GT,
            &'<' => Op::LT,
            &_ => panic!(),
        }
    }
    fn test(&self, a: u32, b: u32) -> bool {
        match self {
            Op::GT => a > b,
            Op::LT => a < b,
        }
    }
}

#[derive(Clone, Debug)]
struct Predicate {
    number: u32,
    field: char,
    op: Op,
    go: String,
}

impl Predicate {
    fn parse(string: &str) -> Self {
        let (tst, goal) = string.split_once(":").unwrap_or((string, ""));

        if goal.is_empty() {
            return Self {
                number: 0,
                field: 'A',
                op: Op::GT,
                go: tst.to_string(),
            };
        }

        // "a<100" -> ["a<","100"]
        let number = tst.split_at(2).1.parse::<u32>().unwrap();
        Self {
            number,
            field: tst.chars().next().unwrap(),
            op: Op::of(&tst.chars().nth(1).unwrap()),
            go: goal.to_string(),
        }
    }

    fn check(&self, input: &Part) -> Option<String> {
        if self.field == 'A' { return Some(self.go.to_string()); }
        let part_number = match self.field {
            'x' => input.xmas[0],
            'm' => input.xmas[1],
            'a' => input.xmas[2],
            's' => input.xmas[3],
            _ => 0
        };
        return if self.op.test(part_number, self.number) {
            Some(self.go.clone())
        } else {
            None
        };
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Part {
    xmas: [u32; 4],
}

impl Part {
    fn of(x: u32, m: u32, a: u32, s: u32) -> Self {
        Self { xmas: [x, m, a, s] }
    }
    fn parse(line: &str) -> Self {
        let mut chars = line.chars();
        chars.next(); // drop {
        chars.next_back(); // drop }
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;

        for part in chars.as_str().split(",") {
            let (c, v) = part.split_once("=").unwrap();
            let value = v.parse::<u32>().unwrap();
            match c {
                "x" => x = value,
                "m" => m = value,
                "a" => a = value,
                "s" => s = value,
                &_ => panic!()
            }
        }
        Self { xmas: [x, m, a, s] }
    }
    fn sum(&self) -> u32 {
        // self.x + self.m + self.a + self.s
        self.xmas.iter().sum()
    }
}

fn part_a(input: &str) -> u32 {
    let (workflows, parts) = parse_input(input);
    sum_accepted(&workflows, &parts)
}

fn part_b(input: &str) -> usize {
    let (workflows, _parts) = parse_input(input);
    possible_parts(&workflows)
}

fn parse_input(input: &str) -> (HashMap<String, Vec<Predicate>>, Vec<Part>) {
    let f = File::open(input).unwrap();
    let mut i = 0;
    let mut workflows = HashMap::<String, Vec<Predicate>>::new();
    let mut parts = Vec::new();
    for line in BufReader::new(f).lines().flatten() {
        if line.is_empty() {
            i += 1;
            continue;
        }
        if i == 0 {
            let (key, mut tests) = line.split_once("{").unwrap();
            let mut chars = tests.chars();
            chars.next_back();
            tests = chars.as_str();
            // println!("key: {:?} , tests: {:?}", key, tests);
            let tst = tests.split(",").collect::<Vec<&str>>();
            // println!("{:?}", tst);
            let predicates = tst.iter()
                .map(|s| Predicate::parse(*s))
                .collect::<Vec<Predicate>>();

            // println!("preds: {:?}", predicates);
            workflows.insert(key.to_string(), predicates);
        } else {
            parts.push(Part::parse(line.as_str()));
        }
    }
    (workflows, parts)
}

fn sum_accepted(workflows: &HashMap<String, Vec<Predicate>>, parts: &Vec<Part>) -> u32 {
    // for (k,v) in workflows.iter(){
    //     println!("{:?} : {:?}",k,v);
    // }
    parts.iter().map(|p| test_part(workflows, p)).sum()
}

fn test_part(workflows: &HashMap<String, Vec<Predicate>>, part: &Part) -> u32 {
    let mut workflow = workflows.get("in");
    // println!("Testing part {:?}", part);
    while workflow.is_some() {
        for p in workflow.unwrap().iter() {
            // println!("\nPredicate: {:?} , part: {:?} == {:?}", p, part, p.check(part));
            if let Some(go) = p.check(part) {
                if go == "A" {
                    // println!("====== ACCEPTED");
                    return part.sum();
                } else if go == "R" {
                    // println!("====== REJECTED");
                    return 0;
                } else {
                    // println!("====== MOVE ON TO {:?}", go);
                    workflow = workflows.get(go.as_str());
                    break;
                }
            }
        }
    }
    0
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct PartRange {
    lower: Part,
    upper: Part,
}

impl PartRange {
    fn sum(&self) -> usize {
        // println!("{:?}", self);
        let mut sum = 1;
        for i in 0..4 {
            sum *= (self.upper.xmas[i] - self.lower.xmas[i] + 1) as usize
        }
        sum
    }

    fn apply(&self, field: char, op: Op, number: u32) -> Option<(PartRange, PartRange)> {
        if field == 'A' {
            return Some((self.clone(), self.clone()));
        }
        let index = match field {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!()
        };
        // Will split a range into two.
        // Given a rule like [in{s<1351:px,qqz}], it means that
        // toe got to px we need to make sure that s is lower than 1351 => Set s to 1350.
        // But we also need to explore where we go if we fail that test, so, make sure
        // to also generate a range where s fails that test => Set s to 1351.
        match op {
            Op::GT =>
                if op.test(self.lower.xmas[index], number) {
                    Some((self.clone(), self.clone()))
                } else if self.lower.xmas[index] <= number && self.upper.xmas[index] >= number {
                    let mut pushed = self.clone();
                    let mut new_bounds = self.clone();
                    pushed.lower.xmas[index] = number + 1;
                    new_bounds.upper.xmas[index] = number;
                    // println!("Unr : {:?}", nr);
                    // println!("Unnr: {:?}", nnr);
                    Some((pushed, new_bounds))
                } else {
                    None
                }
            Op::LT =>
                if op.test(self.upper.xmas[index], number) {
                    Some((self.clone(), self.clone()))
                } else if self.lower.xmas[index] < number && self.upper.xmas[index] >= number {
                    let mut pushed = self.clone();
                    let mut new_bounds = self.clone();
                    pushed.upper.xmas[index] = number - 1;
                    new_bounds.lower.xmas[index] = number;
                    // println!("Lnr : {:?}", pushed);
                    // println!("Lnnr: {:?}", new_bounds);
                    Some((pushed, new_bounds))
                } else {
                    None
                }
        }
    }
}

fn possible_parts(workflows: &HashMap<String, Vec<Predicate>>) -> usize {
    let mut sum = 0;
    let mut unproccessd = Vec::new();
    unproccessd.push(
        (
            "in".to_string(),
            PartRange {
                lower: Part { xmas: [1; 4] },
                upper: Part { xmas: [4000; 4] },
            }
        )
    );
    while let Some((workflow, range)) = unproccessd.pop() {
        let workflow = workflow.as_str();
        match workflow {
            "A" => {
                // println!("Add: {:?} == {:?}", range, range.sum());
                sum += range.sum()
            }
            "R" => continue,
            &_ => {
                get_range_for_workflow(workflows.get(workflow).unwrap(), &range)
                    .into_iter()
                    .for_each(|r| {
                        // println!("{:?}", r);
                        unproccessd.push(r);
                    });
            }
        }
    }
    sum
}

fn get_range_for_workflow(predicates: &Vec<Predicate>, range: &PartRange) -> Vec<(String, PartRange)> {
    // println!("== {:?}", range);
    let mut ret = Vec::new();
    let mut r = range.clone();
    for predicate in predicates.iter() {
        // println!("rang1: {:?}", r);
        if let Some(nr) = r.apply(predicate.field, predicate.op, predicate.number) {
            ret.push((predicate.go.to_string(), nr.0));
            r = nr.1;
        } else {
            // println!("noooo nooo");
        }
    }
    // println!("== RETURNING ==");
    // for pr in ret.iter() {
    //     println!("{:?}", pr);
    // }
    // println!("== == == ==  ==");
    ret
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
        assert_eq!(492702, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day19/input.txt";
        assert_eq!(138616621185978, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day19/test-input.txt";
        let result = part_a(input);
        assert_eq!(19114, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day19/test-input.txt";
        let result = part_b(input);
        assert_eq!(167409079868000, result);
    }

    #[test]
    fn test_partrange_sum() {
        assert_eq!(256000000000000, PartRange {
            lower: Part::of(1, 1, 1, 1),
            upper: Part::of(4000, 4000, 4000, 4000),
        }.sum());

        assert_eq!(16, PartRange {
            lower: Part::of(3999, 3999, 3999, 3999),
            upper: Part::of(4000, 4000, 4000, 4000),
        }.sum());
        assert_eq!(16, PartRange {
            lower: Part::of(1, 1, 1, 1),
            upper: Part::of(2, 2, 2, 2),
        }.sum());
        assert_eq!(1, PartRange {
            lower: Part::of(1, 2, 1, 1),
            upper: Part::of(1, 2, 1, 1),
        }.sum());
        assert_eq!(2, PartRange {
            lower: Part::of(1, 2, 1, 1),
            upper: Part::of(2, 2, 1, 1),
        }.sum());
    }

    #[test]
    fn range_apply() {
        let start = PartRange {
            lower: Part { xmas: [1; 4] },
            upper: Part { xmas: [4000; 4] },
        };
        {
            let expected = Some((
                PartRange {
                    lower: Part::of(2001, 1, 1, 1),
                    upper: start.upper.clone(),
                },
                PartRange {
                    lower: start.lower.clone(),
                    upper: Part::of(2000, 4000, 4000, 4000),
                }
            ));
            let result = start.apply('x', Op::GT, 2000);
            assert_eq!(expected, result);
        }
    }
}