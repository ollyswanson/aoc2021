use aoc2021::day11::Octopuses;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day11.txt");
    let mut octopuses: Octopuses<12, 12> = input.parse()?;

    let part_1 = octopuses.flashed();
    println!("Part 1: {}", part_1);

    Ok(())
}
