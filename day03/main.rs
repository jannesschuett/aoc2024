use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};
use nom::combinator::value;
use nom::multi::{many1, many_till};
use nom::sequence::tuple;
use nom::{IResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Mul(i32, i32),
    Dont,
    Do,
}

fn main() {
    let example = include_str!("example");
    let input = include_str!("input");
    task1(input);
    task2(input);
}

fn task1(i: &str) {
    let (_, parsed) = parse_input(i).unwrap();
    let result: i32 = parsed
        .iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
            Instruction::Dont => 0,
            Instruction::Do => 0,
        })
        .sum();
    println!("{:?}", result);
}

fn task2(i: &str) {
    let (_, parsed) = parse_input(i).unwrap();
    let (_, result) = parsed
        .iter()
        .fold((Instruction::Do, 0), |(process, sum), ins| match ins {
            Instruction::Mul(a, b) => {
                if process == Instruction::Do {
                    (Instruction::Do, sum + a * b)
                } else {
                    (process, sum)
                }
            }
            Instruction::Dont => (Instruction::Dont, sum),
            Instruction::Do => (Instruction::Do, sum),
        });
    println!("{:?}", result);
}

fn parse_input(i: &str) -> IResult<&str, Vec<Instruction>> {
    let (i, instructions) =
        many1(many_till(anychar, parse_instruction).map(|(_discard, mul)| mul))(i)?;
    Ok((i, instructions))
}

fn parse_mul_instr(i: &str) -> IResult<&str, Instruction> {
    let (i, (_, first_digit, _, second_digit, _)) =
        tuple((tag("mul("), digit1, tag(","), digit1, tag(")")))(i)?;
    if first_digit.len() > 3 || second_digit.len() > 3 {
        return Ok((i, Instruction::Mul(0, 0)));
    } else {
        let first_digit = first_digit.parse::<i32>().unwrap();
        let second_digit = second_digit.parse::<i32>().unwrap();
        return Ok((i, Instruction::Mul(first_digit, second_digit)));
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        parse_mul_instr,
    ))(input)
}
