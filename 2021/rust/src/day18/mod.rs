use crate::util::lines_from_file;

pub fn day18() {
    println!("== Day 18 ==");
    let input = lines_from_file("src/day18/input.txt");
    let (a, _) = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> (u32, Pair) {
    let mut pairs = input.iter()
        .map(|s| to_pair(s.as_str()))
        .collect::<Vec<Pair>>();
    // println!("{:?}",pairs);

    let start = pairs.get(0).unwrap().clone();
    pairs.drain(0..1);
    let sum = pairs.iter().fold(start, |acc, next| addition(&acc, next));
    // println!("{:?}", sum);
    (sum.magnitude(), sum)
}

fn part_b(input: &Vec<String>) -> u32 {
    let pairs = input.iter()
        .map(|s| to_pair(s.as_str()))
        .collect::<Vec<Pair>>();

    let mut premutations: Vec<(&Pair, &Pair)> = Vec::new();
    for i in 0..pairs.len() {
        for j in 0..pairs.len() {
            if i == j {
                continue;
            }
            let a = pairs.get(i).unwrap();
            let b = pairs.get(j).unwrap();
            premutations.push((a, b));
        }
    }
    let max = premutations.iter()
        .map(|(a, b)| addition(*a, *b))
        .map(|sum| sum.magnitude())
        .max()
        .unwrap();
    max
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Pair {
    Leaf(u32),
    Branch { left: Box<Pair>, right: Box<Pair> },
}

impl Pair {
    fn magnitude(&self) -> u32 {
        match self {
            Pair::Leaf(value) => *value,
            Pair::Branch { left: l, right: r } => l.magnitude() * 3 + r.magnitude() * 2
        }
    }
}

enum Direction {
    Left,
    Right,
}

impl Pair {
    fn reduce(&mut self, depth: u32) -> Option<(u32, u32)> {
        if let Pair::Leaf(_) = self {
            return None;
        }
        if let Pair::Branch { left: l, right: r } = self {
            if depth == 4 {
                // KABOOM (explode)
                let a = match **l {
                    Pair::Leaf(num) => num,
                    _ => unreachable!()
                };
                let b = match **r {
                    Pair::Leaf(num) => num,
                    _ => unreachable!()
                };
                *self = Pair::Leaf(0);
                return Some((a, b));
            } else {
                if let Some((a, b)) = l.reduce(depth + 1) {
                    r.eat(&Direction::Left, b);
                    return Some((a, 0));
                }
                if let Some((a, b)) = r.reduce(depth + 1) {
                    l.eat(&Direction::Right, a);
                    return Some((0, b));
                }
            }
        }
        None
    }

    fn eat(&mut self, from: &Direction, value: u32) {
        match self {
            Pair::Leaf(current) => { *current += value }
            Pair::Branch { left: l, right: r } => match from {
                Direction::Left => l.eat(from, value),
                Direction::Right => r.eat(from, value)
            }
        }
    }

    fn split(&mut self) -> Option<()> {
        match self {
            Pair::Leaf(value) => {
                if *value > 9 {
                    let half: f32 = *value as f32 / 2.0;
                    let left: u32 = f32::floor(half) as u32;
                    let right: u32 = f32::ceil(half) as u32;
                    *self = Pair::Branch {
                        left: Box::new(Pair::Leaf(left)),
                        right: Box::new(Pair::Leaf(right)),
                    };
                    return Some(());
                }
                None
            }
            Pair::Branch { left: l, right: r } => {
                if let Some(_) = l.split() {
                    return Some(());
                }
                if let Some(_) = r.split() {
                    return Some(());
                }
                None
            }
        }
    }
}

fn to_pair(input: &str) -> Pair {
    if let Ok(number) = input.parse::<u32>() {
        Pair::Leaf(number)
    } else {
        let mut depth = 0;
        let mut split = 0;
        for (i, c) in input.chars().enumerate() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' if depth == 1 => split = i,
                _ => {},
            }
        }
        Pair::Branch {
            left: Box::new(to_pair(&input[1..split]).to_owned()),
            right: Box::new(to_pair(&input[(split + 1)..(input.len() - 1)]).to_owned()),
        }
    }
}

fn addition(a: &Pair, b: &Pair) -> Pair {
    let mut sum = Pair::Branch {
        left: Box::new(a.clone()),
        right: Box::new(b.clone()),
    };
    // loop {
    //     // Reduce and split until nothing happens
    //     if sum.reduce(0).is_none(){break;}
    //     if sum.split().is_none(){break}
    // }
    while sum.reduce(0).is_some() || sum.split().is_some() {}
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day18/test-input.txt";
        let input = lines_from_file(filename);
        let (result, _sum) = part_a(&input);
        // println!("{:?}", _sum);
        assert_eq!(4140, result)
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day18/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(4116, result.0)
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day18/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        // println!("{:?}", _sum);
        assert_eq!(3993, result)
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day18/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(4638, result)
    }
}
