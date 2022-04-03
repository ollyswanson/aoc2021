use std::collections::HashMap;
use std::str::FromStr;

use anyhow::bail;
use nom::character::complete::multispace0;
use nom::sequence::preceded;
use nom::{
    bytes::complete::tag,
    character::complete::char,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::separated_pair,
    Finish, IResult, Parser,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Point {
    fn from(point: (i32, i32)) -> Self {
        Self {
            x: point.0,
            y: point.1,
        }
    }
}

impl std::ops::Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

fn parse_int(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse).parse(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(separated_pair(parse_int, char(','), parse_int), Point::from).parse(input)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn gradient(&self) -> (i32, i32) {
        let gradient = (self.end.x - self.start.x, self.end.y - self.start.y);
        let hcf = num::integer::gcd(gradient.0, gradient.1);

        (gradient.0 / hcf, gradient.1 / hcf)
    }

    fn iter(&self) -> LineIter {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = Point;
    type IntoIter = LineIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LineIter {
            finished: false,
            gradient: self.gradient(),
            current: self.start,
            line: self,
        }
    }
}

pub struct LineIter<'a> {
    finished: bool,
    gradient: (i32, i32),
    current: Point,
    line: &'a Line,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if self.current == self.line.end {
            self.finished = true;
        }

        let point = self.current;
        self.current = self.current + self.gradient;

        Some(point)
    }
}

impl From<(Point, Point)> for Line {
    fn from(points: (Point, Point)) -> Self {
        Self {
            start: points.0,
            end: points.1,
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    map(
        preceded(
            multispace0,
            separated_pair(parse_point, tag(" -> "), parse_point),
        ),
        Line::from,
    )
    .parse(input)
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match parse_line(s).finish() {
            Ok((_, line)) => Ok(line),
            Err(_) => bail!("Failed to parse"),
        }
    }
}

pub struct Lines(Vec<Line>);

impl Lines {
    pub fn dangerous_points_horizontal_or_vert(&self) -> usize {
        let mut grid: HashMap<Point, u32> = HashMap::new();
        for line in self
            .0
            .iter()
            .filter(|line| matches!(line.gradient(), (0, 1) | (1, 0) | (-1, 0) | (0, -1)))
        {
            for point in line.iter() {
                let entry = grid.entry(point).or_insert(0);
                *entry += 1;
            }
        }

        grid.iter().filter(|(_, count)| **count > 1).count()
    }

    pub fn dangerous_points_all(&self) -> usize {
        let mut grid: HashMap<Point, u32> = HashMap::new();
        for line in self.0.iter() {
            for point in line.iter() {
                let entry = grid.entry(point).or_insert(0);
                *entry += 1;
            }
        }

        grid.iter().filter(|(_, count)| **count > 1).count()
    }
}

impl From<Vec<Line>> for Lines {
    fn from(lines: Vec<Line>) -> Self {
        Self(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_line() {
        let line = "5,4 -> 7,8";
        let expected = Line {
            start: Point { x: 5, y: 4 },
            end: Point { x: 7, y: 8 },
        };

        assert_eq!(line.parse::<Line>().unwrap(), expected);
    }

    #[test]
    fn iter_line_grad_is_1() {
        let line: Line = "0,0 -> 5,5".parse().unwrap();
        let mut line_iter = line.iter();

        assert_eq!(line_iter.next(), Some(Point::new(0, 0)));
        assert_eq!(line_iter.next(), Some(Point::new(1, 1)));
        assert_eq!(line_iter.next(), Some(Point::new(2, 2)));
        assert_eq!(line_iter.next(), Some(Point::new(3, 3)));
        assert_eq!(line_iter.next(), Some(Point::new(4, 4)));
        assert_eq!(line_iter.next(), Some(Point::new(5, 5)));
        assert_eq!(line_iter.next(), None);
    }

    #[test]
    fn iter_line_grad_2() {
        let line: Line = "0,0 -> 10,5".parse().unwrap();
        let mut line_iter = line.iter();
        assert_eq!(line_iter.next(), Some(Point::new(0, 0)));
        assert_eq!(line_iter.next(), Some(Point::new(2, 1)));
        assert_eq!(line_iter.next(), Some(Point::new(4, 2)));
        assert_eq!(line_iter.next(), Some(Point::new(6, 3)));
        assert_eq!(line_iter.next(), Some(Point::new(8, 4)));
        assert_eq!(line_iter.next(), Some(Point::new(10, 5)));
        assert_eq!(line_iter.next(), None);
    }

    #[test]
    fn count_dangerous_points() {
        let lines = "\
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2\
        ";

        let lines: Lines = lines
            .lines()
            .map(str::parse)
            .collect::<anyhow::Result<Vec<_>>>()
            .unwrap()
            .into();

        assert_eq!(lines.dangerous_points_horizontal_or_vert(), 5);
    }
}
