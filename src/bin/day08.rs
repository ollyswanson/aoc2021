use std::io;
use std::io::Read;

use aoc2021::day08::{part_1, part_2};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let part_1 = part_1(&input);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&input);
    println!("Part 2: {}", part_2);
    Ok(())
}
