use aoc2021::day11::Octopuses;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day11.txt");
    let mut octopuses: Octopuses<12, 12> = input.parse()?;
    let mut octopuses_for_part_2 = octopuses;

    let part_1 = octopuses.flashed();
    println!("Part 1: {}", part_1);

    let part_2 = octopuses_for_part_2.find_all_flash();
    println!("Part 2: {}", part_2);

    Ok(())
}
