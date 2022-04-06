use aoc2021::day06::NonNaiveSchool;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day06.txt");

    let mut school: NonNaiveSchool = input.parse()?;
    let mut school_for_part_2: NonNaiveSchool = school.clone();
    let part_1 = school.progress(80);
    println!("Part 1: {}", part_1);

    let part_2 = school_for_part_2.progress(256);
    println!("Part 2: {}", part_2);

    Ok(())
}
