use aoc2021::day05::{Line, Lines};

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day05.txt");
    let lines: Lines = input
        .lines()
        .map(str::parse)
        .collect::<anyhow::Result<Vec<Line>>>()?
        .into();

    println!("Part 1: {}", lines.dangerous_points_horizontal_or_vert());
    println!("Part 2: {}", lines.dangerous_points_all());
    Ok(())
}
