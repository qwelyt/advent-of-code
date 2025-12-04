use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

use crate::util::time;

pub fn day17() {
    println!("== Day 17 ==");
    let input = "src/day17/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl GridCoord {
    fn go(self, direction: &Direction) -> GridCoord {
        match direction {
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y - 1),
        }.into()
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
}

impl From<char> for Direction {
    fn from(char: char) -> Self {
        if char == '<' { return Direction::Left; }
        if char == '>' { return Direction::Right; }
        panic!()
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Shape {
    Dash,
    Plus,
    L,
    Line,
    Box,
}

impl Shape {}

impl Shape {
    fn lowest_point(&self, tower_height: usize) -> GridCoord {
        let lowes_point_for_rock = (tower_height + 3 + 1) as usize;
        match self {
            Shape::Dash => (2, lowes_point_for_rock),
            Shape::Plus => (3, lowes_point_for_rock),
            Shape::L => (2, lowes_point_for_rock),
            Shape::Line => (2, lowes_point_for_rock),
            Shape::Box => (2, lowes_point_for_rock)
        }.into()
    }
    fn points_from_lowest_left(&self, lowest_left: GridCoord) -> Vec<GridCoord> {
        let x = lowest_left.x;
        let y = lowest_left.y;
        let v = match self {
            Shape::Dash => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 3, y),
            ],
            Shape::Plus => vec![
                (x, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
                (x, y + 2),
            ],
            Shape::L => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Shape::Line => vec![
                (x, y),
                (x, y + 1),
                (x, y + 2),
                (x, y + 3),
            ],
            Shape::Box => vec![
                (x, y),
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
            ],
        };
        v.iter().map(|p| (*p).into()).collect()
    }
}

struct RockState {
    rock: Shape,
    lowest_left_point: GridCoord,
}

impl RockState {
    fn new(rock: Shape, tower_height: usize) -> Self {
        Self {
            rock,
            lowest_left_point: rock.lowest_point(tower_height),
        }
    }

