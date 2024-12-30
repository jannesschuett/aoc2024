use std::collections::{HashMap, HashSet};

fn main() {
    let _example = include_str!("example").trim();
    let input = include_str!("input").trim();
    task1(_example);
    task1(input);
    task2(_example);
    task2(input);
}

fn task1(input: &str) {
    let input = parse_input(input);
    let starts = input
        .iter()
        .filter(|(_, &v)| v == 0)
        .collect::<Vec<(&(usize, usize), &usize)>>();

    let result: usize = starts
        .iter()
        .map(|(&(x, y), &value)| {
            let mut search_vector = vec![((x, y), value)];
            let mut visited: HashSet<(usize, usize)> = HashSet::default();
            visited.insert((x, y));
            loop {
                match search_vector.first() {
                    Some(&((x, y), _)) => {
                        // append all possible next steps, make sure to keep a hashset of visited nodes
                        append_to_vector_if_exists(
                            &mut search_vector,
                            &mut visited,
                            &input,
                            (x + 1, y),
                        );
                        if x > 0 {
                            append_to_vector_if_exists(
                                &mut search_vector,
                                &mut visited,
                                &input,
                                (x - 1, y),
                            );
                        }
                        append_to_vector_if_exists(
                            &mut search_vector,
                            &mut visited,
                            &input,
                            (x, y + 1),
                        );
                        if y > 0 {
                            append_to_vector_if_exists(
                                &mut search_vector,
                                &mut visited,
                                &input,
                                (x, y - 1),
                            );
                        }
                        search_vector.remove(0);
                    }
                    None => {
                        break;
                    }
                }
            }
            visited
                .iter()
                .map(|(x, y)| input.get(&(*x, *y)).unwrap())
                .filter(|value| **value == 9)
                .count()
        })
        .sum();
    println!("Task 1: {:?}", result);
}

fn task2(input: &str) {
    let input = parse_input(input);
    let starts = input
        .iter()
        .filter(|(_, &v)| v == 0)
        .collect::<Vec<(&(usize, usize), &usize)>>();

    let result: usize = starts
        .iter()
        .map(|(&(x, y), &value)| {
            let mut search_vector = vec![((x, y), value)];
            let mut sum: usize = 0;
            loop {
                match search_vector.first() {
                    Some(&((x, y), value)) => {
                        if value == 9 {
                            sum += 1;
                        } else {
                            // append all possible next steps, make sure to keep a hashset of visited nodes
                            append_to_vector_if_exists_task2(
                                &mut search_vector,
                                &input,
                                (x + 1, y),
                            );
                            if x > 0 {
                                append_to_vector_if_exists_task2(
                                    &mut search_vector,
                                    &input,
                                    (x - 1, y),
                                );
                            }
                            append_to_vector_if_exists_task2(
                                &mut search_vector,
                                &input,
                                (x, y + 1),
                            );
                            if y > 0 {
                                append_to_vector_if_exists_task2(
                                    &mut search_vector,
                                    &input,
                                    (x, y - 1),
                                );
                            }
                        }
                        search_vector.remove(0);
                    }
                    None => {
                        break;
                    }
                }
            }
            sum
        })
        .sum();
    println!("Task 2: {:?}", result);
}

fn append_to_vector_if_exists(
    next: &mut Vec<((usize, usize), usize)>,
    visited: &mut HashSet<(usize, usize)>,
    map: &HashMap<(usize, usize), usize>,
    position: (usize, usize),
) {
    match map.get(&position) {
        Some(value) => {
            if *value == next.first().unwrap().1 + 1 {
                visited.insert(position);
                next.push((position, *value))
            }
        }
        None => {}
    }
}

fn append_to_vector_if_exists_task2(
    next: &mut Vec<((usize, usize), usize)>,
    map: &HashMap<(usize, usize), usize>,
    position: (usize, usize),
) {
    match map.get(&position) {
        Some(value) => {
            if *value == next.first().unwrap().1 + 1 {
                next.push((position, *value))
            }
        }
        None => {}
    }
}

fn parse_input(input: &str) -> HashMap<(usize, usize), usize> {
    input
        .chars()
        .fold(
            (HashMap::default(), 0, 0),
            |(mut map, x, y): (HashMap<(usize, usize), usize>, usize, usize), char| match char {
                '\n' => (map, 0, y + 1),
                char => {
                    map.insert((x, y), char.to_digit(10).unwrap() as usize);
                    (map, x + 1, y)
                }
            },
        )
        .0
}
