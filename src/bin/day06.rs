use std::io;
use std::io::Read;

use aoc2021::day06::School;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut school: School = input.parse()?;
    let part_1 = school.progress(80);
    println!("Part 1: {}", part_1);

    Ok(())
}
