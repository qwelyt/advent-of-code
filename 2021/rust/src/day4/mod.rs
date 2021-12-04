use crate::util::{lines_from_file, stringsp_to_i32};

pub fn day4() {
    println!("== Day 4 ==");
    let input = lines_from_file("src/day4/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

#[derive(PartialEq, Debug)]
struct Data {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

#[derive(PartialEq, Debug)]
struct Board {
    board: Vec<Vec<i32>>,
}

#[derive(PartialEq, Debug)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Debug)]
struct Positions {
    board_index: usize,
    positions: Vec<Position>,
}

fn read_data(input: &Vec<String>) -> Data {
    let x: Vec<&str> = input[0].split(",").collect();
    let numbers = stringsp_to_i32(x);

    let mut boards: Vec<Board> = Vec::new();
    let mut rows: Vec<&String> = Vec::new();

    for (index, line) in input.iter().enumerate() {
        if index < 2 {
            continue;
        }
        if line.trim().is_empty() {
            // println!("Build board from: {:?}", rows);
            boards.push(build_board(&rows));
            rows.clear();
        } else {
            rows.push(line);
        }
    }
    if !rows.is_empty() {
        boards.push(build_board(&rows));
        rows.clear();
    }

    return Data {
        numbers,
        boards,
    };
}

fn build_board(rows: &Vec<&String>) -> Board {
    let mut board: Vec<Vec<i32>> = Vec::new();
    // println!("{:?}", rows);

    for r in rows {
        let string = *r;
        let input = string.split(" ")
            .filter(|s| !s.is_empty())
            .collect();
        board.push(stringsp_to_i32(input));
    }

    Board { board }
}

fn part_a(input: &Vec<String>) -> i32 {
    let data = read_data(input);
    let mut board_hit: Vec<Board> = empty_boards(&data.boards);

    for n in data.numbers {
        for (index, board) in data.boards.iter().enumerate() {
            let position = find_position(board, n);
            if position.is_some() {
                let pos = position.unwrap();
                board_hit.get_mut(index)
                    .unwrap()
                    .board[pos.row][pos.col] = 1;
            }
        }
        let winner: Option<Positions> = check_winner(&board_hit);
        if winner.is_some() {
            let positions = winner.unwrap();
            let actual_board = data.boards.get(positions.board_index).unwrap();
            let marked_positions = board_hit.get(positions.board_index).unwrap();
            let sum_of_unmarked: i32 = sum_of_unmarked_positions(actual_board, marked_positions);
            return sum_of_unmarked * n;
        }
    }
    return 0;
}

fn part_b(input: &Vec<String>) -> i32 {
    let data = read_data(input);
    let mut board_hit: Vec<Board> = empty_boards(&data.boards);

    let mut winners: Vec<usize> = Vec::new();
    for n in data.numbers {
        for (index, board) in data.boards.iter().enumerate() {
            let position = find_position(board, n);
            if position.is_some() {
                let pos = position.unwrap();
                board_hit.get_mut(index)
                    .unwrap()
                    .board[pos.row][pos.col] = 1;
            }
        }
        let winner: Vec<Positions> = check_winners(&board_hit);
        for win in winner.iter() {
            if !winners.contains(&win.board_index) {
                winners.push(win.board_index);
            }
        }
        if winners.len() == data.boards.len() {
            let last_winner = winners.last().unwrap();
            // println!("winners: {:?}", winners);
            let actual_board = data.boards.get(*last_winner).unwrap();
            let marked_positions = board_hit.get(*last_winner).unwrap();
            let sum_of_unmarked: i32 = sum_of_unmarked_positions(actual_board, marked_positions);
            // println!("Sum of unmarked: {}",sum_of_unmarked);
            return sum_of_unmarked * n;
        }
    }

    return 0;
}


fn sum_of_unmarked_positions(actual_board: &Board, marked_positions: &Board) -> i32 {
    let mut sum: i32 = 0;
    for (r_index, row) in actual_board.board.iter().enumerate() {
        for (c_index, col) in row.iter().enumerate() {
            if *marked_positions.board.get(r_index).unwrap().get(c_index).unwrap() == 0 {
                sum += col;
            }
        }
    }
    sum
}

fn check_winners(boards: &Vec<Board>) -> Vec<Positions> {
    let mut winners = Vec::new();
    for (index, board) in boards.iter().enumerate() {
        let b = check_win(board);
        if b.is_some() {
            winners.push(Positions {
                board_index: index,
                positions: b.unwrap(),
            });
        }
    }
    winners
}

fn check_winner(boards: &Vec<Board>) -> Option<Positions> {
    for (index, board) in boards.iter().enumerate() {
        let b = check_win(board);
        if b.is_some() {
            return Some(Positions {
                board_index: index,
                positions: b.unwrap(),
            });
        }
    }
    None
}

fn check_win(board: &Board) -> Option<Vec<Position>> {
    let rows = board.board.len();
    let cols = board.board[0].len();
    for i in 0..rows {
        let sum: i32 = board.board[i].iter().sum();
        if sum as usize == cols {
            let mut pos: Vec<Position> = Vec::new();
            for c in 0..cols {
                pos.push(Position { row: i, col: c })
            }
            return Some(pos);
        }
    }
    for i in 0..cols {
        let mut c: Vec<i32> = Vec::new();
        for ii in 0..rows {
            c.push(board.board[ii][i]);
        }
        let sum: i32 = c.iter().sum();
        if sum as usize == rows {
            let mut pos: Vec<Position> = Vec::new();
            for c in 0..rows {
                pos.push(Position { row: c, col: i })
            }
            return Some(pos);
        }
    }
    None
}

fn empty_boards(boards: &Vec<Board>) -> Vec<Board> {
    boards.iter()
        .map(|b| board_of_size(b.board.len(), b.board.get(0).unwrap().len()))
        .collect()
}

fn board_of_size(rows: usize, cols: usize) -> Board {
    Board { board: vec![vec![0; cols]; rows] }
}

fn find_position(board: &Board, number: i32) -> Option<Position> {
    for (row_i, row) in board.board.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == number {
                return Some(Position { row: row_i, col: col_i });
            }
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_test_data() {
        let filename = "src/day4/test-input.txt";
        let input = lines_from_file(filename);
        let expected = Data {
            numbers: vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1],
            boards: vec![
                Board {
                    board: vec![
                        vec![22, 13, 17, 11, 0],
                        vec![8, 2, 23, 4, 24],
                        vec![21, 9, 14, 16, 7],
                        vec![6, 10, 3, 18, 5],
                        vec![1, 12, 20, 15, 19],
                    ]
                },
                Board {
                    board: vec![
                        vec![3, 15, 0, 2, 22],
                        vec![9, 18, 13, 17, 5],
                        vec![19, 8, 7, 25, 23],
                        vec![20, 11, 10, 24, 4],
                        vec![14, 21, 16, 12, 6],
                    ]
                },
                Board {
                    board: vec![
                        vec![14, 21, 17, 24, 4],
                        vec![10, 16, 15, 9, 19],
                        vec![18, 8, 23, 26, 20],
                        vec![22, 11, 13, 6, 5],
                        vec![2, 0, 12, 3, 7],
                    ]
                },
            ],
        };
        let data = read_data(&input);
        assert_eq!(expected, data);
    }

    #[test]
    fn part_a_test_input() {
        let filename = "src/day4/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(4512, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day4/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(4512, result);
    }

    #[test]
    fn part_b_test_input() {
        let filename = "src/day4/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(1924, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day4/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(7075, result);
    }
}