mod part1;

fn main() {
    let _example = include_str!("example");
    let input = include_str!("input");
    part1::solve(_example);
    part1::solve(input);
}
