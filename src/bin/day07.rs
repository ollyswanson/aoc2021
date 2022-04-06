use aoc2021::day07::Crabs;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day07.txt");

    let crabs: Crabs = input.parse()?;

    let part_1 = crabs.find_best_position().unwrap();
    println!("Part 1: {}", part_1);

    let part_2 = crabs.find_best_position_alt().unwrap();
    println!("Part 2: {}", part_2);
    Ok(())
}
