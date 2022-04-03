use std::ops::Index;
use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    inner: [[(u8, Status); WIDTH]; HEIGHT],
    has_won: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Called,
    Uncalled,
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    pub fn play_turn(&mut self, draw: u8) -> Option<u32> {
        if self.has_won {
            return None;
        }

        self.mark(draw);

        if self.has_won() {
            self.has_won = true;
            return Some(self.internal_score() * (draw as u32));
        }

        None
    }
    pub fn mark(&mut self, draw: u8) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.inner[y][x].0 == draw {
                    self.inner[y][x].1 = Status::Called;
                }
            }
        }
    }

    pub fn has_won(&self) -> bool {
        if self.has_won {
            return true;
        }

        let mut complete_columns = [true; WIDTH];

        for y in 0..HEIGHT {
            let mut complete_row = true;
            for x in 0..WIDTH {
                let status = self.inner[y][x].1;
                if status == Status::Uncalled {
                    complete_row = false;
                    complete_columns[x] = false;
                }
            }
            if complete_row {
                return true;
            }
        }

        complete_columns.into_iter().any(|complete| complete)
    }

    pub fn internal_score(&self) -> u32 {
        self.inner
            .iter()
            .flatten()
            .copied()
            .filter(|(_, status)| *status == Status::Uncalled)
            .fold(0, |score, (value, _)| score + (value as u32))
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> From<&[u8]> for Board<WIDTH, HEIGHT> {
    fn from(slice: &[u8]) -> Self {
        if slice.len() != WIDTH * HEIGHT {
            panic!("Invalid board size");
        }

        let mut board = [[(0, Status::Uncalled); WIDTH]; HEIGHT];

        for (i, value) in slice.iter().copied().enumerate() {
            board[i / HEIGHT][i % WIDTH].0 = value;
        }

        Self {
            inner: board,
            has_won: false,
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for Board<WIDTH, HEIGHT> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut board = [[(0, Status::Uncalled); WIDTH]; HEIGHT];

        for (y, line) in s.lines().enumerate() {
            for (x, token) in line.split_whitespace().enumerate() {
                let value: u8 = token.parse()?;
                board[y][x].0 = value;
            }
        }

        Ok(Self {
            inner: board,
            has_won: false,
        })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<usize> for Board<WIDTH, HEIGHT> {
    type Output = [(u8, Status); WIDTH];

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

#[derive(Clone, Debug)]
pub struct Bingo<const BOARD_WIDTH: usize, const BOARD_HEIGHT: usize> {
    pub sequence: Vec<u8>,
    pub boards: Vec<Board<BOARD_WIDTH, BOARD_HEIGHT>>,
    pub turn: usize,
}

impl<const BOARD_WIDTH: usize, const BOARD_HEIGHT: usize> Bingo<BOARD_WIDTH, BOARD_HEIGHT> {
    fn play_turn(&mut self) -> Option<u32> {
        let draw = self.sequence[self.turn];
        self.turn += 1;

        for board in self.boards.iter_mut() {
            board.mark(draw);
        }

        self.boards
            .iter()
            .find(|board| board.has_won())
            .map(|board| board.internal_score() * (draw as u32))
    }

    pub fn play_game(&mut self) -> u32 {
        loop {
            if let Some(score) = self.play_turn() {
                return score;
            }
        }
    }

    pub fn play_until_last(&mut self) -> u32 {
        let mut boards_remaining = self.boards.len();
        let mut last_score = 0;

        for &draw in self.sequence.iter() {
            for board in self.boards.iter_mut() {
                if let Some(score) = board.play_turn(draw) {
                    last_score = score;
                }
            }
        }

        last_score
    }
}

impl<const BOARD_WIDTH: usize, const BOARD_HEIGHT: usize> FromStr
    for Bingo<BOARD_WIDTH, BOARD_HEIGHT>
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut spliterator = s.split("\n\n");

        let sequence: Vec<u8> = if let Some(sequence) = spliterator.next() {
            sequence
                .split(',')
                .map(|token| token.parse())
                .collect::<Result<_, _>>()?
        } else {
            bail!("Missing call sequence!");
        };

        let boards: Vec<Board<BOARD_WIDTH, BOARD_HEIGHT>> = spliterator
            .map(|board| board.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            sequence,
            boards,
            turn: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_str() {
        let board = "23 45\n 47 90";
        let board: Board<2, 2> = board.parse().unwrap();
        let expected: Board<2, 2> = Board {
            inner: [
                [(23, Status::Uncalled), (45, Status::Uncalled)],
                [(47, Status::Uncalled), (90, Status::Uncalled)],
            ],
            has_won: false,
        };
        assert_eq!(board, expected);
    }

    #[test]
    fn from_slice() {
        let board: [u8; 4] = [23, 45, 47, 90];
        let board: Board<2, 2> = board[..].into();
        let expected: Board<2, 2> = Board {
            inner: [
                [(23, Status::Uncalled), (45, Status::Uncalled)],
                [(47, Status::Uncalled), (90, Status::Uncalled)],
            ],
            has_won: false,
        };

        assert_eq!(board, expected);
    }

    #[test]
    fn index_works() {
        let board: [u8; 4] = [23, 45, 47, 90];
        let board: Board<2, 2> = board[..].into();

        assert_eq!(board[0][0].0, 23);
        assert_eq!(board[0][1].0, 45);
        assert_eq!(board[1][0].0, 47);
        assert_eq!(board[1][1].0, 90);
    }

    #[test]
    fn draw_value() {
        let board: [u8; 4] = [23, 45, 47, 90];
        let mut board: Board<2, 2> = board[..].into();
        board.mark(45);

        assert_eq!(board[0][1].1, Status::Called);
    }

    #[test]
    fn has_won_row() {
        let board: [u8; 4] = [23, 45, 47, 90];
        let mut board: Board<2, 2> = board[..].into();

        assert!(!board.has_won());

        board.mark(90);
        assert!(!board.has_won());

        board.mark(47);
        assert!(board.has_won());
    }

    #[test]
    fn has_won_column() {
        let board: [u8; 4] = [23, 45, 47, 90];
        let mut board: Board<2, 2> = board[..].into();

        assert!(!board.has_won());

        board.mark(45);
        assert!(!board.has_won());

        board.mark(90);
        assert!(board.has_won());
    }

    #[test]
    fn score() {
        let board: [u8; 4] = [23, 45, 47, 90];
        let mut board: Board<2, 2> = board[..].into();

        board.mark(45);

        board.mark(90);
        assert_eq!(board.internal_score(), 70);
    }
}
