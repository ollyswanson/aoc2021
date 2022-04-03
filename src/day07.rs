use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{anyhow, bail};
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Crabs {
    crabs: Vec<i32>,
    min: i32,
    max: i32,
}

impl Crabs {
    /// Returns Some(total_fuel_used) or None if all the fuel is used before the crabs are in
    /// position.
    fn move_crabs_to_position(&self, pos: i32) -> i32 {
        let mut fuel_used = 0;
        for &crab_position in self.crabs.iter() {
            fuel_used += (pos - crab_position).abs();
        }
        fuel_used
    }

    fn move_crabs_to_position_alt(&self, pos: i32) -> i32 {
        let mut fuel_used = 0;
        for &crab_position in self.crabs.iter() {
            fuel_used += triangular_number((pos - crab_position).abs());
        }
        fuel_used
    }

    pub fn find_best_position(&self) -> Option<i32> {
        (self.min..=self.max)
            .map(|pos| self.move_crabs_to_position(pos))
            .min()
    }

    pub fn find_best_position_alt(&self) -> Option<i32> {
        (self.min..=self.max)
            .map(|pos| self.move_crabs_to_position_alt(pos))
            .min()
    }
}

impl FromStr for Crabs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let crabs: Vec<i32> = s
            .split(',')
            .map(|position| position.parse().map_err(|e: ParseIntError| anyhow!(e)))
            .collect::<anyhow::Result<_>>()?;

        let (&min, &max) = match crabs.iter().minmax() {
            itertools::MinMaxResult::MinMax(min, max) => (min, max),
            _ => bail!("Invalid input"),
        };

        Ok(Self { crabs, min, max })
    }
}

fn triangular_number(num: i32) -> i32 {
    ((num + 1) * num) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_crabs_fromstr() {
        let crabs = "1,2,3,4,5,6,7,8";
        let expected = Crabs {
            crabs: vec![1, 2, 3, 4, 5, 6, 7, 8],
            min: 1,
            max: 8,
        };

        assert_eq!(crabs.parse::<Crabs>().unwrap(), expected);
    }

    #[test]
    fn find_best_position() {
        let crabs: Crabs = "16,1,2,0,4,2,7,1,2,14".parse().unwrap();

        assert_eq!(crabs.find_best_position().unwrap(), 37);
    }

    #[test]
    fn find_best_position_alt() {
        let crabs: Crabs = "16,1,2,0,4,2,7,1,2,14".parse().unwrap();

        assert_eq!(crabs.find_best_position_alt().unwrap(), 168);
    }
}
