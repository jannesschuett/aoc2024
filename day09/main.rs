use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BlockType {
    File,
    Free,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    struct_type: BlockType,
    id: i64,
    size: usize,
}

fn main() {
    let _example = include_str!("example").trim();
    let input = include_str!("input").trim();
    task1(_example);
    task1(input);
    task2(_example);
    task2(input);
}

fn task1(input: &str) {
    let mut input = parse_input(input);

    let mut lower_bound = 0;
    let mut upper_bound = input.len() - 1;
    loop {
        while input[lower_bound] != -1 {
            lower_bound += 1;
        }
        while input[upper_bound] == -1 {
            upper_bound -= 1;
        }
        if lower_bound >= upper_bound {
            break;
        }
        input[lower_bound] = input[upper_bound];
        input[upper_bound] = -1;
    }
    let result: u64 = input
        .iter()
        .filter(|num| **num != -1)
        .enumerate()
        .map(|(index, num)| *num as u64 * index as u64)
        .sum();
    println!("{:?}", result);
}

fn task2(input: &str) {
    let mut input = parse_input_to_disk(input);

    let mut upper_bound = input.len() - 1;
    loop {
        // print_disk(&input);
        while upper_bound > 1 && input[upper_bound].struct_type == BlockType::Free {
            upper_bound -= 1;
        }
        if upper_bound <= 1 {
            break;
        }
        let file_to_mirgate = input[upper_bound];
        let mut fitting_space = input[..upper_bound]
            .iter()
            .enumerate()
            .filter(|(_, block)| {
                block.struct_type == BlockType::Free && block.size >= file_to_mirgate.size
            });
        match fitting_space.next() {
            Some((index, _)) => {
                input[upper_bound].struct_type = BlockType::Free;
                input[upper_bound].id = -1;
                input[index].size -= file_to_mirgate.size;
                input.insert(index, file_to_mirgate);
            }
            None => {
                upper_bound -= 1;
                continue;
            }
        }
    }
    let result: usize = input
        .iter()
        .scan(0, |pos, block| {
            let index = *pos;
            *pos += block.size;
            Some((*block, index))
        })
        .filter(|(block, _)| block.struct_type == BlockType::File)
        .map(|(block, index)| {
            let mut sum: usize = 0;
            for i in 0..block.size {
                sum += block.id as usize * (index + i);
            }
            sum
        })
        .sum();
    println!("{:?}", result);
}

fn print_disk(input: &Vec<Block>) {
    for item in input.iter() {
        for _ in 0..item.size {
            match item.struct_type {
                BlockType::File => print!("{}", item.id),
                BlockType::Free => print!("."),
            }
        }
    }
    println!();
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .chars()
        .enumerate()
        .fold(vec![], |mut vec, (index, char)| {
            let digit = char.to_digit(10).unwrap();
            if index % 2 == 0 {
                for _ in 0..digit {
                    vec.push((index / 2).try_into().unwrap());
                }
            } else {
                for _ in 0..digit {
                    vec.push(-1);
                }
            }
            vec
        })
}

fn parse_input_to_disk(input: &str) -> Vec<Block> {
    input
        .chars()
        .enumerate()
        .fold(vec![], |mut blocks, (index, char)| {
            let digit = char.to_digit(10).unwrap();
            if digit == 0 {
                return blocks;
            }
            if index % 2 == 0 {
                blocks.push(Block {
                    struct_type: BlockType::File,
                    id: (index / 2).try_into().unwrap(),
                    size: digit as usize,
                });
            } else {
                blocks.push(Block {
                    struct_type: BlockType::Free,
                    id: -1,
                    size: digit as usize,
                });
            }
            blocks
        })
}
