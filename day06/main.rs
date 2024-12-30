use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Map {
    dimension: Dimension,
    guard_position: Position,
    fields: HashMap<Position, FieldStatus>,
    visited_directions: HashMap<Position, HashSet<Direction>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Dimension {
    x: u32,
    y: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldStatus {
    Empty,
    Blocked,
    Visited,
    Guard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let _example = include_str!("example");
    let input = include_str!("input");
    task1(input);
    task2(input);
}

fn task1(input: &str) {
    let (map, _) = guard_cover(parse_field(input).unwrap());
    let result = map
        .fields
        .iter()
        .filter(|(_, value)| value == &&FieldStatus::Visited)
        .count();

    // print_map(&map);

    println!("{:?}", result);
}

fn task2(input: &str) {
    let time = std::time::Instant::now();
    let map = parse_field(input).unwrap();
    let (guard_cover_map, _) = guard_cover(map.clone());
    let mut result = 0;
    // for every visited field check if guard is looping when replaced with obstacle
    // we need an advanced map, tracking the visisted direction for every field, if field is visited in same direction twice we know it is looping
    for (index, value) in guard_cover_map.fields.iter() {
        if value == &FieldStatus::Visited {
            let mut map = map.clone();
            map.fields.insert(*index, FieldStatus::Blocked);
            let (_result, is_looping) = guard_cover(map);
            if is_looping {
                result += 1;
            }

            // now check if looping
            // print_map(&result);
        }
    }
    println!("Time: {:?}", time.elapsed());
    println!("{:?}", result);
}

fn guard_cover(mut map: Map) -> (Map, bool) {
    let view_direction = vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    let mut current_view_direction_index = 0;
    let mut is_looping = false;
    map.visited_directions
        .insert(map.guard_position, HashSet::from([Direction::Up]));
    loop {
        let current_view_direction = view_direction[current_view_direction_index];
        let current_position = map.guard_position;
        let next_position;
        match current_view_direction {
            Direction::Up => {
                next_position = Position {
                    x: current_position.x,
                    y: current_position.y - 1,
                };
                // is next position is outside of dimensions break
            }
            Direction::Right => {
                next_position = Position {
                    x: current_position.x + 1,
                    y: current_position.y,
                };
            }
            Direction::Down => {
                next_position = Position {
                    x: current_position.x,
                    y: current_position.y + 1,
                };
            }
            Direction::Left => {
                next_position = Position {
                    x: current_position.x - 1,
                    y: current_position.y,
                };
            }
        }
        map.fields.insert(current_position, FieldStatus::Visited);
        map.visited_directions
            .entry(current_position)
            .or_insert(HashSet::new())
            .insert(current_view_direction);
        if check_outside(&next_position, &map.dimension) {
            break;
        }
        match map.visited_directions.get(&next_position) {
            Some(dir) => {
                if dir.contains(&current_view_direction) {
                    is_looping = true;
                    break;
                }
            }
            _ => {}
        }

        match map.fields.get(&next_position) {
            Some(FieldStatus::Blocked) => {
                current_view_direction_index = (current_view_direction_index + 1) % 4;
            }
            _ => {
                map.guard_position = next_position;
            }
        }
    }
    (map, is_looping)
}

fn _print_map(map: &Map) {
    let dims = map.dimension;
    for j in 0..dims.x {
        for i in 0..dims.y {
            let pos = Position {
                x: i as i32,
                y: j as i32,
            };
            match map.fields.get(&pos) {
                Some(FieldStatus::Blocked) => print!("#"),
                Some(FieldStatus::Empty) => print!("."),
                Some(FieldStatus::Visited) => print!("X"),
                Some(FieldStatus::Guard) => print!("^"),
                None => print!(" "),
            }
        }
        println!();
    }
    println!("\n\n");
}

fn check_outside(pos: &Position, dim: &Dimension) -> bool {
    if pos.y < 0 || pos.y >= dim.y as i32 || pos.x < 0 || pos.x >= dim.x as i32 {
        true
    } else {
        false
    }
}

fn parse_field(input: &str) -> Result<Map, nom::error::Error<&str>> {
    let (_, guard_pos, map, dim) = input.chars().fold(
        (
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            HashMap::default(),
            Dimension { x: 0, y: 0 },
        ),
        |(mut pos, mut guard_pos, mut map, mut dim), char| match char {
            '^' => {
                guard_pos = pos;
                map.insert(pos, FieldStatus::Visited);
                pos.x += 1;
                (pos, guard_pos, map, dim)
            }
            '\n' => {
                dim.x = pos.x as u32;
                pos.y += 1;
                dim.y = pos.y as u32;
                pos.x = 0;
                (pos, guard_pos, map, dim)
            }
            _ => {
                map.insert(
                    pos,
                    match char {
                        '.' => FieldStatus::Empty,
                        '#' => FieldStatus::Blocked,
                        '^' => FieldStatus::Guard,
                        _ => panic!("Invalid character"),
                    },
                );
                pos.x += 1;
                (pos, guard_pos, map, dim)
            }
        },
    );
    Ok(Map {
        dimension: dim,
        fields: map,
        guard_position: guard_pos,
        visited_directions: HashMap::default(),
    })
}
