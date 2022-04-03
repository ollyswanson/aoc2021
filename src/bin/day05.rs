use std::io::{self, BufRead, BufReader};

use aoc2021::day05::{Line, Lines};

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(io::stdin());
    let mut lines: Vec<Line> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line.parse()?);
    }

    let lines: Lines = lines.into();

    println!("Part 1: {}", lines.dangerous_points_horizontal_or_vert());
    println!("Part 2: {}", lines.dangerous_points_all());
    Ok(())
}
