use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone)]
struct Map {
    dimension: Position,
    fields: HashMap<Position, char>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Position> for &'a Position {
    type Output = Position;

    fn sub(self, other: &'b Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Position;

    fn mul(self, other: i32) -> Position {
        Position {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

fn main() {
    let _example = include_str!("example");
    let input = include_str!("input");
    task1(_example);
    task1(input);
    task2(_example);
    task2(input);
}

fn task1(input: &str) {
    let parsed: Map = parse_input(input);
    let result = parsed
        .fields
        .iter()
        .tuple_combinations()
        .filter(|((_, val1), (_, val2))| val1 == val2)
        .fold(
            HashSet::default(),
            |mut set: HashSet<Position>, (first, second)| {
                let (pos1, _) = first;
                let (pos2, _) = second;
                let pos_diff = pos1 - pos2;
                set.insert(pos1 + &pos_diff);
                set.insert(pos2 - &pos_diff);
                set
            },
        );
    println!(
        "{:?}",
        result
            .iter()
            .filter(|pos| !is_outside(**pos, parsed.dimension))
            .count()
    );
    //_print_map(&result.1, Position { x: 50, y: 50 });
}

fn task2(input: &str) {
    let time = std::time::Instant::now();
    let parsed: Map = parse_input(input);
    let result = parsed
        .fields
        .iter()
        .tuple_combinations()
        .filter(|((_, val1), (_, val2))| val1 == val2)
        .fold(
            HashSet::default(),
            |mut set: HashSet<Position>, (first, second)| {
                let (pos1, _) = first;
                let (pos2, _) = second;
                let pos_diff = pos1 - pos2;
                let mut scalar = 0;
                while !is_outside(pos1 + &(pos_diff * scalar), parsed.dimension) {
                    set.insert(pos1 + &(pos_diff * scalar));
                    scalar += 1;
                }
                scalar = 0;
                while !is_outside(pos2 - &(pos_diff * scalar), parsed.dimension) {
                    set.insert(pos2 - &(pos_diff * scalar));
                    scalar += 1;
                }
                set
            },
        );
    println!("{:?}", result.iter().count());
    println!("Time: {:?}", time.elapsed());
    //_print_map(&result.1, Position { x: 50, y: 50 });
}
fn is_outside(pos: Position, dims: Position) -> bool {
    pos.x < 0 || pos.x >= dims.x || pos.y < 0 || pos.y >= dims.y
}

fn parse_input(input: &str) -> Map {
    let (map, _, y) =
        input.chars().fold(
            (HashMap::default(), 0, 0),
            |(mut map, x, y), char| match char {
                '\n' => (map, 0, y + 1),
                char if char.is_alphanumeric() => {
                    map.insert(Position { x, y }, char);
                    (map, x + 1, y)
                }
                _ => (map, x + 1, y),
            },
        );
    Map {
        fields: map,
        dimension: Position { x: y, y },
    }
}

fn _print_map(map: &HashMap<Position, char>, dims: Position) {
    for j in 0..dims.x {
        for i in 0..dims.y {
            let pos = Position {
                x: i as i32,
                y: j as i32,
            };

            match map.get(&pos) {
                Some(&value) => print!("{}", value),
                None => {}
            }
        }
        println!();
    }
    println!("\n\n");
}
