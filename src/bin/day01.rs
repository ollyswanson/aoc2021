use anyhow::Error;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let input = include_str!("../../inputs/day01.txt");
    let depths: Vec<i32> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let part1 = depths.iter().tuple_windows().filter(|(i, j)| j > i).count();
    println!("part 1: {}", part1);

    let part2 = depths
        .iter()
        .tuple_windows()
        .map(|(i, j, k)| i + j + k)
        .tuple_windows()
        .filter(|(i, j)| j > i)
        .count();
    println!("part 2: {}", part2);

    Ok(())
}
