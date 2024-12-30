use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{
        complete,
        streaming::{line_ending, space1},
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const OPERATORS: [char; 2] = ['*', '+'];
const OPERATORS_2: [char; 3] = ['+', '*', '|'];

fn main() {
    let _example = include_str!("example");
    let input = include_str!("input");
    task1(input);
    task2(input);
}

fn task1(input: &str) {
    let (_, calculations) = parse_input(input).unwrap();
    let result: u64 = calculations
        .iter()
        .filter_map(|(test, numbers)| {
            let operator_count = numbers.len() - 1;
            (0..operator_count)
                .map(|_| OPERATORS)
                .multi_cartesian_product()
                .any(|seq| {
                    let mut s = seq.iter();
                    *test
                        == numbers
                            .iter()
                            .copied()
                            .reduce(|a, b| match s.next().unwrap() {
                                '*' => a * b,
                                '+' => a + b,
                                _ => panic!("Invalid operator."),
                            })
                            .unwrap()
                })
                .then_some(test)
        })
        .sum();
    println!("{:?}", result)
}

fn task2(input: &str) {
    let time = std::time::Instant::now();
    let (_, calculations) = parse_input(input).unwrap();
    let result: u64 = calculations
        .iter()
        .filter_map(|(test, numbers)| {
            let operator_count = numbers.len() - 1;
            (0..operator_count)
                .map(|_| OPERATORS_2)
                .multi_cartesian_product()
                .any(|seq| {
                    let mut s = seq.iter();
                    *test
                        == numbers
                            .iter()
                            .copied()
                            .reduce(|a, b| match s.next().unwrap() {
                                '*' => a * b,
                                '+' => a + b,
                                '|' => a * 10u64.pow(b.ilog10() + 1) + b,
                                _ => panic!("Invalid operator."),
                            })
                            .unwrap()
                })
                .then_some(test)
        })
        .sum();
    println!("Time: {:?}", time.elapsed());
    println!("{:?}", result)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    let (input, calculations) = separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)?;
    Ok((input, calculations))
}
