use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn main() {
    let example = include_str!("example");
    let input = include_str!("input");
    task1(input);
    task2(input);
}

fn task1(i: &str) {
    let field = build_field(i);
    let mut sum = 0;
    for (pos, _) in &field {
        if field.get(&pos) == Some(&"X") {
            sum += check_mas(&field, &pos);
        }
    }
    println!("{:?}", sum);
}

fn task2(input: &str) {
    let field = build_field(input);
    let sum: i32 = field
        .iter()
        .map(|(pos, value)| {
            if value == &"A" {
                check_x_mas(&field, pos)
            } else {
                false
            }
        })
        .map(|result| if result { 1 } else { 0 })
        .sum();
    println!("{:?}", sum);
}

fn build_field(input: &str) -> HashMap<Position, &str> {
    let mut field: HashMap<Position, &str> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.split("").enumerate() {
            field.insert(
                Position {
                    x: x as i32,
                    y: y as i32,
                },
                char,
            );
        }
    }
    field
}

fn check_mas(field: &HashMap<Position, &str>, pos: &Position) -> i32 {
    check_horizontal(&field, &pos) + check_vertical(&field, &pos)
}

fn check_x_mas(field: &HashMap<Position, &str>, pos: &Position) -> bool {
    let first_diag: bool = check_up_left_down_right(&field, &pos);
    let second_diag: bool = check_up_right_down_left(&field, &pos);
    first_diag && second_diag
}

fn check_up_left_down_right(field: &HashMap<Position, &str>, pos: &Position) -> bool {
    let up_left = Position {
        x: pos.x - 1,
        y: pos.y - 1,
    };
    let down_right = Position {
        x: pos.x + 1,
        y: pos.y + 1,
    };
    if field.get(&up_left) == Some(&"M") && field.get(&down_right) == Some(&"S") {
        return true;
    }
    if field.get(&up_left) == Some(&"S") && field.get(&down_right) == Some(&"M") {
        return true;
    }
    false
}

fn check_up_right_down_left(field: &HashMap<Position, &str>, pos: &Position) -> bool {
    let up_right = Position {
        x: pos.x + 1,
        y: pos.y - 1,
    };
    let down_left = Position {
        x: pos.x - 1,
        y: pos.y + 1,
    };
    if field.get(&up_right) == Some(&"M") && field.get(&down_left) == Some(&"S") {
        return true;
    }
    if field.get(&up_right) == Some(&"S") && field.get(&down_left) == Some(&"M") {
        return true;
    }
    false
}

fn check_horizontal(field: &HashMap<Position, &str>, pos: &Position) -> i32 {
    let directions = vec![
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];
    let mas = vec!["M", "A", "S"];
    let mut found = 0;
    for direction in directions {
        let mut complete_mas = true;
        for mas_pos in 1..=mas.len() {
            let i = mas_pos as i32;
            let new_pos = match direction {
                Direction::Left => Position {
                    x: pos.x - i,
                    y: pos.y,
                },
                Direction::Right => Position {
                    x: pos.x + i,
                    y: pos.y,
                },
                Direction::Up => Position {
                    x: pos.x,
                    y: pos.y - i,
                },
                Direction::Down => Position {
                    x: pos.x,
                    y: pos.y + i,
                },
                _ => Position { x: 0, y: 0 },
            };
            if !(field.get(&new_pos) == Some(&mas[mas_pos - 1])) {
                complete_mas = false;
                break;
            }
        }
        if complete_mas {
            found += 1;
        }
    }
    found
}

fn check_vertical(field: &HashMap<Position, &str>, pos: &Position) -> i32 {
    let directions = vec![
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];
    let mas = vec!["M", "A", "S"];
    let mut found = 0;
    for direction in directions {
        let mut complete_mas = true;
        for mas_pos in 1..=mas.len() {
            let i = mas_pos as i32;
            let new_pos = match direction {
                Direction::UpLeft => Position {
                    x: pos.x - i,
                    y: pos.y - i,
                },
                Direction::UpRight => Position {
                    x: pos.x + i,
                    y: pos.y - i,
                },
                Direction::DownLeft => Position {
                    x: pos.x - i,
                    y: pos.y + i,
                },
                Direction::DownRight => Position {
                    x: pos.x + i,
                    y: pos.y + i,
                },
                _ => Position { x: 0, y: 0 },
            };
            if !(field.get(&new_pos) == Some(&mas[mas_pos - 1])) {
                complete_mas = false;
                break;
            }
        }
        if complete_mas {
            found += 1;
        }
    }
    found
}
