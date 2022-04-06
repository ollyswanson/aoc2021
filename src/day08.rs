use itertools::Itertools;

struct Log<'a> {
    left: Vec<&'a str>,
    right: Vec<&'a str>,
}

impl<'a> Log<'a> {
    pub fn part_1(&self) -> usize {
        self.right
            .iter()
            .filter(|token| matches!(token.len(), 2 | 3 | 4 | 7))
            .count()
    }

    //  0:      1:      2:      3:      4:
    // aaaa    ....    aaaa    aaaa    ....
    // b    c  .    c  .    c  .    c  b    c
    // b    c  .    c  .    c  .    c  b    c
    //  ....    ....    dddd    dddd    dddd
    // e    f  .    f  e    .  .    f  .    f
    // e    f  .    f  e    .  .    f  .    f
    //  gggg    ....    gggg    gggg    ....

    //   5:      6:      7:      8:      9:
    //  aaaa    aaaa    aaaa    aaaa    aaaa
    // b    .  b    .  .    c  b    c  b    c
    // b    .  b    .  .    c  b    c  b    c
    //  dddd    dddd    ....    dddd    dddd
    // .    f  e    f  .    f  e    f  .    f
    // .    f  e    f  .    f  e    f  .    f
    //  gggg    gggg    ....    gggg    gggg

    pub fn part_2(&self) -> usize {
        // digits[0] holds the index of the &str in self.left represents 0
        // digits[1] holds the index of the &str in self.left represents 1 etc.
        let mut digits: [usize; 10] = [0; 10];
        // using prime representations for the displays and segments allows us to do simple
        // arithmetic to find the digits.
        let mut left_as_products: [u32; 10] = [0; 10];

        for (i, &token) in self.left.iter().enumerate() {
            left_as_products[i] = str_to_unique(token);
        }

        for (i, &token) in self.left.iter().enumerate() {
            match token.len() {
                2 => digits[1] = i,
                4 => digits[4] = i,
                3 => digits[7] = i,
                7 => digits[8] = i,
                _ => {}
            }
        }

        // 0, 6, 9
        // 6 is the only number that doesn't contain both segments from 1
        digits[6] = self
            .left
            .iter()
            .enumerate()
            .filter(|(_, token)| token.len() == 6)
            .find(|(_, token)| str_to_unique(**token) % left_as_products[digits[1]] != 0)
            .unwrap()
            .0;

        // 0, 6, 9
        // 9 is the only number that does contain all segments from 4
        digits[9] = self
            .left
            .iter()
            .enumerate()
            .filter(|(_, token)| token.len() == 6)
            .find(|(_, token)| str_to_unique(**token) % left_as_products[digits[4]] == 0)
            .unwrap()
            .0;

        // 2, 5, 3
        // 3 is the only number that contains all segments from 1
        digits[3] = self
            .left
            .iter()
            .enumerate()
            .filter(|(_, token)| token.len() == 5)
            .find(|(_, token)| str_to_unique(**token) % left_as_products[digits[1]] == 0)
            .unwrap()
            .0;

        let segment_a = left_as_products[digits[7]] / left_as_products[digits[1]];
        let segment_c = left_as_products[digits[8]] / left_as_products[digits[6]];
        let segment_e = left_as_products[digits[8]] / left_as_products[digits[9]];
        let segment_g = left_as_products[digits[9]] / (left_as_products[digits[4]] * segment_a);

        digits[5] = left_as_products
            .iter()
            .find_position(|&&product| product == left_as_products[digits[6]] / segment_e)
            .unwrap()
            .0;

        let segment_f = left_as_products[digits[1]] / segment_c;
        let segment_d = left_as_products[digits[3]] / (left_as_products[digits[7]] * segment_g);
        let segment_b = left_as_products[digits[4]] / (left_as_products[digits[1]] * segment_d);

        digits[0] = left_as_products
            .iter()
            .find_position(|&&product| product == left_as_products[digits[8]] / segment_d)
            .unwrap()
            .0;

        digits[2] = left_as_products
            .iter()
            .find_position(|&&product| {
                product == left_as_products[digits[8]] / (segment_b * segment_f)
            })
            .unwrap()
            .0;

        let mut value = 0;
        for (i, &digit) in self.right.iter().rev().enumerate() {
            let unique = str_to_unique(digit);
            let digit = left_as_products
                .iter()
                .find_position(|&&product| product == unique)
                .unwrap()
                .0;

            value +=
                10usize.pow(i as u32) * digits.iter().find_position(|&&d| d == digit).unwrap().0;
        }

        value
    }
}

impl<'a> From<&'a str> for Log<'a> {
    fn from(input: &'a str) -> Self {
        let mut spliterator = input.split('|');
        let left: Vec<&'a str> = spliterator.next().unwrap().split_whitespace().collect();
        let right: Vec<&'a str> = spliterator.next().unwrap().split_whitespace().collect();

        Self { left, right }
    }
}

const PRIMES: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];

#[inline(always)]
fn abcdefg_to_prime(letter: u8) -> u32 {
    PRIMES[(letter - b'a') as usize]
}

fn str_to_unique(s: &str) -> u32 {
    s.as_bytes().iter().copied().map(abcdefg_to_prime).product()
}

pub fn part_1(input: &str) -> usize {
    input.lines().map(|line| Log::from(line).part_1()).sum()
}

pub fn part_2(input: &str) -> usize {
    input.lines().map(|line| Log::from(line).part_2()).sum()
}

mod tests {
    #[test]
    fn part_2() {
        use super::Log;

        let log_str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |\
cdfeb fcadb cdfeb cdbaf";
        let log: Log = Log::from(log_str);

        assert_eq!(log.part_2(), 5353);
    }
}
