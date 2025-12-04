use crate::util::time;

pub fn day20() {
    println!("== Day 20 ==");
    let input = "src/day20/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Debug)]
struct Message {
    msg: Vec<i32>,
}

impl Message {
    fn parse(input: &str) -> Self {
        let mut msg = Vec::new();
        for line in input.lines() {
            msg.push(line.parse::<i32>().unwrap());
        }
        Self { msg }
    }

    fn decrypt(&self) -> (i32, Vec<i32>) {
        let mut msg = self.msg.iter()
            .enumerate()
            .map(|(idx, v)| (idx, *v))
            .collect::<Vec<(usize, i32)>>();

        for i in 0..self.msg.len() {
            let (current_index, &original) = msg.iter().enumerate().find(|(_idx, (oidx, _v))| *oidx == i).unwrap();
            let new_index = (current_index as i32 + original.1).rem_euclid(msg.len() as i32 - 1) as usize;
            msg.remove(current_index);
            msg.insert(new_index, original);
        }

        let out_msg = msg.iter().map(|(_i, v)| *v).collect::<Vec<i32>>();
        let zero = out_msg.iter().position(|i| *i == 0).unwrap();
        let sum = (1..=3).map(|i| i * 1000)
            .map(|i| zero + i)
            .map(|i| i % out_msg.len())
            .map(|i| out_msg[i])
            .sum();

        (sum, out_msg)
    }

    fn decrypt_with_key(&self, key: isize, times: u32) -> (isize, Vec<isize>) {
        let mut msg = self.msg.iter()
            .enumerate()
            .map(|(idx, v)| (idx, *v as isize * key))
            .collect::<Vec<(usize, isize)>>();

        for _ in 0..times {
            for i in 0..self.msg.len() {
                let (current_index, &original) = msg.iter().enumerate().find(|(_idx, (oidx, _v))| *oidx == i).unwrap();
                let new_index = (current_index as isize + original.1).rem_euclid(msg.len() as isize - 1) as usize;
                msg.remove(current_index);
                msg.insert(new_index, original);
            }
        }

        let out_msg = msg.iter().map(|(_i, v)| *v).collect::<Vec<isize>>();
        let zero = out_msg.iter().position(|i| *i == 0).unwrap();
        let sum = (1..=3).map(|i| i * 1000)
            .map(|i| zero + i)
            .map(|i| i % out_msg.len())
            .map(|i| out_msg[i])
            .sum();

        (sum, out_msg)
    }
}

fn part_a(input: &str) -> i32 {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let message = Message::parse(open.as_str());
    // println!("{:?}", message);
    let (value, _msg) = message.decrypt();
    // println!("{} :: {:?}", value, msg);
    value
}

fn part_b(input: &str) -> isize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let message = Message::parse(open.as_str());
    // println!("{:?}", message);
    let (value, _msg) = message.decrypt_with_key(811589153, 10);
    // println!("{} :: {:?}", value, msg);
    value
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
        let result = part_a(input);
        assert_eq!(9945, result);
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day20/input.txt";
        assert_eq!(3338877775442, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day20/test-input.txt";
        let result = part_a(input);
        assert_eq!(3, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day20/test-input.txt";
        let result = part_b(input);
        assert_eq!(1623178306, result);
    }
}