use std::collections::HashMap;

use grid::Grid;
use itertools::Itertools;

use crate::utils::direction::Direction;

pub struct Day10;

crate::impl_day!("10", true);

fn process_input(input: &str) -> Grid<u32> {
    println!("Processing input");

    Grid::from_vec(
        input
            .trim()
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec(),
        input.lines().next().unwrap().len(),
    )
}

fn recursive(
    input: &Grid<u32>,
    pos: (usize, usize),
    visited_peaks: &mut HashMap<(usize, usize), i32>,
) {
    let height = input[(pos.1, pos.0)];

    if height == 9 {
        visited_peaks
            .entry(pos)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        return;
    }

    for dir in &[
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        if let Some(offset) = dir.checked_offset(pos) {
            if offset.0 >= input.cols() || offset.1 >= input.rows() {
                continue;
            }

            let offset_height = input[(offset.1, offset.0)];

            if offset_height != height + 1 {
                continue;
            }

            recursive(input, offset, visited_peaks);
        }
    }
}

fn count_trailheads(input: &Grid<u32>, starting_pos: (usize, usize), is_part2: bool) -> i32 {
    let mut visited_peaks: HashMap<(usize, usize), i32> = HashMap::new();

    recursive(input, starting_pos, &mut visited_peaks);

    if is_part2 {
        visited_peaks.values().copied().sum()
    } else {
        visited_peaks.len() as i32
    }
}

fn compute_result(input: &Grid<u32>, is_part2: bool) -> i32 {
    let mut res = 0;

    for y in 0..input.rows() {
        for x in 0..input.cols() {
            let v = input[(y, x)];

            if v != 0 {
                continue;
            }

            let count = count_trailheads(input, (x, y), is_part2);

            res += count;
        }
    }

    res
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new(compute_result(&input, false))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new(compute_result(&input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(36.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(81.to_string(), *solve_part2(INPUT).to_string());
    }
}
