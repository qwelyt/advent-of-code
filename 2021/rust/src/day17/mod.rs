use crate::util::lines_from_file;

pub fn day17() {
    println!("== Day 17 ==");
    let input = lines_from_file("src/day17/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> i32 {
    let target = input.get(0)
        .unwrap()
        .split(": ")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split(", ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.split("=").collect::<Vec<&str>>())
        .map(|v| {
            let xy = v.get(0).unwrap().chars().collect::<Vec<char>>()[0];
            let range_vec: Vec<i32> = v.get(1).unwrap().split("..").collect::<Vec<&str>>().iter().map(|n| i32::from_str_radix(n, 10).unwrap()).collect();
            return (xy, (*range_vec.get(0).unwrap(), *range_vec.get(1).unwrap()));
        })
        .collect::<Vec<(char, (i32, i32))>>();
    println!("{:?}", target);

    let mut pos = (0, 0);
    let mut vel = (0, 0);
    let steps = i32::abs(target[1].1.0) * i32::abs(target[0].1.1);
    let mut y = 0;
    let mut velocities = Vec::new();
    for x in 0..target[0].1.1 {
        for y in 0..i32::abs(target[1].1.0) {
            velocities.push((x, y));
        }
    }
    for test_vel in velocities {
        pos = (0, 0);
        vel = test_vel;
        // println!("Testing: {:?}", vel);
        let mut ys = Vec::new();
        for step in 0..steps {
            pos.0 += vel.0;
            pos.1 += vel.1;
            vel.0 -= 1;
            vel.1 -= 1;
            if vel.0 < 0 {
                vel.0 = 0;
            }

            ys.push(pos.1);

            // println!("Pos: {:?} , in range: {}", pos, in_range(&pos, &target));
            if in_range(&pos, &target) {
                let by = *ys.iter().max().unwrap();
                if by > y { y = by }
            }
            if past_range(&pos, &target) {
                break;
            }
        }
    }

    y
}

fn past_range(pos: &(i32, i32), target: &Vec<(char, (i32, i32))>) -> bool {
    if pos.0 > target[0].1.1 { return true; }
    if pos.1 < target[1].1.0 { return true; }
    return false;
}

fn in_range(pos: &(i32, i32), target: &Vec<(char, (i32, i32))>) -> bool {
    let x = pos.0;
    let y = pos.1;
    let tx = target[0].1;
    let ty = target[1].1;

    x >= tx.0 && x <= tx.1 && y >= ty.0 && y <= ty.1
}

fn part_b(_input: &Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_range_t() {
        let target = vec![('x', (20, 30)), ('y', (-10, -5))];
        assert_eq!(true, in_range(&(21, -10), &target));
        assert_eq!(false, in_range(&(19, -10), &target));
        assert_eq!(false, in_range(&(20, -11), &target));
        assert_eq!(false, in_range(&(10, -11), &target));
        assert_eq!(true, in_range(&(30, -5), &target));
        assert_eq!(true, in_range(&(20, -5), &target));
    }

    #[test]
    fn part_a_test_input() {
        let filename = "src/day17/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(45, result)
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day17/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(35511, result)
    }
}
