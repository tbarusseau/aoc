use std::convert::TryFrom;

use itertools::Itertools;

use crate::utils::direction::Direction;

pub struct Day2;

crate::impl_day!("2", true);

fn process_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().flat_map(Direction::try_from).collect())
        .collect_vec()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut result: i32 = 0;
    let mut pos: (i32, i32) = (0, 0);
    let lines = process_input(input);

    for line in lines {
        for dir in line {
            let delta = dir.get_delta();

            pos.0 += delta.0;
            pos.1 += delta.1;

            pos.0 = pos.0.clamp(-1, 1);
            pos.1 = pos.1.clamp(-1, 1);
        }

        let digit = match pos {
            (-1, 1) => 1,
            (0, 1) => 2,
            (1, 1) => 3,
            (-1, 0) => 4,
            (0, 0) => 5,
            (1, 0) => 6,
            (-1, -1) => 7,
            (0, -1) => 8,
            (1, -1) => 9,
            _ => panic!("invalid keypad position: {:?}", pos),
        };

        result *= 10;
        result += digit;
    }

    Box::new(result)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    const VALID_POSITIONS: &[(i32, i32)] = &[
        (0, 2),
        (-1, 1),
        (0, 1),
        (1, 1),
        (-2, 0),
        (-1, 0),
        (0, 0),
        (1, 0),
        (2, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (0, -2),
    ];

    let mut result: String = String::new();
    let mut pos: (i32, i32) = (-2, 0);
    let lines = process_input(input);

    for line in lines {
        for dir in line {
            let delta = dir.get_delta();

            let mut new_pos = pos;

            new_pos.0 += delta.0;
            new_pos.1 += delta.1;

            if VALID_POSITIONS.contains(&new_pos) {
                pos = new_pos;
            }
        }

        let c = match pos {
            (0, 2) => '1',
            (-1, 1) => '2',
            (0, 1) => '3',
            (1, 1) => '4',
            (-2, 0) => '5',
            (-1, 0) => '6',
            (0, 0) => '7',
            (1, 0) => '8',
            (2, 0) => '9',
            (-1, -1) => 'A',
            (0, -1) => 'B',
            (1, -1) => 'C',
            (0, -2) => 'D',
            _ => panic!("invalid keypad position: {:?}", pos),
        };

        result.push(c);
    }

    Box::new(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"ULL
RRDDD
LURDL
UUUUD
";

    #[test]
    fn test_part1() {
        assert_eq!(1985.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!("5DB3".to_string(), *solve_part2(INPUT).to_string());
    }
}
