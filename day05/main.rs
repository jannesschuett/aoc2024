use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

fn main() {
    let example = include_str!("example");
    let input = include_str!("input");
    task1(input);
    task2(input);
}

fn task1(i: &str) {
    let (remaining_input, rules) = parse_rules(i).unwrap();
    let (_, manuals) = parse_manual_pages(remaining_input).unwrap();
    let mut sum = 0;
    for manual in &manuals {
        sum += check_correctly_ordered(manual, &rules);
    }
    println!("{:?}", sum);
}

fn task2(input: &str) {
    let (remaining_input, rules) = parse_rules(input).unwrap();
    let (_, manuals) = parse_manual_pages(remaining_input).unwrap();
    let mut sum = 0;
    for manual in &manuals {
        if check_correctly_ordered(manual, &rules) == 0 {
            // println!("{:?}", manual);
            sum += reorder_manual(manual, &rules);
        }
    }
    println!("{:?}", sum);
}

fn check_correctly_ordered(manual: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut valid_manual = true;
    for (index, page) in manual.iter().enumerate() {
        for page_before in 0..index {
            let Some(page_rules) = rules.get(page) else {
                continue;
            };
            if page_rules.contains(manual.get(page_before).unwrap()) {
                valid_manual = false;
                break;
            }
        }
        if !valid_manual {
            break;
        }
    }
    if valid_manual {
        let middle_page = manual.get(manual.len() / 2).unwrap();
        return *middle_page;
    } else {
        return 0;
    }
}

fn reorder_manual(manual: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut reordered = vec![manual[0]];
    for page in manual.iter().skip(1) {
        let mut inserted = false;
        for (index, reordered_page) in reordered.iter().enumerate() {
            let Some(page_rules) = rules.get(page) else {
                continue;
            };
            if page_rules.contains(reordered_page) {
                reordered.insert(index, *page);
                inserted = true;
                break;
            }
        }
        if !inserted {
            reordered.push(*page);
        }
    }

    *reordered.get(reordered.len() / 2).unwrap()
}

fn parse_rules(input: &str) -> IResult<&str, HashMap<u32, Vec<u32>>> {
    let (input, rules): (&str, Vec<(u32, u32)>) = many1(terminated(
        separated_pair(complete::u32, tag("|"), complete::u32),
        line_ending,
    ))(input)?;
    let (input, _) = line_ending(input)?;
    let map = rules.iter().fold(
        HashMap::default(),
        |mut acc: HashMap<u32, Vec<u32>>, (page, after)| {
            acc.entry(*page)
                .and_modify(|existing_entry| existing_entry.push(*after))
                .or_insert(vec![*after]);
            acc
        },
    );
    Ok((input, map))
}

fn parse_manual_pages(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, pages): (&str, Vec<Vec<u32>>) = many1(terminated(
        separated_list1(tag(","), complete::u32),
        line_ending,
    ))(input)?;
    Ok((input, pages))
}
