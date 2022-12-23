use std::cmp::max;

use crate::util::time;

pub fn day22() {
    println!("== Day 22 ==");
    let input = "src/day22/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Open,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            ' ' => Tile::Empty,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => panic!("Can't turn {} into a tile", c)
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Rotate {
    Clockwise,
    CounterClockwise,
}

impl From<char> for Rotate {
    fn from(c: char) -> Self {
        match c {
            'R' => Rotate::Clockwise,
            'L' => Rotate::CounterClockwise,
            _ => panic!("Can't turn {} into a rotation", c)
        }
    }
}


#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn apply(&self, rotate: &Rotate) -> Direction {
        match self {
            Direction::Up => match rotate {
                Rotate::Clockwise => Direction::Right,
                Rotate::CounterClockwise => Direction::Left,
            }
            Direction::Right => match rotate {
                Rotate::Clockwise => Direction::Down,
                Rotate::CounterClockwise => Direction::Up,
            }
            Direction::Down => match rotate {
                Rotate::Clockwise => Direction::Left,
                Rotate::CounterClockwise => Direction::Right,
            }
            Direction::Left => match rotate {
                Rotate::Clockwise => Direction::Up,
                Rotate::CounterClockwise => Direction::Down,
            }
        }
    }

    fn value(&self) -> isize {
        match self {
            Direction::Up => 3,
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Move {
    Rotate(Rotate),
    Move(u32),
}

impl Move {
    fn parse(input: &str) -> Vec<Move> {
        let mut mvs = Vec::new();
        let mut crs = Vec::new();
        for c in input.chars() {
            if c.is_digit(10) {
                crs.push(c);
            }
            if c == 'R' || c == 'L' {
                let x: String = crs.iter().collect();
                let amount = x.parse::<u32>().unwrap();
                mvs.push(Move::Move(amount));
                crs.clear();

                mvs.push(Move::Rotate(Rotate::from(c)));
            }
        }
        if !crs.is_empty() {
            let x: String = crs.iter().collect();
            let amount = x.parse::<u32>().unwrap();
            mvs.push(Move::Move(amount));
            crs.clear();
        }
        mvs
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
    facing: Direction,
}

impl From<(isize, isize, Direction)> for Position {
    fn from(p: (isize, isize, Direction)) -> Self {
        Self {
            x: p.0,
            y: p.1,
            facing: p.2,
        }
    }
}

impl Position {
    fn apply(&self, mv: &Move, board: &Board) -> Position {
        match mv {
            Move::Rotate(rot) => self.rotate(rot),
            Move::Move(amount) => self.mv(*amount as isize, board),
        }
    }
    fn apply_folded(&self, mv: &Move, cube: &Cube) -> Position {
        match mv {
            Move::Rotate(rot) => self.rotate(rot),
            Move::Move(amount) => self.mv_folded(*amount as isize, cube),
        }
    }

    fn rotate(&self, rotation: &Rotate) -> Position {
        Self {
            x: self.x,
            y: self.y,
            facing: self.facing.apply(rotation),
        }
    }

    fn mv(&self, amount: isize, board: &Board) -> Position {
        let (x, y) = match self.facing {
            Direction::Up => {
                let mut y = self.y;
                for _ in 0..amount {
                    y = board.dec_y(self.x, y);
                }
                (self.x, y)
            }
            Direction::Right => {
                let mut x = self.x;
                for _ in 0..amount {
                    x = board.inc_x(x, self.y);
                }
                (x, self.y)
            }
            Direction::Down => {
                let mut y = self.y;
                for _ in 0..amount {
                    y = board.inc_y(self.x, y);
                }
                (self.x, y)
            }
            Direction::Left => {
                let mut x = self.x;
                for _ in 0..amount {
                    x = board.dec_x(x, self.y);
                }
                (x, self.y)
            }
        };
        Self {
            x,
            y,
            facing: self.facing,
        }
    }

    fn mv_folded(&self, amount: isize, cube: &Cube) -> Position {
        let mut pos = *self;
        for _ in 0..amount {
            pos = cube.mv(&pos);
        }
        pos
    }
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<Tile>>,
}

impl Board {
    fn new(rows: &Vec<Vec<Tile>>, width: usize, height: usize) -> Self {
        let mut r = vec![vec![Tile::Empty; width]; height];
        for y in 0..rows.len() {
            let row = rows.get(y).unwrap();
            for x in 0..row.len() {
                r[y][x] = *row.get(x).unwrap();
            }
        }

        Self {
            rows: r
        }
    }
    fn top_left(&self) -> usize {
        self.rows[0].iter().position(|p| *p == Tile::Open).unwrap()
    }

    fn dec_y(&self, x: isize, y: isize) -> isize {
        let x = x as usize;
        // if (y == 0 && self.rows[self.height Vjj- 1][x] == Tile::Wall) || self.rows[y - 1][x] == Tile::Wall {
        //     return y as isize;
        // }
        let ny = (y - 1).rem_euclid(self.rows.len() as isize) as usize;
        let y = y as usize;
        let ry = match self.rows[ny][x] {
            Tile::Empty => {
                let mut yy = self.rows.len() - 1;
                while self.rows[yy][x] == Tile::Empty {
                    yy -= 1;
                }
                if self.rows[yy][x] == Tile::Open {
                    yy
                } else {
                    y
                }
            }
            Tile::Wall => y,
            Tile::Open => ny,
        };
        ry as isize
    }

    fn inc_y(&self, x: isize, y: isize) -> isize {
        let x = x as usize;
        let y = y as usize;
        // if y == self.rows.len()-1 && self.rows[0][x] == Tile::Wall {
        //     return y as isize;
        // }
        let ny = (y + 1).rem_euclid(self.rows.len());
        let ry = match self.rows[ny][x] {
            Tile::Empty => {
                let mut yy = 0;
                while self.rows[yy][x] == Tile::Empty {
                    yy += 1;
                }
                if self.rows[yy][x] == Tile::Open {
                    yy
                } else {
                    y
                }
            }
            Tile::Wall => y,
            Tile::Open => ny,
        };
        ry as isize
    }

    fn dec_x(&self, x: isize, y: isize) -> isize {
        let row = self.rows.get(y as usize).unwrap();
        // if x == 0 && row.last() == Some(&Tile::Wall) {
        //     return x as isize;
        // }
        let nx = (x - 1).rem_euclid(row.len() as isize) as usize;
        let x = x as usize;
        let rx = match row[nx] {
            Tile::Empty => {
                let mut xx = row.len() - 1;
                while row[xx] == Tile::Empty {
                    xx -= 1;
                }
                if row[xx] == Tile::Open {
                    xx
                } else {
                    x
                }
            }
            Tile::Wall => x,
            Tile::Open => nx,
        };
        rx as isize
    }

    fn inc_x(&self, x: isize, y: isize) -> isize {
        let x = x as usize;
        let row = self.rows.get(y as usize).unwrap();
        // if x == row.len() - 1 && row[0] == Tile::Wall {
        //     return x as isize;
        // }
        let nx = (x + 1).rem_euclid(row.len());
        let rx = match row[nx] {
            Tile::Empty => {
                let mut xx = 0;
                while row[xx] == Tile::Empty {
                    xx += 1;
                }
                if row[xx] == Tile::Open {
                    xx
                } else {
                    x
                }
            }
            Tile::Wall => x,
            Tile::Open => nx,
        };
        rx as isize
    }
}

#[derive(Debug)]
struct Cube {
    region_size: isize,
    rows: Vec<Vec<Tile>>,
    regions: Vec::<(isize, isize)>,
}

impl Cube {
    fn new(board: &Board) -> Self {
        let region_size = (board.rows.len() / 4) as isize; // Going round a cube you have 4 going round and 2 on the side
        Self {
            region_size,
            rows: board.rows.clone(),
            /*
                 01
                 4
                32
                5
                (col,row) / (x,y)
             */
            regions: vec![(region_size, 0),
                          (region_size * 2, 0),
                          (region_size, region_size * 2),
                          (0, region_size * 2),
                          (region_size, region_size),
                          (0, region_size * 3),
            ],
        }
    }

    fn in_bounds(&self, xy: (isize, isize)) -> bool {
        if xy.1 >= 0 && xy.1 < self.rows.len() as isize {
            let y = xy.1 as usize;
            let row = self.rows.get(y).unwrap();
            if xy.0 >= 0 && xy.0 < row.len() as isize {
                return true;
            }
        }
        false
    }

    fn re_face(&self, xy: (isize, isize), facing: &Direction) -> Position {
        println!("{:?}", xy);
        let region_index = self.regions.iter()
            .enumerate()
            .filter_map(|(index, reg)| {
                // println!("{:?} .. {:?}", reg.0 + self.region_size, reg.1+self.region_size  );
                println!("x: {:?} > {:?} >= {:?}", reg.0 + self.region_size, xy.0, reg.0);
                println!("y: {:?} > {:?} >= {:?}", reg.1 + self.region_size, xy.1, reg.1);
                println!("{}", xy.1 as f64 / self.rows.len() as f64 * 3.0);
                println!("{}", xy.1 as f64 / self.rows.len() as f64 * 4.0);
                if reg.0 + self.region_size > xy.0
                    && xy.0 >= reg.0
                    && reg.1 + self.region_size > xy.1
                    && xy.1 >= reg.1 {
                    Some(index)
                } else {
                    None
                }
            })
            .last()
            .unwrap();

        let reg_coord = (xy.0 - self.regions[region_index].0, xy.1 - self.regions[region_index].1);
        /*
             01    0
             4   123
            32     45
            5
         */
        let rs_1 = self.region_size - 1;
        let x = match region_index {
            0 => match facing {
                Direction::Up => (self.regions[5].0, self.regions[5].1 + reg_coord.0, Direction::Right),
                Direction::Left => (self.regions[3].0, self.regions[3].1 - reg_coord.1 + rs_1, Direction::Right),
                _ => (xy.0, xy.1, *facing)
            }
            1 => match facing {
                Direction::Up => (reg_coord.0, self.regions[5].1 + self.region_size - 1, Direction::Up),
                Direction::Right => (self.regions[2].0 + rs_1, self.regions[2].1 + rs_1 - reg_coord.1, Direction::Left),
                Direction::Down => (self.regions[4].0 + rs_1, self.regions[4].1 + reg_coord.0, Direction::Left),
                _ => (xy.0, xy.1, *facing)
            }
            2 => match facing {
                Direction::Right => (self.regions[1].0 + rs_1, self.regions[1].1 + rs_1 - reg_coord.1, Direction::Left),
                Direction::Down => (self.regions[5].0 + rs_1, self.regions[5].1 + reg_coord.0, Direction::Left),
                _ => (xy.0, xy.1, *facing)
            },
            3 => match facing {
                Direction::Up => (self.regions[4].0, self.regions[4].1 + reg_coord.0, Direction::Right),
                Direction::Left => (self.regions[0].0, self.regions[0].1 - reg_coord.1 + rs_1, Direction::Right),
                _ => (xy.0, xy.1, *facing)
            }
            4 => match facing {
                Direction::Right => (self.regions[1].0 + reg_coord.1, self.regions[1].1 + rs_1, Direction::Up),
                Direction::Left => (self.regions[3].0 + reg_coord.1, self.regions[3].1, Direction::Down),
                _ => (xy.0, xy.1, *facing)
            }
            5 => match facing {
                Direction::Right => (self.regions[2].0 + reg_coord.1, self.regions[1].1 + rs_1, Direction::Up),
                Direction::Down => (self.regions[1].0 + reg_coord.0, self.regions[1].1, Direction::Down),
                Direction::Left => (self.regions[0].0 + reg_coord.1, self.regions[0].1, Direction::Down),
                _ => (xy.0, xy.1, *facing)
            }
            _ => panic!("Unknown index")
        };
        x.into()
    }

    fn mv(&self, pos: &Position) -> Position {
        let go: (isize, isize) = match pos.facing {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };
        let new_xy = (pos.x + go.0, pos.y + go.1);
        if self.in_bounds(new_xy) {
            match self.rows.get(new_xy.1 as usize).unwrap().get(new_xy.0 as usize).unwrap() {
                Tile::Empty => {
                    // We have gone to a new region and need to re-face.
                    let n = self.re_face(new_xy, &pos.facing);
                    if self.rows.get(n.y as usize).unwrap().get(n.x as usize).unwrap() != &Tile::Wall {
                        return n;
                    } else {
                        return *pos;
                    }
                }
                Tile::Wall => return *pos,
                Tile::Open => return Position::from((new_xy.0, new_xy.1, pos.facing)),
            }
        }
        *pos
    }
}

#[derive(Debug)]
struct MonkeyMap {
    cube: Cube,
    board: Board,
    moves: Vec<Move>,
}

impl MonkeyMap {
    fn parse(input: &str) -> Self {
        let moves = input.lines().last().map(|s| Move::parse(s)).unwrap();
        let mut rows = Vec::new();
        let mut width = 0;
        for line in input.lines() {
            if line.is_empty() {
                break;
            }
            let row: Vec<Tile> = line.chars().map(|c| Tile::from(c)).collect();
            width = max(width, row.len());
            rows.push(row);
        }
        let height = rows.len();
        let board = Board::new(&rows, width, height);
        Self {
            cube: Cube::new(&board),
            board,
            moves,
        }
    }

    fn walk(&self) -> isize {
        let mut pos = Position {
            x: self.board.top_left() as isize,
            y: 0,
            facing: Direction::Right,
        };
        // println!("Start pos: {:?}", pos);
        // MonkeyMap::print(&pos, &self.board);
        for mv in self.moves.iter() {
            // println!("Apply move: {:?} {:?}", mv, pos.facing);
            pos = pos.apply(mv, &self.board);
            // println!("New pos: {:?} \t after {:?}", pos, mv);
            // MonkeyMap::print(&pos, &self.board);
        }
        // println!("End pos: {:?}", pos);
        // MonkeyMap::print(&pos, &self.board);
        (pos.y + 1) * 1000 + (pos.x + 1) * 4 + pos.facing.value()
    }
    fn walk_folded(&self) -> isize {
        let mut pos = Position {
            x: self.board.top_left() as isize,
            y: 0,
            facing: Direction::Right,
        };
        // println!("{:?}", self.cube);
        for mv in self.moves.iter() {
            // println!("Apply move: {:?} {:?}", mv, pos.facing);
            pos = pos.apply_folded(mv, &self.cube);
            // println!("New pos: {:?} \t after {:?}", pos, mv);
            // MonkeyMap::print(&pos, &self.board);
        }

        (pos.y + 1) * 1000 + (pos.x + 1) * 4 + pos.facing.value()
    }

    #[allow(dead_code)]
    fn print(pos: &Position, board: &Board) {
        for y in 0..board.rows.len() {
            for x in 0..board.rows.get(y).unwrap().len() {
                if pos.y as usize == y && pos.x as usize == x {
                    match pos.facing {
                        Direction::Up => print!("^"),
                        Direction::Right => print!(">"),
                        Direction::Down => print!("v"),
                        Direction::Left => print!("<"),
                    }
                } else {
                    match board.rows.get(y).unwrap().get(x).unwrap() {
                        Tile::Empty => print!(" "),
                        Tile::Wall => print!("#"),
                        Tile::Open => print!("."),
                    }
                }
            }
            println!();
        }
    }
}

fn part_a(input: &str) -> isize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let map = MonkeyMap::parse(open.as_str());
// println!("{:?}", map);
    map.walk()
}

fn part_b(input: &str) -> isize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let map = MonkeyMap::parse(open.as_str());
    // map.walk_folded()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day22();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day22/input.txt";
        assert_eq!(43466, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day22/input.txt";
        assert_eq!(0, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day22/test-input.txt";
        let result = part_a(input);
        assert_eq!(6032, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day22/test-input.txt";
        let result = part_b(input);
        assert_eq!(5031, result);
    }
}