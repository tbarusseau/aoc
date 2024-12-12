#![allow(unused)]

use std::convert::TryFrom;

use tuple::T2;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    pub fn from_char(value: char) -> Self {
        match value {
            'U' | 'u' => Self::Up,
            'D' | 'd' => Self::Down,
            'R' | 'r' => Self::Right,
            'L' | 'l' => Self::Left,
            _ => panic!("invalid char for Direction: {}", value),
        }
    }
}

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
        }
    }
}

impl From<Direction> for T2<isize, isize> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (0, -1).into(),
            Direction::Right => (1, 0).into(),
            Direction::Left => (-1, 0).into(),
            Direction::Down => (0, 1).into(),
        }
    }
}

impl Direction {
    pub fn iterator() -> std::slice::Iter<'static, Self> {
        use Direction::{Down, Left, Right, Up};

        static DIRECTIONS: [Direction; 4] = [Up, Right, Down, Left];
        DIRECTIONS.iter()
    }

    pub fn turn_right(&mut self) {
        match self {
            Self::Up => *self = Self::Right,
            Self::Right => *self = Self::Down,
            Self::Down => *self = Self::Left,
            Self::Left => *self = Self::Up,
        }
    }

    pub fn turn_left(&mut self) {
        match self {
            Self::Up => *self = Self::Left,
            Self::Left => *self = Self::Down,
            Self::Down => *self = Self::Right,
            Self::Right => *self = Self::Up,
        }
    }

    pub fn get_delta(self) -> (i32, i32) {
        match self {
            Self::Up => (0, 1),
            Self::Right => (1, 0),
            Self::Left => (-1, 0),
            Self::Down => (0, -1),
        }
    }

    pub fn get_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn get_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn checked_offset(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        let (dx, dy) = self.get_delta();
        let (x, y) = (x as i32 + dx, y as i32 + dy);

        if x >= 0 && y >= 0 {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }

    pub fn checked_offset_with_dimensions(
        self,
        (x, y): (usize, usize),
        (width, height): (usize, usize),
    ) -> Option<(usize, usize)> {
        let (dx, dy) = self.get_delta();
        let (x, y) = (x as i32 + dx, y as i32 + dy);

        if x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' | 'u' => Ok(Self::Up),
            'D' | 'd' => Ok(Self::Down),
            'R' | 'r' => Ok(Self::Right),
            'L' | 'l' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
            Self::Down => write!(f, "Down"),
        }
    }
}
