use crate::util::lines_from_file;

pub fn day20() {
    println!("== Day 20 ==");
    let input = lines_from_file("src/day20/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> usize {
    let algorithm = input.get(0).unwrap().chars().collect::<Vec<char>>();
    let image: Vec<Vec<char>> = input.iter()
        .skip(2)
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    // println!("{:?}", algorithm);
    // println!("{:?}", image);

    print_image(&image);
    println!();

    let enhanced_1: Vec<Vec<char>> = enhance(&algorithm, &image);
    print_image(&enhanced_1);
    println!();
    let enhanced_2: Vec<Vec<char>> = enhance(&algorithm, &enhanced_1);
    print_image(&enhanced_2);


    enhanced_2.iter()
        .flatten()
        .filter(|c| **c == '#')
        .count()
}

fn print_image(image: &Vec<Vec<char>>) {
    for row in image.iter() {
        let string = row.iter().collect::<String>();
        println!("{}", string);
    }
}

fn enhance(algorithm: &Vec<char>, image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rd = [-1, -1, -1, 0, 0, 0, 1, 1, 1];
    let cd = [-1, 0, 1, -1, 0, 1, -1, 0, 1];
    let rows = image.len();
    let cols = image.get(0).unwrap().len();
    let scan_rows = (rows + 2) as i32;
    let scan_cols = (cols + 2) as i32;
    // println!("({},{}) :: ({},{})", rows, cols, scan_rows,scan_cols);
    let mut new_image_info = Vec::new();
    for r_index in -2..scan_rows {
        // In input image
        let mut new_row = Vec::new();
        for c_index in -2..scan_cols {
            let mut pixels = Vec::new();
            for i in 0..rd.len() {
                let rr = r_index as i32 + rd[i];
                let cc = c_index as i32 + cd[i];
                if rr < 0 || cc < 0 || rr >= rows as i32 || cc >= cols as i32 {
                    pixels.push('.');
                } else {
                    pixels.push(image[rr as usize][cc as usize]);
                }
            }
            let pixel_string = pixels.iter()
                .map(|c| {
                    match c {
                        '.' => '0',
                        '#' => '1',
                        _ => unreachable!()
                    }
                }).collect::<String>();
            let result = u32::from_str_radix(pixel_string.as_str(), 2).unwrap();
            new_row.push(result);
        }
        new_image_info.push(new_row);
    }
    for r in new_image_info.iter() {
        println!("{:?}", r);
    }

    // println!("{:?}", new_image_info);
    let vec = new_image_info.iter()
        .map(|r| r.iter()
            .map(|c| *algorithm.get(*c as usize).unwrap())
            .collect::<Vec<char>>()
        ).collect::<Vec<Vec<char>>>();

    // println!("{:?}", vec);


    vec
}

fn part_b(input: &Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test_input() {
        let filename = "src/day20/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(35, result)
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day20/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(true, 5413 > result);
        assert_eq!(1, result);
    }
}
