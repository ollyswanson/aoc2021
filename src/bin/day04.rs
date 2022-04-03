use std::io;
use std::io::Read;

use aoc2021::day04::Bingo;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut bingo_game: Bingo<5, 5> = input.parse()?;

    println!("part 1: {}", bingo_game.play_game());
    println!("part 2: {}", bingo_game.play_until_last());

    Ok(())
}
