use crate::util::{lines, time};

pub fn day10() {
    println!("== Day 10 ==");
    let input = "src/day10/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> isize {
    let (_, signal_strength, _) = run_program(input);
    signal_strength
}

fn part_b(input: &str) -> String {
    let (_, _, crt) = run_program(input);
    let mut lines = Vec::new();
    lines.push("".to_string());
    for i in 0..6 {
        let x = crt.get(i).unwrap();
        lines.push(x.to_string());
    }
    lines.push("".to_string());
    lines.join("\r\n")
}


fn run_program(input: &str) -> (i32, isize, Vec<String>) {
    let mut x: i32 = 1;
    let mut amount_to_add = 0;
    let mut signal_strength: isize = 0;
    let mut do_cycle = 0;
    let mut crt_lines: Vec<String> = Vec::new();
    let instructions = lines(input);
    // println!("{:?}", instructions);
    let mut instruction_i = 0;
    for i in 1..300 {
        let pixel_pos = (i - 1) % 40; // Cycle 1 draws at pixel 0
        // println!("== Cycle {} == ", i);
        // println!("Cycle start: x = {}, ss = {}, ata = {}, dc = {}, pp = {}", x, signal_strength, amount_to_add, do_cycle, pixel_pos);
        if vec![20, 60, 100, 140, 180, 220].contains(&i) {
            signal_strength += i as isize * x as isize;
            // println!("ss = {}", signal_strength);
        }
        if vec![x - 1, x, x + 1].contains(&pixel_pos) {
            crt_lines.push("#".to_string());
        } else {
            crt_lines.push(".".to_string());
        }

        if do_cycle == 0 {
            let option = instructions.get(instruction_i);
            if option.is_none() {
                break;
            }
            // println!("{:?}", option);
            let instruction = option.unwrap();
            instruction_i += 1;
            let ins = instruction.split(" ").collect::<Vec<&str>>();
            if ins[0].eq("addx") {
                amount_to_add = ins[1].parse::<i32>().unwrap();
                // println!("ata = {}", amount_to_add);
                do_cycle = 1;
            }
        } else {
            do_cycle -= 1;
            x += amount_to_add;
            // println!("x = {}", x);
            amount_to_add = 0;
        }

        // println!("Cycle end: x = {}, ss = {}, ata = {}, dc = {}, pp = {}", x, signal_strength, amount_to_add, do_cycle, pixel_pos);
        // println!();
    }
    let mut crt = Vec::new();
    for line in crt_lines.chunks(40) {
        crt.push(line.concat());
    }

    // println!("{:?}", crt_lines);
    (x, signal_strength, crt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day10();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day10/input.txt";
        assert_eq!(13440, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day10/input.txt";
        let expected = vec!["",
                            "###..###..####..##..###...##..####..##..",
                            "#..#.#..#....#.#..#.#..#.#..#....#.#..#.",
                            "#..#.###....#..#....#..#.#..#...#..#..#.",
                            "###..#..#..#...#.##.###..####..#...####.",
                            "#....#..#.#....#..#.#.#..#..#.#....#..#.",
                            "#....###..####..###.#..#.#..#.####.#..#.",
                            ""].join("\r\n"); // PBZGRAZA

        let result = part_b(input);
        // for line in result.iter() {
        //     println!("{}", line);
        // }
        // println!();
        assert_eq!(expected, result);
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day10/test-input.txt";
        let result = run_program(input);
        assert_eq!(-1, result.0);
    }

    #[test]
    fn part_a_test_input2() {
        let input = "src/day10/test-input2.txt";
        let result = run_program(input);
        assert_eq!(17, result.0);
        assert_eq!(13140, result.1);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day10/test-input2.txt";
        let result = part_b(input);
        // println!("{:?}", result);
        let expected = vec!["",
                            "##..##..##..##..##..##..##..##..##..##..",
                            "###...###...###...###...###...###...###.",
                            "####....####....####....####....####....",
                            "#####.....#####.....#####.....#####.....",
                            "######......######......######......####",
                            "#######.......#######.......#######.....",
                            ""].join("\r\n");
        assert_eq!(expected, result);
    }
}