    fn can_move(&self, history: &HashMap<usize, HashSet<usize>>, direction: &Direction) -> bool {
        let points: Vec<GridCoord> = self.rock.points_from_lowest_left(self.lowest_left_point);
        // println!("Can move {:?} \t{:?} \n\t\t\t\t{:?}", direction, chamber, self.rock.as_chamber_arr(self.lowest_left_point));
        match direction {
            Direction::Left => {
                for point in points.iter() {
                    if point.x == 0 {
                        return false;
                    }
                    if let Some(height) = history.get(&point.y) {
                        if height.contains(&(point.x - 1)) {
                            return false;
                        }
                    }
                }
            }
            Direction::Right => {
                for point in points.iter() {
                    if point.x == 6 {
                        return false;
                    }
                    if let Some(height) = history.get(&point.y) {
                        if height.contains(&(point.x + 1)) {
                            return false;
                        }
                    }
                }
            }
            Direction::Down => {
                for point in points.iter() {
                    if point.y == 1 {
                        return false;
                    }
                    if let Some(height) = history.get(&(point.y - 1)) {
                        if height.contains(&point.x) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn go(&mut self, direction: &Direction) {
        self.lowest_left_point = self.lowest_left_point.go(direction);
    }

    fn points(&self) -> Vec<GridCoord> {
        self.rock.points_from_lowest_left(self.lowest_left_point)
    }
}

struct Tower {
    jets: Vec<Direction>,
    rock_turn: usize,
    jet_turn: usize,
    rock_order: [Shape; 5],
    history: HashMap<usize, HashSet<usize>>,
}

impl Tower {
    fn parse(input: &str) -> Self {
        Self {
            jets: input.chars().map(|c| Direction::from(c)).collect(),
            rock_turn: 0,
            jet_turn: 0,
            rock_order: [
                Shape::Dash,
                Shape::Plus,
                Shape::L,
                Shape::Line,
                Shape::Box,
            ],
            history: HashMap::new(),
        }
    }

    fn update_history(&mut self, points: Vec<GridCoord>) {
        // println!("{:?}", points);
        for point in points.iter() {
            // if point.y == 111 {
            //     println!("{:?}", self);
            // }
            if !self.history.contains_key(&point.y) {
                self.history.insert(point.y, HashSet::new());
            }
            if let Some(ve) = self.history.get_mut(&point.y) {
                ve.insert(point.x);
            }
        }
    }
    fn jet_turn(&self) -> usize {
        self.jet_turn % self.jets.len()
    }
    fn rock_turn(&self) -> usize {
        self.rock_turn % self.rock_order.len()
    }
    fn let_the_rocks_fall(&mut self, num_rocks: usize, print: bool) -> usize {
        for _ in 0..num_rocks {
            let mut rock = RockState::new(self.rock_order[self.rock_turn()], *self.history.keys().max().unwrap_or(&0));
            self.rock_turn += 1;
            if print {
                if print { println!("Dropping a {:?} with LL at {:?}", rock.rock, rock.lowest_left_point); }
                self.print_state(&rock);
            }

            let mut settled = false;
            while !settled {
                // If the rock can go down, then it will first be affected by the jets,
                // then it moves down.
                // If it can no longer move down it has settled and won't be affected by jets anymore.
                let jet = self.jets.get(self.jet_turn()).unwrap();
                if rock.can_move(&self.history, jet) {
                    rock.go(jet);
                    if print {
                        println!("{:?}({},{})", jet, self.jet_turn(), self.jet_turn);
                        self.print_state(&rock);
                    }
                } else {
                    settled = true;
                    if print {
                        println!("{:?}({},{}) BLOCKED ({})", jet, self.jet_turn(), self.jet_turn, settled);
                        self.print_state(&rock);
                    }
                }
                self.jet_turn += 1;
                if rock.can_move(&self.history, &Direction::Down) {
                    rock.go(&Direction::Down);
                    settled = false;
                    if print {
                        println!("{:?} ({})", Direction::Down, settled);
                        self.print_state(&rock);
                    }
                } else {
                    settled = true;
                    if print {
                        println!("{:?} ({})", Direction::Down, settled);
                        self.print_state(&rock);
                    }
                }
            }
            // Rock can no longer move down, this means that it has hit the bottom
            // Change the chamber
            self.update_history(rock.points());
        }

        *self.history.keys().max().unwrap_or(&0)
    }
    fn cycle_height_after(&mut self, num_rocks: usize) -> usize {
        let cycle_jet_turns = self.jets.len() * self.rock_order.len();
        self.let_the_rocks_fall(2022, false); // Get rids of false cycles.
        let start = self.jet_turn;
        let bottom_height = *self.history.keys().max().unwrap_or(&0);
        let bottom_rock = self.rock_turn;
        // Do one cycle and see how far up we get
        while self.jet_turn - start < cycle_jet_turns {
            self.let_the_rocks_fall(1, false);
        }
        let height_per_cycle = *self.history.keys().max().unwrap_or(&0) - bottom_height;
        let rocks_per_cycle = self.rock_turn - bottom_rock;

        let cycles_needed = (num_rocks - 2022) / rocks_per_cycle;
        let total_rocks = cycles_needed * rocks_per_cycle + bottom_rock;
        let total_height = cycles_needed * height_per_cycle + bottom_height;
        // We don't reach the needed target of rocks. So let's drop some more
        // and see how far up that brings us.
        // Then add that height to what we say the total height of all the cycles were
        let need_more_rocks = num_rocks - total_rocks;
        let curr_h = *self.history.keys().max().unwrap_or(&0);
        self.let_the_rocks_fall(need_more_rocks, false);
        let h = *self.history.keys().max().unwrap_or(&0) - curr_h;
        // println!("H: {}", i);

        total_height + h
    }

    fn print_state(&self, rock: &RockState) {
        let rock_points = rock.rock.points_from_lowest_left(rock.lowest_left_point);
        // let max_y = max(*self.chamber.iter().max().unwrap(), rock_points.iter().map(|p| p.y).max().unwrap());
        // let min_y = *self.chamber.iter().min().unwrap();
        let max_y = *self.history.keys().max().unwrap_or(&0);
        let min_y = *self.history.keys().min().unwrap_or(&0);
        for y in (min_y..=max_y).rev() {
            print!("{}\t\t|", y);
            let h = self.history.get(&y).unwrap();
            for x in 0..7 {
                if h.contains(&x) {
                    print!("#");
                } else if rock_points.contains(&(x, y).into()) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!();
    }
    fn print_history(&self) {
        println!("Ze history!");
        let max_y = *self.history.keys().max().unwrap_or(&0);
        // for (nr, line) in self.history.iter().enumerate().rev() {
        let empty_set = HashSet::new();
        for y in (0..=max_y).rev() {
            print!("{}\t|", y);
            let set = self.history.get(&y).unwrap_or(&empty_set);
            for c in 0..7 {
                if set.contains(&c) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!()
    }
}

impl std::fmt::Debug for Tower {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.jets)?;
        // writeln!(f, "{:?}", self.chamber)?;
        writeln!(f, "Jet turn: {:?}({:?}) : {:?}", self.jet_turn(), self.jet_turn, self.jets[self.jet_turn()])?;
        writeln!(f, "Rock turn: {:?}({:?}) : {:?}", self.rock_turn(), self.rock_turn, self.rock_order[self.rock_turn()])?;


        let max_y = *self.history.keys().max().unwrap_or(&0);
        // for (nr, line) in self.history.iter().enumerate().rev() {
        let empty_set = HashSet::new();
        for y in (0..=max_y).rev() {
            write!(f, "{}\t|", y)?;
            let set = self.history.get(&y).unwrap_or(&empty_set);
            for c in 0..7 {
                if set.contains(&c) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f)?;
        Ok(())
    }
}

fn part_a(input: &str) -> usize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let mut tower = Tower::parse(open.as_str());
    tower.let_the_rocks_fall(2022, false);
    // tower.let_the_rocks_fall(1, true);
    // println!("{:?}",tower.chamber);
    // tower.print_history();
    tower.let_the_rocks_fall(0, false)
}

fn part_b(input: &str) -> usize {
    let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
    let mut tower = Tower::parse(open.as_str());
    // tower.let_the_rocks_fall(1_000_000_000_000, false)
    tower.cycle_height_after(1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day17();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day17/input.txt";
        assert_eq!(3175, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day17/input.txt";
        assert_eq!(1555113636385, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day17/test-input.txt";
        let result = part_a(input);
        assert_eq!(3068, result);
    }

    #[test]
    fn test_tower_state() {
        let input = "src/day17/test-input.txt";
        let open = std::fs::read_to_string(input.to_string()).expect("Could not read file");
        let mut tower = Tower::parse(open.as_str());
        tower.let_the_rocks_fall(1, false);
        // assert_eq!([0, 0, 1, 1, 1, 1, 0], tower.chamber);
        // tower.let_the_rocks_fall(1, false);
        // assert_eq!([0, 0, 3, 4, 3, 1, 0], tower.chamber);
        // tower.let_the_rocks_fall(1, false);
        // assert_eq!([4, 4, 6, 4, 3, 1, 0], tower.chamber);
        // tower.let_the_rocks_fall(1, false);
        // assert_eq!([4, 4, 6, 4, 7, 1, 0], tower.chamber);
        // tower.let_the_rocks_fall(1, false);
        // assert_eq!([4, 4, 6, 4, 9, 9, 0], tower.chamber);
        // tower.let_the_rocks_fall(1, false);
        // assert_eq!([4, 10, 10, 10, 10, 9, 0], tower.chamber);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day17/test-input.txt";
        let result = part_b(input);
        assert_eq!(1_514_285_714_288, result);
    }
}