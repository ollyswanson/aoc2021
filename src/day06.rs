use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Lanternfish(u8);

impl Lanternfish {
    pub fn new() -> Self {
        Self(8)
    }

    /// Returns true if counter has reached 0 and Lanternfish will reproduce
    fn progress_day(&mut self) -> bool {
        if self.0 == 0 {
            self.0 = 6;
            return true;
        }

        self.0 -= 1;
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct School {
    school: Vec<Lanternfish>,
}

impl School {
    fn progress_day(&mut self) {
        let mut new_fish: u32 = 0;

        for fish in self.school.iter_mut() {
            if fish.progress_day() {
                new_fish += 1;
            }
        }

        for _ in 0..new_fish {
            self.school.push(Lanternfish::new());
        }
    }

    /// Progress by x days and return the size of the school after the specified number of days.
    pub fn progress(&mut self, days: usize) -> usize {
        for _ in 0..days {
            self.progress_day();
        }

        self.school.len()
    }
}

impl FromStr for School {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let school: Vec<Lanternfish> = s
            .split(',')
            .map(str::parse::<u8>)
            .map(|r| r.map_err(|e| anyhow!(e)).map(Lanternfish))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self { school })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_school_fromstr() {
        let school = "1,2,3,4,5";
        let expected = School {
            school: vec![
                Lanternfish(1),
                Lanternfish(2),
                Lanternfish(3),
                Lanternfish(4),
                Lanternfish(5),
            ],
        };

        assert_eq!(school.parse::<School>().unwrap(), expected);
    }

    #[test]
    fn progress_by() {
        let mut school: School = "3,4,3,1,2".parse().unwrap();
        let size = school.progress(80);

        assert_eq!(size, 5934);
    }
}
