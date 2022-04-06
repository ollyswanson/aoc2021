use aoc2021::day12::CaveGraph;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day12.txt");

    let cave_graph: CaveGraph = input.parse()?;
    let part_1 = cave_graph.find_all_paths().len();
    println!("Part 1: {}", part_1);

    let part_2 = cave_graph.find_all_paths_alt().len();
    println!("Part 2: {}", part_2);
    Ok(())
}
