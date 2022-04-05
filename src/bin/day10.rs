use std::io;
use std::io::Read;

use nom::Parser;

use aoc2021::day10::Lines;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let lines: Lines = input
        .lines()
        .into_iter()
        .map(|line| line.parse())
        .collect::<anyhow::Result<Vec<_>>>()?
        .into();

    let part_1 = lines.score();
    println!("Part 1: {}", part_1);

    let part_2 = lines.completion_score();
    println!("Part 2: {}", part_2);

    Ok(())
}
