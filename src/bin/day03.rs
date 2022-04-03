use std::io::{self, BufRead, BufReader};

use anyhow::anyhow;

use aoc2021::day03::Readings;

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(io::stdin());
    let readings: Readings<12> = reader
        .lines()
        .map(|line| line.map_err(|e| anyhow!(e))?.parse())
        .collect::<anyhow::Result<Vec<_>>>()?
        .into();

    let (gamma, epsilon) = readings.power_consumption();
    println!("Part 1: {}", gamma * epsilon);

    let life_support_rating = readings.life_support_rating();
    println!("Part 2: {}", life_support_rating);

    Ok(())
}
