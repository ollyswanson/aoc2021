use std::io;
use std::io::Read;

use aoc2021::day09::{HeightMap, QuickUnion};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let height_map: HeightMap<102, 102> = input.parse()?;

    let part_1 = height_map.risk_level();
    println!("Part 1: {}", part_1);

    let quick_union = QuickUnion::new(&height_map);

    let part_2 = quick_union.product_three_biggest_basins();
    println!("Part 2; {}", part_2);

    Ok(())
}
