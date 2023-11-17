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
    pub fn iterator() -> std::slice::Iter<'static, Direction> {
        use Direction::*;

        static DIRECTIONS: [Direction; 4] = [Up, Right, Left, Down];
        DIRECTIONS.iter()
    }

    pub fn turn_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }

    pub fn turn_left(&mut self) {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Left => *self = Direction::Down,
            Direction::Down => *self = Direction::Right,
            Direction::Right => *self = Direction::Up,
        }
    }

    pub fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Down => (0, -1),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' | 'u' => Ok(Direction::Up),
            'D' | 'd' => Ok(Direction::Down),
            'R' | 'r' => Ok(Direction::Right),
            'L' | 'l' => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}
