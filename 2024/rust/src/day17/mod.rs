use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::util::time;

pub fn solve() {
    println!("== Day 17 ==");
    let input = "src/day17/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> String {
    let mut computer = Computer::parse(input);
    // println!("{:?}", computer);
    computer.runs();
    // println!("{:?}", computer);
    computer.output()
}

fn part_b(input: &str) -> usize {
    let orig_computer = Computer::parse(input);
    // let mut i = 0; /// Will _never_ work, waaaay to big of a number
    // loop {
    //     let mut computer = orig_computer.clone();
    //     computer.register_a = i;
    //     computer.runs();
    //     if computer.output == computer.instructions {
    //         println!("{:?}",computer);
    //         return i
    //     }
    //     i += 1;
    // }

    let i = find(&orig_computer.instructions, 0).unwrap();
    let mut computer = orig_computer.clone();
    computer.register_a = i;
    computer.runs();
    if computer.output == orig_computer.instructions {
        i
    } else {
        println!("Ins: {:?}", orig_computer.instructions);
        println!("Out: {:?}", computer.output);
        panic!("Invalid instruction")
    }
}

fn find(program: &Vec<usize>, ans: usize) -> Option<usize> {
    /* Program: 2,4, 1,1, 7,5, 4,7, 1,4, 0,3, 5,5, 3,0
        (init)a = ???
        (2,4) b = combo(4) % 8 = a % 8
        (1,1) b = b ^ 1
        (7,5) c = a >> combo(5) = a >> b
        (4,7) b = b ^ c
        (1,4) b = b ^ 4
        (0,3) a = a >> combo(3) = a >> 3
        (5,5) out += combo(5) % 8  = b % 8
        (3,0) if a != 0 loop
     */
    if program.len() == 0 { return Some(ans); }
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for i in 0..8 {
        a = ans << 3 | i;
        b = 0;
        c = 0;

        b = a % 8;
        b = b ^ 1;
        c = a >> b;
        b = b ^ c;
        b = b ^ 4;
        // a = a >> 3;
        // println!("b={} c={} a={} last={:?}, ans={}", b, c, a, program.last(), ans);
        if (b % 8) == *program.last().unwrap() as usize {
            // println!("i: {:?}, a:{} {:?}, ans= {}", i, a, program, ans);
            let part = find(&program[..program.len() - 1].to_vec(), a);
            if part.is_some() {
                return part;
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    instructions: Vec<usize>,
    instruction_pointer: usize,
    output: Vec<usize>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let mut registers = Vec::new();
        let mut instructions = None;
        let mut r = true;
        for line in File::open(input).map(|f| BufReader::new(f).lines().flatten()).unwrap() {
            if line.is_empty(){
                r=false;
                continue;
            }
            let x = line.split_once(": ").unwrap().1;
            if r {
                registers.push(x.parse::<usize>().unwrap());
            } else {
                instructions = Some(x.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>());
            }
        }
        Self {
            register_a: registers[0],
            register_b: registers[1],
            register_c: registers[2],
            instructions: instructions.unwrap(),
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    fn runs(&mut self)  {
        while self.instruction_pointer < self.instructions.len() {
            // println!("{:?}", self);
            let opcode = self.instructions[self.instruction_pointer];
            let operand = self.instructions[self.instruction_pointer + 1];

            match opcode {
                0 => self.register_a = self.register_a >> self.combo_operand(operand),
                1 => self.register_b = self.register_b ^ operand,
                2 => self.register_b = self.combo_operand(operand) % 8,
                3 => if self.register_a != 0 {
                    self.instruction_pointer = operand as usize;
                    continue;
                },
                4 => self.register_b = self.register_b ^ self.register_c,
                5 => self.output.push(self.combo_operand(operand) % 8),
                6 => self.register_b = self.register_a >> self.combo_operand(operand),
                7 => self.register_c = self.register_a >> self.combo_operand(operand),
                _ => {}
            }
            self.instruction_pointer += 2;
        }
    }

    fn combo_operand(&self, operand: usize) -> usize {
        match operand {
            0|1|2|3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Unknown combo operand: {}", operand),
        }
    }
    fn output(&self) -> String{
        self.output.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn run_day() {
        solve();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day17/input.txt";
        assert_eq!("1,3,7,4,6,4,2,3,5", part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day17/input.txt";
        assert_eq!(202367025818154, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day17/test-input.txt";
        let result = part_a(input);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day17/test-input.txt";
        let result = part_b(input);
        assert_eq!(117440, result);
    }
}