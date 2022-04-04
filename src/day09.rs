use std::str::FromStr;

use anyhow::anyhow;

/// X and Y are the dimensions of the input with an extra layer of surrounding space
pub struct HeightMap<const X: usize, const Y: usize> {
    heights: [[u8; X]; Y],
    min_map: [[bool; X]; Y],
}

impl<const X: usize, const Y: usize> HeightMap<X, Y> {
    fn new(heights: [[u8; X]; Y]) -> Self {
        let min_map = [[false; X]; Y];
        let mut height_map: Self = Self { heights, min_map };
        height_map.calculate_mins();

        height_map
    }

    fn calculate_mins(&mut self) {
        for y in 1..Y {
            for x in 1..X {
                let height = self.heights[y][x];
                if height < self.heights[y - 1][x]
                    && height < self.heights[y][x - 1]
                    && height < self.heights[y][x + 1]
                    && height < self.heights[y + 1][x]
                {
                    self.min_map[y][x] = true;
                }
            }
        }
    }

    pub fn risk_level(&self) -> u32 {
        let mut risk_level: u32 = 0;
        for (y, row) in self.min_map.iter().enumerate() {
            for (x, &is_safe) in row.iter().enumerate() {
                if is_safe {
                    risk_level += self.heights[y][x] as u32 + 1;
                }
            }
        }

        risk_level
    }
}

impl<const X: usize, const Y: usize> FromStr for HeightMap<X, Y> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut heights = [[9; X]; Y];

        for (y, line) in (1..Y).zip(s.lines()) {
            for (x, c) in (1..X).zip(line.chars()) {
                heights[y][x] = c.to_digit(10).ok_or_else(|| anyhow!("Invalid input"))? as u8;
            }
        }

        Ok(Self::new(heights))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let height_map = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

        let height_map: HeightMap<12, 7> = height_map.parse().unwrap();

        assert_eq!(height_map.risk_level(), 15)
    }
}
