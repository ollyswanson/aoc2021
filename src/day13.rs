use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Debug, Clone)]
pub struct Paper(HashSet<(i32, i32)>);

impl Paper {
    pub fn fold(&mut self, fold: Fold) {
        let current = std::mem::take(&mut self.0);

        for (x, y) in current {
            let (x, y) = match fold {
                Fold::X(offset) => {
                    if x > offset {
                        (2 * offset - x, y)
                    } else {
                        (x, y)
                    }
                }
                Fold::Y(offset) => {
                    if y > offset {
                        (x, 2 * offset - y)
                    } else {
                        (x, y)
                    }
                }
            };
            self.0.insert((x, y));
        }
    }

    pub fn count_points(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for Paper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut set: HashSet<(i32, i32)> = HashSet::new();

        for line in s.lines() {
            let (x, y) = line
                .split_once(',')
                .ok_or_else(|| anyhow!("Invalid Input"))?;
            let (x, y) = (x.parse()?, y.parse()?);
            set.insert((x, y));
        }

        Ok(Self(set))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Fold {
    X(i32),
    Y(i32),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (axis, position) = s
            .trim_start_matches("fold along ")
            .split_once('=')
            .ok_or_else(|| anyhow!("Invalid input"))?;

        let position = position.parse::<i32>()?;

        match axis {
            "x" => Ok(Fold::X(position)),
            "y" => Ok(Fold::Y(position)),
            _ => bail!("Invalid Input!"),
        }
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<(Paper, Vec<Fold>)> {
    let (paper, folds) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Invalid Input"))?;

    let paper: Paper = paper.parse()?;
    let folds: Vec<Fold> = folds
        .lines()
        .map(str::parse)
        .collect::<anyhow::Result<_>>()?;

    Ok((paper, folds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        let (mut paper, folds) = parse_input(input).unwrap();
        paper.fold(folds[0]);

        assert_eq!(paper.count_points(), 17);
    }
}
