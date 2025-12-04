use std::cmp::{max, Ordering};

use crate::util::{lines, time, vecs};

pub fn day13() {
    println!("== Day 13 ==");
    let input = "src/day13/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let pairs = vecs(&lines(input));
    pairs.iter().enumerate()
        .map(|(index, pair)| (index, correct_order(pair)))
        .filter(|(_index, valid)| {
            // println!("{}: {}", _index, valid.0);
            // println!("{:?}", valid.1);
            // println!("{:?}", valid.2);
            valid.0 == true
        })
        .map(|(index, _valid)| index + 1)
        .sum()
}

fn part_b(input: &str) -> usize {
    let lines = lines(input);
    let pairs = sorted_with_divider(lines);
    let first = pairs.iter().position(|s| s == "[[2]]").unwrap() + 1;
    let second = pairs.iter().position(|s| s == "[[6]]").unwrap() + 1;
    first * second
}

fn sorted_with_divider(lines: Vec<String>) -> Vec<String> {
    let mut pairs = lines.iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();
    pairs.push("[[2]]");
    pairs.push("[[6]]");

    pairs.sort_by(|a, b| {
        let order = correct_order(&vec![a.to_string(), b.to_string()]);
        if order.0 {
            return Ordering::Less;
        }
        Ordering::Greater
    });
    pairs.iter().map(|s| s.to_string()).collect()
}


fn correct_order(pair: &Vec<String>) -> (bool, Vec<String>, Vec<String>) {
    let orig_lef = split(pair.get(0).unwrap());
    let orig_right = split(pair.get(1).unwrap());
    let mut left = orig_lef.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let mut right = orig_right.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let mut exp_l = 0;
    let mut exp_r = 0;
    // println!(" == ");
    for i in 0..max(left.len(), right.len()) {
        let l = left.get(i);
        let r = right.get(i);
        // println!(" = {:?} :: {:?} = ", l, r);
        if l.is_some() && r.is_none() {
            return (false,
                    left.iter().map(|s| s.to_string()).collect(),
                    right.iter().map(|s| s.to_string()).collect(),
            );
        } else if l.is_none() && r.is_some() {
            return (true,
                    left.iter().map(|s| s.to_string()).collect(),
                    right.iter().map(|s| s.to_string()).collect(),
            );
        }

        let l = *l.unwrap();
        let r = *r.unwrap();
        if l == "]" && r != "]" {
            return (true,
                    left.iter().map(|s| s.to_string()).collect(),
                    right.iter().map(|s| s.to_string()).collect(),
            );
        } else if l != "]" && r == "]" {
            return (false,
                    left.iter().map(|s| s.to_string()).collect(),
                    right.iter().map(|s| s.to_string()).collect(),
            );
        }
        if l == "[" && r != "[" {
            right.insert(i, "[");
            exp_r += 1;
        } else if l != "[" && r == "[" {
            left.insert(i, "[");
            exp_l += 1;
        }
        // println!("L: {} | {} |{}||  {:?}", l, left[i], exp_l, left.concat());
        // println!("R: {} | {} |{}||  {:?}", r, right[i], exp_r, right.concat());
        // println!();
        let ln = l.parse::<i32>().is_ok();
        let rn = r.parse::<i32>().is_ok();
        if ln && rn {
            let ld = l.parse::<i32>().unwrap();
            let rd = r.parse::<i32>().unwrap();
            if ld == rd {
                continue;
            }
            return (ld < rd,
                    left.iter().map(|s| s.to_string()).collect(),
                    right.iter().map(|s| s.to_string()).collect(),
            );
        } else if ln && (r == "," || r == "]") {
            return (false,
                    left.iter().map(|s| s.to_string()).collect(),
                    right.iter().map(|s| s.to_string()).collect(),
            );
        } else if rn && (l == "," || l == "]") {
            return (true,
                    left.iter().map(|s| s.to_string()).collect(),
                    right.iter().map(|s| s.to_string()).collect(),
            );
        }
        if ln || l == "]" {
            while exp_l > 0 {
                left.insert(i + 2, "]");
                exp_l -= 1;
            }
        }
        if rn || r == "]" {
            while exp_r > 0 {
                right.insert(i + 2, "]");
                exp_r -= 1;
            }
        }
    }
    // println!("{:?}", left);
    // println!("{:?}", right);

    return (true,
            left.iter().map(|s| s.to_string()).collect(),
            right.iter().map(|s| s.to_string()).collect(),
    );
}

fn split(line: &String) -> Vec<String> {
    let mut s = Vec::new();
    let chars = line.chars().collect::<Vec<char>>();
    let mut skip = false;
    for i in 0..chars.len() {
        if skip {
            skip = false;
            continue;
        }
        if chars[i].is_digit(10) && chars[i + 1].is_digit(10) {
            let string = chars[i].to_string() + chars[i + 1].to_string().as_str();
            s.push(string);
            skip = true;
        } else {
            s.push(chars[i].to_string());
        }
    }
    s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day13();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day13/input.txt";
        assert_eq!(6086, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day13/input.txt";
        assert_eq!(27930, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day13/test-input.txt";
        let result = part_a(input);
        assert_eq!(13, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day13/test-input.txt";
        let result = part_b(input);
        assert_eq!(140, result);
    }

    #[test]
    fn sorted_with_divider_test() {
        let input = "src/day13/test-input.txt";
        let lines = lines(input);
        let expected = vec![
            "[]",
            "[[]]",
            "[[[]]]",
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "[[1],[2,3,4]]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[[1],4]",
            "[[2]]",
            "[3]",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "[[6]]",
            "[7,7,7]",
            "[7,7,7,7]",
            "[[8,7,6]]",
            "[9]",
        ];
        assert_eq!(expected, sorted_with_divider(lines));
    }

    #[test]
    fn correct_test() {
        {
            let input = vec![
                "[[[]]]".to_string(),
                "[[]]".to_string(),
            ];
            assert_eq!(false, correct_order(&input).0);
        }
        {
            let input = vec![
                "[[],[[[7,0]]]]".to_string(),
                "[[[],[[1,4,6],10,6],[6,6],1],[0],[[[1]],1,[[2,4],1,2],0],[],[[[9,5],[],3,[10,3,10,9],7],[9],4,[5]]]".to_string(),
            ];
            assert_eq!(true, correct_order(&input).0);
        }
        {
            let input = vec![
                "[[[10,1,0],[[9]]],[[[6],3,[],7],[[0],6,[8,9]]],[[3,[3]],[[2,5,8,10],7,[],[0,5,0,6]],[],[[2,2,1],[3,7],[],[8,9,1]],3],[],[2,1,[[10,3,1],2]]]".to_string(),
                "[[10,[[5,10,6]],4],[[10,6],9,2],[6,[[7,1,1],[6,0,6,10],[]]],[1,6,3,[],[[6,6,1],1]]]".to_string(),
            ];
            assert_eq!(false, correct_order(&input).0);
        }
        {
            let input = vec![
                "[[[],3]]".to_string(),
                "[[],[4],[]]".to_string(),
            ];
            assert_eq!(false, correct_order(&input).0);
        }
    }

    #[test]
    fn split_test() {
        let str = "[[[10,1,0],[[9]]]".to_string();
        let expected = vec![
            "[", "[", "[", "10", ",", "1", ",", "0", "]", ",",
            "[", "[", "9", "]", "]", "]",
        ];
        let result = split(&str);
        assert_eq!(expected, result)
    }
}