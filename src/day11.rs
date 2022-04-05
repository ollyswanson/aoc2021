use std::str::FromStr;

use anyhow::anyhow;

/// Until const generics support arithmetic then X and Y take into account ghost rows around the
/// input meaning that if the grid of octopuses is 10 x 10 then X is 12 and Y is 12
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Octopuses<const X: usize, const Y: usize> {
    inner: [[i32; X]; Y],
}

const DELTAS: [(usize, usize); 8] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (0, 1),
    (2, 1),
    (0, 2),
    (1, 2),
    (2, 2),
];

impl<const X: usize, const Y: usize> Octopuses<X, Y> {
    fn step(&mut self) -> u32 {
        // any octoptus with an energy level of 9 will be pushed into the queue, have it's energy
        // level set to 0, and be marked as "flashed" for the remainder of the step
        let mut queue: Vec<(usize, usize)> = Vec::new();
        let mut flashed = [[false; X]; Y];
        let mut num_flashed: u32 = 0;

        // increase energy levels
        for energy_level in self.inner.iter_mut().flatten() {
            *energy_level += 1;
        }

        // init queue
        for (y, row) in self.inner.iter_mut().enumerate() {
            for (x, energy_level) in row.iter_mut().enumerate() {
                if *energy_level > 9 {
                    queue.push((x, y));
                    flashed[y][x] = true;
                    *energy_level = 0;
                    num_flashed += 1;
                }
            }
        }

        while let Some((x, y)) = queue.pop() {
            for &delta in DELTAS.iter() {
                let (i, j) = (x + delta.0 - 1, y + delta.1 - 1);
                let flashed = &mut flashed[j][i];
                if !*flashed {
                    let energy_level = &mut self.inner[j][i];
                    *energy_level += 1;
                    if *energy_level > 9 {
                        *flashed = true;
                        *energy_level = 0;
                        queue.push((i, j));
                        num_flashed += 1;
                    }
                }
            }
        }

        num_flashed
    }

    pub fn flashed(&mut self) -> u32 {
        let mut flashed = 0;

        for _ in 0..100 {
            flashed += self.step();
        }

        flashed
    }

    pub fn find_all_flash(&mut self) -> u32 {
        for i in 0.. {
            if self.step() == 100 {
                return i + 1;
            }
        }

        unreachable!()
    }
}

impl<const X: usize, const Y: usize> FromStr for Octopuses<X, Y> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        // we will set the ghost rows to i32::min as sentinel values so that although we will
        // operate on them, they won't affect the algorithm.
        let mut octopuses: [[i32; X]; Y] = [[i32::MIN; X]; Y];
        for (y, line) in (1..Y).zip(s.lines()) {
            for (x, c) in (1..X).zip(line.chars()) {
                octopuses[y][x] = c.to_digit(10).ok_or_else(|| anyhow!("Invalid input"))? as i32;
            }
        }

        Ok(Self { inner: octopuses })
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::Octopuses;

    #[test]
    fn count_flashes() {
        let mut octopuses: Octopuses<12, 12> = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            .parse()
            .unwrap();

        assert_eq!(octopuses.flashed(), 1656);
    }
}
