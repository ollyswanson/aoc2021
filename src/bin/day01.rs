use std::io::{self, BufRead, BufReader};

use anyhow::{anyhow, Error};
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(io::stdin());
    let depths: Vec<i32> = reader
        .lines()
        .map(|line| {
            line.map_err(|e| anyhow!(e))?
                .parse::<i32>()
                .map_err(|e| anyhow!(e))
        })
        .collect::<Result<_, Error>>()?;

    let part1 = depths.iter().tuple_windows().filter(|(i, j)| j > i).count();
    println!("part 1: {}", part1);

    let part2 = depths
        .iter()
        .tuple_windows()
        .map(|(i, j, k)| i + j + k)
        .tuple_windows()
        .filter(|(i, j)| j > i)
        .count();
    println!("part 2: {}", part2);

    Ok(())
}
