use std::str::FromStr;

use anyhow::anyhow;

// Find proper solution below.
// Part 1 was naively implemented and over-engineered. Leaving as a reminder to self to think about
// the problem deeply before getting carried away!
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

// Non-naive implementation that represents the school as 9 buckets with each bucket containing
// the number of Lanternfish scheduled to reproduce after x days.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonNaiveSchool {
    school: [u64; 9],
}

impl NonNaiveSchool {
    fn progress_day(&mut self) {
        self.school.rotate_left(1);
        self.school[6] += self.school[8];
    }

    pub fn progress(&mut self, days: usize) -> u64 {
        for _ in 0..days {
            self.progress_day();
        }

        self.school.iter().sum()
    }
}

impl FromStr for NonNaiveSchool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut school: [u64; 9] = [0; 9];
        for c in s.split(',') {
            let days_until_reprod: usize = c.parse()?;
            school[days_until_reprod] += 1;
        }

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

    #[test]
    fn can_parse_non_naive_school_fromstr() {
        let school = "1,2,3,4,5,5";
        let expected = [0, 1, 1, 1, 1, 2, 0, 0, 0];

        assert_eq!(school.parse::<NonNaiveSchool>().unwrap().school, expected);
    }

    #[test]
    fn progress_by_non_naive() {
        let mut school: NonNaiveSchool = "3,4,3,1,2".parse().unwrap();
        let size = school.progress(80);

        assert_eq!(size, 5934);
    }
}
