use std::str::FromStr;

use anyhow::bail;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Bracket {
    Left(BracketType),
    Right(BracketType),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BracketType {
    Parens,
    Square,
    Curly,
    Angle,
}

impl BracketType {
    fn value_part1(&self) -> u64 {
        match self {
            Self::Parens => 3,
            Self::Square => 57,
            Self::Curly => 1197,
            Self::Angle => 25137,
        }
    }
    fn value_part2(&self) -> u64 {
        match self {
            Self::Parens => 1,
            Self::Square => 2,
            Self::Curly => 3,
            Self::Angle => 4,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line(Vec<Bracket>);

impl Line {
    pub fn score(&self) -> u64 {
        let mut stack: Vec<BracketType> = Vec::with_capacity(self.0.len());

        for &bracket in self.0.iter() {
            match bracket {
                Bracket::Left(left) => stack.push(left),
                Bracket::Right(right) => {
                    if let Some(left) = stack.pop() {
                        if left != right {
                            return right.value_part1();
                        }
                    } else {
                        return right.value_part1();
                    }
                }
            }
        }

        0
    }

    pub fn completion_score(&self) -> u64 {
        let mut stack: Vec<BracketType> = Vec::with_capacity(self.0.len());

        for &bracket in self.0.iter() {
            match bracket {
                Bracket::Left(left) => stack.push(left),
                Bracket::Right(right) => {
                    if let Some(left) = stack.pop() {
                        if left != right {
                            return 0;
                        }
                    } else {
                        return 0;
                    }
                }
            }
        }

        stack.iter().rev().fold(0, |score, &bracket_type| {
            score * 5 + bracket_type.value_part2()
        })
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        use Bracket::{Left, Right};
        use BracketType::{Angle, Curly, Parens, Square};

        let line = s
            .chars()
            .map(|c| match c {
                '(' => Ok(Left(Parens)),
                ')' => Ok(Right(Parens)),
                '[' => Ok(Left(Square)),
                ']' => Ok(Right(Square)),
                '{' => Ok(Left(Curly)),
                '}' => Ok(Right(Curly)),
                '<' => Ok(Left(Angle)),
                '>' => Ok(Right(Angle)),
                _ => bail!("Invalid bracket"),
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self(line))
    }
}

pub struct Lines(Vec<Line>);

impl Lines {
    pub fn score(&self) -> u64 {
        self.0.iter().map(|line| line.score()).sum()
    }

    pub fn completion_score(&self) -> u64 {
        let scores: Vec<u64> = self
            .0
            .iter()
            .map(|line| line.completion_score())
            .filter(|&score| score != 0)
            .sorted()
            .collect();

        scores[scores.len() / 2]
    }
}

impl From<Vec<Line>> for Lines {
    fn from(lines: Vec<Line>) -> Self {
        Self(lines)
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::Bracket::{Left, Right};
    use crate::day10::BracketType::{Angle, Curly, Parens, Square};

    use super::*;

    #[test]
    fn can_parse_line() {
        let line = "(<{[]}>)";
        let expected = Line(vec![
            Left(Parens),
            Left(Angle),
            Left(Curly),
            Left(Square),
            Right(Square),
            Right(Curly),
            Right(Angle),
            Right(Parens),
        ]);

        assert_eq!(line.parse::<Line>().unwrap(), expected);
    }

    #[test]
    fn complete_line_returns_zero() {
        let line: Line = "(<{[]}>)".parse().unwrap();

        assert_eq!(line.score(), 0);
    }

    #[test]
    fn syntax_error_returns_score() {
        let line: Line = ")".parse().unwrap();
        assert_eq!(line.score(), 3);

        let line: Line = "(){[}".parse().unwrap();
        assert_eq!(line.score(), 1197);
    }

    #[test]
    fn completion_score() {
        let line_1: Line = "[({(<(())[]>[[{[]{<()<>>".parse().unwrap();
        let line_2: Line = "[(()[<>])]({[<{<<[]>>(".parse().unwrap();
        let line_3: Line = "(((({<>}<{<{<>}{[]{[]{}".parse().unwrap();
        let line_4: Line = "{<[[]]>}<{[{[{[]{()[[[]".parse().unwrap();
        let line_5: Line = "<{([{{}}[<[[[<>{}]]]>[]]".parse().unwrap();

        assert_eq!(line_1.completion_score(), 288957);
        assert_eq!(line_2.completion_score(), 5566);
        assert_eq!(line_3.completion_score(), 1480781);
        assert_eq!(line_4.completion_score(), 995444);
        assert_eq!(line_5.completion_score(), 294);
    }
}
