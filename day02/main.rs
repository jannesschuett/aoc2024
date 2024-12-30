use nom::character::complete::{digit1, newline, space1};
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug)]
enum Direction {
    Ascending,
    Descending,
}

fn main() {
    let example = include_str!("example");
    let input = include_str!("input");
    task1(input);
    task2(input);
}

fn task1(input: &str) {
    let (_, parsed) = parse_input(input).unwrap();
    let mut sum = 0;
    for report in &parsed {
        if valid_report(&report) {
            sum += 1;
        }
    }
    println!("{:?}", sum);
}

fn task2(input: &str) {
    let (_, parsed) = parse_input(input).unwrap();
    let mut sum = 0;
    for report in &parsed {
        if valid_report(&report) {
            sum += 1;
        } else {
            for i in 0..report.len() {
                let mut modified_vector = report.clone();
                modified_vector = modified_vector
                    .iter()
                    .enumerate()
                    .filter_map(|(index, &value)| if index == i { None } else { Some(value) })
                    .collect();
                if valid_report(&modified_vector) {
                    sum += 1;
                    break;
                }
            }
        }
    }
    println!("{:?}", sum);
}

fn valid_report(report: &Vec<i32>) -> bool {
    let first = report[0];
    let second = report[1];
    let mut direction = Direction::Ascending;
    if first > second {
        direction = Direction::Descending;
    }
    for i in 0..report.len() - 1 {
        let current = report[i];
        let next = report[i + 1];
        if (current - next).abs() > 3 {
            return false;
        }
        match direction {
            Direction::Ascending => {
                if current >= next {
                    return false;
                }
            }
            Direction::Descending => {
                if current <= next {
                    return false;
                }
            }
        }
    }
    true
}

fn parse_input(i: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (i, lines) = separated_list1(newline, parse_line)(i)?;
    Ok((i, lines))
}

fn parse_line(i: &str) -> IResult<&str, Vec<i32>> {
    let (i, numbers) = separated_list1(space1, parse_num)(i)?;
    Ok((i, numbers))
}

fn parse_num(i: &str) -> IResult<&str, i32> {
    let (i, num) = digit1(i)?;
    Ok((i, num.parse::<i32>().unwrap()))
}
