use aoc2021::day13::parse_input;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day13.txt");
    let (mut paper, folds) = parse_input(input)?;
    paper.fold(&folds[0..1]);

    let part_1 = paper.count_points();
    println!("Part 1: {}", part_1);

    paper.fold(&folds[1..]);
    print!("{}", paper);

    Ok(())
}
