#![allow(unused)]

use std::convert::TryFrom;

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    pub fn iterator() -> std::slice::Iter<'static, Self> {
        use Direction::{Down, Left, Right, Up};

        static DIRECTIONS: [Direction; 4] = [Up, Right, Left, Down];
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

    pub fn get_delta(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, 1),
            Self::Right => (1, 0),
            Self::Left => (-1, 0),
            Self::Down => (0, -1),
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
