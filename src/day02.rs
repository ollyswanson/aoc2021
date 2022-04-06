use std::str::FromStr;

use anyhow::{anyhow, bail};

pub struct Commands(Vec<Command>);

impl Commands {
    pub fn final_position(&self) -> (i32, i32) {
        use Direction::{Down, Forward, Up};
        self.0
            .iter()
            .fold((0, 0), |(x, y), command| match command.direction {
                Up => (x, y - command.amount),
                Down => (x, y + command.amount),
                Forward => (x + command.amount, y),
            })
    }

    pub fn final_position_aim(&self) -> ((i32, i32), i32) {
        use Direction::{Down, Forward, Up};
        self.0.iter().fold(((0, 0), 0), |((x, y), aim), command| {
            match command.direction {
                Up => ((x, y), aim - command.amount),
                Down => ((x, y), aim + command.amount),
                Forward => ((x + command.amount, y + command.amount * aim), aim),
            }
        })
    }
}

impl FromStr for Commands {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let commands: Vec<Command> = s.lines().map(str::parse).collect::<anyhow::Result<_>>()?;

        Ok(Commands(commands))
    }
}

pub struct Command {
    pub direction: Direction,
    pub amount: i32,
}

pub enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut tokens = s.split_whitespace();
        let direction = match tokens.next().ok_or_else(|| anyhow!("Missing direction"))? {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => bail!("Invalid direction"),
        };

        let amount = tokens
            .next()
            .ok_or_else(|| anyhow!("Missing amount"))?
            .parse::<i32>()?;

        Ok(Self { direction, amount })
    }
}
