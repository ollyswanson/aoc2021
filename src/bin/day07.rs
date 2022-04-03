use std::io;
use std::io::Read;

use aoc2021::day07::Crabs;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let crabs: Crabs = input.parse()?;

    let part_1 = crabs.find_best_position().unwrap();
    println!("Part 1: {}", part_1);
    Ok(())
}
