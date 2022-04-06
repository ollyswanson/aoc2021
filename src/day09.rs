use std::str::FromStr;

use anyhow::anyhow;
use itertools::Itertools;

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

pub struct QuickUnion<'a, const X: usize, const Y: usize> {
    height_map: &'a HeightMap<X, Y>,
    tree_size: [[usize; X]; Y],
    trees: [[(usize, usize); X]; Y],
}

impl<'a, const X: usize, const Y: usize> QuickUnion<'a, X, Y> {
    pub fn new(height_map: &'a HeightMap<X, Y>) -> Self {
        let tree_size = [[1usize; X]; Y];
        let mut trees = [[(0, 0); X]; Y];

        for y in 0..Y {
            for x in 0..X {
                trees[y][x] = (x, y);
            }
        }

        let mut quick_union = Self {
            height_map,
            tree_size,
            trees,
        };

        quick_union.solve();

        quick_union
    }

    fn solve(&mut self) {
        for y in 1..Y {
            for x in 1..X {
                self.union((x, y), (x + 1, y));
                self.union((x, y), (x, y + 1));
            }
        }
    }

    fn find(&self, mut a: (usize, usize)) -> (usize, usize) {
        while a != self.trees[a.1][a.0] {
            a = self.trees[a.1][a.0];
        }
        a
    }

    fn union(&mut self, a: (usize, usize), b: (usize, usize)) {
        if self.height_map.heights[a.1][a.0] == 9 || self.height_map.heights[b.1][b.0] == 9 {
            return;
        }

        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        if self.tree_size[root_a.1][root_a.0] < self.tree_size[root_b.1][root_b.0] {
            self.trees[root_a.1][root_a.0] = root_b;
            self.tree_size[root_b.1][root_b.0] += self.tree_size[root_a.1][root_a.0];
        } else {
            self.trees[root_b.1][root_b.0] = root_a;
            self.tree_size[root_a.1][root_a.0] += self.tree_size[root_b.1][root_b.0];
        }
    }

    pub fn product_three_biggest_basins(&self) -> usize {
        self.tree_size
            .iter()
            .flatten()
            .sorted()
            .rev()
            .take(3)
            .product()
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

    #[test]
    fn part_2() {
        let height_map = "\
2199943210
3987894921
9856789892
8767896789
9899965678";
        let height_map: HeightMap<12, 7> = height_map.parse().unwrap();
        let quick_union = QuickUnion::new(&height_map);

        assert_eq!(quick_union.product_three_biggest_basins(), 1134);
    }
}
