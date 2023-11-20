use std::convert::TryFrom;

use lazy_static::lazy_static;
use regex::Regex;

use crate::solver::Solver;

pub struct Day8;

crate::impl_day!("8", true);

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

lazy_static! {
    static ref RE_RECT: Regex = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
    static ref RE_ROTATE_ROW: Regex = Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();
    static ref RE_ROTATE_COLUMN: Regex = Regex::new(r"^rotate row y=(\d+) by (\d+)$").unwrap();
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if RE_RECT.is_match(value) {
            let captures = RE_RECT.captures(value).ok_or_else(|| ())?;

            let x = usize::from_str_radix(captures.get(1).ok_or_else(|| ())?.as_str(), 10)?;
            let y = usize::from_str_radix(captures.get(2).ok_or_else(|| ())?.as_str(), 10)?;

            Ok(Instruction::Rect(x, y))
        } else if RE_ROTATE_ROW.is_match(value) {
        } else if RE_ROTATE_COLUMN.is_match(value) {
        } else {
            Err(())
        }
    }
}

fn process_input(input: &str) -> Vec<Instruction> {
    input
}

#[allow(unused)]
fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = "Part 1 not done";
    Box::new(res)
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = "Part 2 not done";
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test_part1() {
        assert_eq!(0.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
