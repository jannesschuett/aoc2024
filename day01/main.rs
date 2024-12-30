use std::collections::HashMap;
use std::collections::HashSet;

use nom::character::complete::{digit1, newline, space1};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let input_string = include_str!("input");
    let example_string = include_str!("example");
    task1(input_string);
    task2(input_string);
}

fn task1(input_string: &str) {
    let (_, (mut first_col, mut second_col)) = parse_input(input_string).unwrap();
    first_col.sort();
    second_col.sort();

    let mut sum_of_differences = 0;
    for (i, j) in first_col.iter().zip(second_col.iter()) {
        sum_of_differences += (j - i).abs();
    }

    println!("Sum of differences: {}", sum_of_differences);
}

fn task2(input_string: &str) {
    let (_, (first_col, second_col)) = parse_input(input_string).unwrap();
    let unique_numbers: HashSet<i32> = HashSet::from_iter(first_col.iter().cloned());
    let mut number_of_occurences: HashMap<i32, i32> = HashMap::new();
    for number in second_col {
        if unique_numbers.contains(&number) {
            let count = number_of_occurences.entry(number).or_insert(0);
            *count += 1;
        }
    }
    let mut sum = 0;
    for number in first_col {
        if number_of_occurences.contains_key(&number) {
            sum += number * number_of_occurences.get(&number).unwrap();
        }
    }
    println!("Sum of numbers: {}", sum);
}

fn parse_input(i: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    let (i, tuples) = separated_list1(newline, parse_line)(i)?;
    let (first_col, second_col) = tuples.into_iter().unzip();
    Ok((i, (first_col, second_col)))
}

fn parse_line(i: &str) -> IResult<&str, (i32, i32)> {
    let (i, (left, _, right)) = tuple((digit1, space1, digit1))(i)?;
    Ok((
        i,
        (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()),
    ))
}
