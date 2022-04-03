use std::io::{self, BufReader};

use aoc2021::day02::Commands;

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(io::stdin());
    let commands: Commands = reader.try_into()?;

    let final_position = commands.final_position();
    println!("part 1: {}", final_position.0 * final_position.1);

    let ((x, y), _) = commands.final_position_aim();
    println!("part 2: {}", x * y);

    Ok(())
}
