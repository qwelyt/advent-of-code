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
    solve(input, 2)
}

fn part_b(input: &Vec<String>) -> usize {
    solve(input, 50)
}

fn solve(input: &Vec<String>, iterations: u32) -> usize {
    let algorithm = input.get(0).unwrap().chars().collect::<Vec<char>>();
    let image: Vec<Vec<char>> = input.iter()
        .skip(2)
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    // println!("{:?}", algorithm);
    // println!("{:?}", image);

    // print_image(&image);
    // println!();

    let mut img = image.clone();
    for i in 0..iterations {
        // println!("{}    {}", i, i % 2);
        img = enhance(&algorithm, &img, i);
        // print_image(&img);
        // println!();
    }

    img.iter()
        .flatten()
        .filter(|c| **c == '#')
        .count()
}

#[allow(dead_code)]
fn print_image(image: &Vec<Vec<char>>) {
    for row in image.iter() {
        let string = row.iter().collect::<String>();
        println!("{}", string);
    }
}

fn enhance(algorithm: &Vec<char>, image: &Vec<Vec<char>>, iteration: u32) -> Vec<Vec<char>> {
    let rd = [-1, -1, -1, 0, 0, 0, 1, 1, 1];
    let cd = [-1, 0, 1, -1, 0, 1, -1, 0, 1];
    let rows = image.len();
    let cols = image.get(0).unwrap().len();
    let border = 1;
    let scan_rows = (rows + border) as i32;
    let scan_cols = (cols + border) as i32;
    let scan_start = border as i32 * -1;
    // println!("({},{}) :: ({},{})", rows, cols, scan_rows,scan_cols);
    let mut new_image_info = Vec::new();
    for r_index in scan_start..scan_rows {
        // In input image
        let mut new_row = Vec::new();
        for c_index in scan_start..scan_cols {
            let mut pixels = Vec::new();
            for i in 0..rd.len() {
                let rr = r_index as i32 + rd[i];
                let cc = c_index as i32 + cd[i];
                if rr < 0 || cc < 0 || rr >= rows as i32 || cc >= cols as i32 {
                    // So a ['.'; 9] maps to algorithm[0], what does [algorithm[0]; 9] produce
                    // on the next iteration?
                    // The world flickers for each step. The test data does not show this as
                    // algorithm[0] in that case is a '.', but our data that is '#' which means
                    // the next iteration we have 512 as the read value (happens to be a '.' in our
                    // case, but a '#' in the test data)
                    if iteration % 2 == 0 {
                        pixels.push('.');
                    } else {
                        pixels.push(algorithm[0]);
                    }
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
    // for r in new_image_info.iter() {
    //     println!("{:?}", r);
    // }

    // println!("{:?}", new_image_info);
    let vec = new_image_info.iter()
        .map(|r| r.iter()
            .map(|c| *algorithm.get(*c as usize).unwrap())
            .collect::<Vec<char>>()
        ).collect::<Vec<Vec<char>>>();

    // println!("{:?}", vec);


    vec
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
        assert_ne!(5352, result);
        assert_eq!(5400, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day20/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(3351, result)
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day20/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(18989, result);
    }
}
