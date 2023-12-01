use std::collections::HashSet;

use crate::utils::{direction::Direction, manhattan::manhattan_distance};

pub struct Day1;

crate::impl_day!("1", true);

fn process_input(input: &str) -> Vec<&str> {
    input.trim().split(", ").collect::<Vec<&str>>()
}

fn process_instructions<F>(instructions: Vec<&str>, mut f: F) -> (i32, i32)
where
    F: FnMut((i32, i32), (i32, i32), &Direction),
{
    let mut pos = (0, 0);
    let mut direction = Direction::Up;

    for inst in instructions {
        let mut chars = inst.chars();
        let turn_right = chars.next().expect("invalid turn input") == 'R';
        let steps = i32::from_str_radix(chars.as_str(), 10).expect("invalid steps input");

        if turn_right {
            direction.turn_right();
        } else {
            direction.turn_left();
        }

        let last_pos = pos;

        match direction {
            Direction::Up => pos.1 += steps,
            Direction::Down => pos.1 -= steps,
            Direction::Right => pos.0 += steps,
            Direction::Left => pos.0 -= steps,
        }

        f(last_pos, pos, &direction);
    }

    pos
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let instructions = process_input(input);
    let pos = process_instructions::<_>(instructions, |_, _, _| {});

    Box::new(manhattan_distance((0, 0), (pos.0 as isize, pos.1 as isize)))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();
    let mut final_pos = None;

    let instructions = process_input(input);
    process_instructions(instructions, |last_pos, pos, dir| {
        let delta = dir.get_delta();
        let mut last_pos = last_pos;
        loop {
            if visited_pos.contains(&last_pos) && final_pos.is_none() {
                final_pos = Some(last_pos);
            }

            visited_pos.insert(last_pos);

            last_pos.0 += delta.0;
            last_pos.1 += delta.1;

            if last_pos == pos {
                break;
            }
        }
    });

    let final_pos = final_pos.expect("no valid final pos");

    Box::new(manhattan_distance(
        (0, 0),
        (final_pos.0 as isize, final_pos.1 as isize),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(5.to_string(), *solve_part1("R2, L3").to_string());
        assert_eq!(2.to_string(), *solve_part1("R2, R2, R2").to_string());
        assert_eq!(12.to_string(), *solve_part1("R5, L5, R5, R3").to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(4.to_string(), *solve_part2("R8, R4, R4, R8").to_string());
    }
}
