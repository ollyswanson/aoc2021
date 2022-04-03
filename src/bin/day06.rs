use std::io;
use std::io::Read;

use aoc2021::day06::{NonNaiveSchool, School};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut school: NonNaiveSchool = input.parse()?;
    let mut school_for_part_2: NonNaiveSchool = school.clone();
    let part_1 = school.progress(80);
    println!("Part 1: {}", part_1);

    let part_2 = school_for_part_2.progress(256);
    println!("Part 2: {}", part_2);

    Ok(())
}
