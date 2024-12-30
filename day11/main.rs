use std::collections::HashMap;

use itertools::Either;
use nom::{character::complete::space1, multi::separated_list1, IResult};
use num_traits::Euclid;

fn main() {
    let _example = include_str!("example").trim();
    let input = include_str!("input").trim();
    task1(_example);
    task1(input);
    task2(input);
}

fn task1(input: &str) {
    let (_, mut numbers) = parse(input).unwrap();

    for _ in 0..25 {
        numbers = numbers
            .iter()
            .map(|number| match number {
                0 => vec![1],
                number if number.to_string().len() % 2 == 0 => {
                    let number_as_string = number.to_string();
                    vec![
                        number_as_string[0..number_as_string.len() / 2]
                            .parse::<u64>()
                            .unwrap(),
                        number_as_string[number_as_string.len() / 2..]
                            .parse::<u64>()
                            .unwrap(),
                    ]
                }
                default => vec![default * 2024],
            })
            .flatten()
            .collect();
    }
    println!("{:?}", &numbers.iter().count());
}

fn task2(input: &str) {
    let (_, mut nums) = parse(input).unwrap();

    // stone_number, stone_count
    let mut cache: HashMap<u64, u64> = HashMap::default();

    for num in nums {
        cache
            .entry(num)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    }

    for _ in 0..75 {
        let mut new_cache: HashMap<u64, u64> = HashMap::default();

        for (num, count) in cache.into_iter() {
            match num {
                0 => {
                    new_cache
                        .entry(1)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                }
                n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                    let num_digits = n.checked_ilog10().unwrap_or(0) + 1;
                    let (left, right) = n.div_rem_euclid(&10u64.pow(num_digits / 2));

                    new_cache
                        .entry(left)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                    new_cache
                        .entry(right)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                }
                n => {
                    new_cache
                        .entry(n * 2024)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                }
            }
        }
        cache = new_cache;
    }

    println!("{:?}", cache.values().sum::<u64>());
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, numbers) = separated_list1(space1, nom::character::complete::u64)(input)?;
    Ok((input, numbers))
}
