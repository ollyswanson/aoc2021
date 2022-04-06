use aoc2021::day03::Readings;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day03.txt");
    let readings: Readings<12> = input
        .lines()
        .map(str::parse)
        .collect::<anyhow::Result<Vec<_>>>()?
        .into();

    let (gamma, epsilon) = readings.power_consumption();
    println!("Part 1: {}", gamma * epsilon);

    let life_support_rating = readings.life_support_rating();
    println!("Part 2: {}", life_support_rating);

    Ok(())
}
