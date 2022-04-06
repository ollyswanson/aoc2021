use aoc2021::day10::Lines;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day10.txt");
    let lines: Lines = input
        .lines()
        .map(str::parse)
        .collect::<anyhow::Result<Vec<_>>>()?
        .into();

    let part_1 = lines.score();
    println!("Part 1: {}", part_1);

    let part_2 = lines.completion_score();
    println!("Part 2: {}", part_2);

    Ok(())
}
