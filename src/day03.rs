use std::str::FromStr;

use anyhow::bail;

pub struct Readings<const READING_LENGTH: usize>(Vec<Reading<READING_LENGTH>>);

const fn masks<const LENGTH: usize>() -> [u32; LENGTH] {
    let mut masks = [0u32; LENGTH];
    let mut i = 0;
    while i < LENGTH {
        masks[i] = 1 << (LENGTH - 1 - i);
        i += 1;
    }

    masks
}

impl<const READING_LENGTH: usize> Readings<READING_LENGTH> {
    pub fn power_consumption(&self) -> (u32, u32) {
        let masks = masks::<READING_LENGTH>();
        // count number of 1s in each position
        let counts = self
            .0
            .iter()
            .fold([0u32; READING_LENGTH], |mut count, reading| {
                for i in 0..READING_LENGTH {
                    count[i] += (masks[i] & reading.0) >> (READING_LENGTH - 1 - i);
                }
                count
            });

        let half_len = (self.0.len() / 2) as u32;
        let mut bits = [0u32; READING_LENGTH];
        for (i, &count) in counts.iter().enumerate() {
            // if there are more 1s than 0s then set position to 1
            if count > half_len {
                bits[i] = 1;
            }
        }

        let gamma = bits
            .iter()
            .rev()
            .enumerate()
            .fold(0u32, |gamma, (i, &bit)| gamma + (bit << i));

        // flip bits to find epsilon and set bits for > 2^4 back to 0
        let epsilon = !gamma & ((1 << READING_LENGTH) - 1);

        (gamma, epsilon)
    }

    pub fn life_support_rating(&self) -> u32 {
        self.o2().0 * self.co2().0
    }

    fn o2(&self) -> Reading<READING_LENGTH> {
        let mut readings: Vec<Reading<READING_LENGTH>> = self.0.to_vec();
        let mut temp: Vec<Reading<READING_LENGTH>> = Vec::new();
        let mut pos: usize = 0;

        while readings.len() > 1 {
            let bit = Self::most_common_bit(&readings, pos);
            for reading in readings.iter().copied() {
                if (reading.0 >> (READING_LENGTH - 1 - pos) & 1) == bit {
                    temp.push(reading);
                }
            }
            pos += 1;
            std::mem::swap(&mut temp, &mut readings);
            temp.clear();
        }

        readings[0]
    }

    fn co2(&self) -> Reading<READING_LENGTH> {
        let mut readings: Vec<Reading<READING_LENGTH>> = self.0.to_vec();
        let mut temp: Vec<Reading<READING_LENGTH>> = Vec::new();
        let mut pos: usize = 0;

        while readings.len() > 1 {
            let bit = Self::least_common_bit(&readings, pos);
            for reading in readings.iter().copied() {
                if (reading.0 >> (READING_LENGTH - 1 - pos) & 1) == bit {
                    temp.push(reading);
                }
            }
            pos += 1;
            std::mem::swap(&mut temp, &mut readings);
            temp.clear();
        }

        readings[0]
    }

    fn most_common_bit(readings: &[Reading<READING_LENGTH>], pos: usize) -> u32 {
        let masks = masks::<READING_LENGTH>();
        let count = readings.iter().copied().fold(0, |count, reading| {
            count + ((masks[pos] & reading.0) >> (READING_LENGTH - 1 - pos))
        });

        let half_len = (readings.len() / 2) as u32;

        match readings.len() % 2 {
            0 => {
                if count >= half_len {
                    1
                } else {
                    0
                }
            }
            1 => {
                if count > half_len {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    fn least_common_bit(readings: &[Reading<READING_LENGTH>], pos: usize) -> u32 {
        let masks = masks::<READING_LENGTH>();
        let count = readings.iter().copied().fold(0, |count, reading| {
            count + ((masks[pos] & reading.0) >> (READING_LENGTH - 1 - pos))
        });

        let half_len = (readings.len() / 2) as u32;

        match readings.len() % 2 {
            0 => {
                if count >= half_len {
                    0
                } else {
                    1
                }
            }
            1 => {
                if count > half_len {
                    0
                } else {
                    1
                }
            }
            _ => unreachable!(),
        }
    }
}

impl<const READING_LENGTH: usize> From<Vec<Reading<READING_LENGTH>>> for Readings<READING_LENGTH> {
    fn from(readings: Vec<Reading<READING_LENGTH>>) -> Self {
        Self(readings)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Reading<const LENGTH: usize>(u32);

impl<const LENGTH: usize> FromStr for Reading<LENGTH> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        if s.len() != LENGTH {
            bail!("Incorrect input length")
        }

        let reading: u32 = s
            .chars()
            .rev()
            .enumerate()
            .try_fold(0, |reading, (i, c)| match c {
                '0' => Ok(reading),
                '1' => Ok(reading + (1 << i)),
                _ => bail!("invalid input"),
            })?;

        Ok(Self(reading))
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    mod reading {
        use test_case::test_case;

        use super::Reading;

        #[test_case("00000", 0; "Should parse to 0")]
        #[test_case("00001", 1; "Should parse to 1")]
        #[test_case("01010", 10; "Should parse to 10")]
        #[test_case("10000", 16; "Should parse to 16")]
        #[test_case("10101", 21; "Should parse to 21")]
        #[test_case("11111", 31; "Should parse to 31")]
        fn parse_reading(input: &str, expected: u32) {
            let reading: Reading<5> = input.parse().unwrap();
            assert_eq!(reading.0, expected);
        }

        #[test]
        #[should_panic]
        fn errors_on_invalid_length() {
            let _reading: Reading<5> = "01".parse().unwrap();
        }

        #[test]
        #[should_panic]
        fn errors_on_invalid_character() {
            let _reading: Reading<5> = "20101".parse().unwrap();
        }
    }

    mod readings {
        use super::Readings;

        #[test]
        fn calculate_power_consumption() {
            let readings: Readings<5> = vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .into_iter()
            .map(|reading| reading.parse())
            .collect::<anyhow::Result<Vec<_>>>()
            .unwrap()
            .into();

            let (gamma_rate, epsilon_rate) = readings.power_consumption();

            assert_eq!(gamma_rate, 22);
            assert_eq!(epsilon_rate, 9);
        }

        #[test]
        fn calculate_life_support() {
            let readings: Readings<5> = vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .into_iter()
            .map(|reading| reading.parse())
            .collect::<anyhow::Result<Vec<_>>>()
            .unwrap()
            .into();

            let life_support = readings.life_support_rating();
            assert_eq!(life_support, 230);
        }
    }
}
