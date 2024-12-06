use std::collections::HashSet;

use grid::Grid;
use itertools::Itertools;

use crate::utils::direction::Direction;

pub struct Day6;

crate::impl_day!("6", true);

#[derive(PartialEq, Eq)]
enum Outcome {
    Running,
    Blocked,
    Outside,
    Loop,
}

fn process_input(input: &str) -> ((isize, isize), Direction, Grid<bool>) {
    let input = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let width = input[0].len();
    let height = input.len();

    let mut dir = Direction::Up;
    let mut pos = (0, 0);

    for (y, row) in input.iter().enumerate().take(height) {
        for (x, &c) in row.iter().enumerate().take(width) {
            let x = x as isize;
            let y = y as isize;

            match c {
                '^' => {
                    dir = Direction::Up;
                    pos = (x, y);
                }
                'v' => {
                    dir = Direction::Down;
                    pos = (x, y);
                }
                '>' => {
                    dir = Direction::Right;
                    pos = (x, y);
                }
                '<' => {
                    dir = Direction::Left;
                    pos = (x, y);
                }
                _ => {}
            };
        }
    }

    let grid = Grid::from_vec(
        input
            .iter()
            .flat_map(|v| v.iter().map(|&c| c == '#').collect_vec())
            .collect_vec(),
        width,
    );

    (pos, dir, grid)
}

fn step(
    pos: &mut (isize, isize),
    dir: &mut Direction,
    grid: &Grid<bool>,
    visited_pos: &mut HashSet<((isize, isize), Direction)>,
) -> Outcome {
    let offset: (isize, isize) = (*dir).into();
    let next_pos = (pos.0 + offset.0, pos.1 + offset.1);

    if let Some(&next_is_obstacle) = grid.get(next_pos.1, next_pos.0) {
        if next_is_obstacle {
            dir.turn_right();
            return Outcome::Blocked;
        }

        if visited_pos.contains(&(*pos, *dir)) {
            return Outcome::Loop;
        }

        visited_pos.insert((*pos, *dir));
        *pos = next_pos;

        return Outcome::Running;
    }

    Outcome::Outside
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let (mut pos, mut dir, grid) = process_input(input);
    let mut visited_pos = HashSet::new();

    loop {
        let outcome = step(&mut pos, &mut dir, &grid, &mut visited_pos);
        assert!(outcome != Outcome::Loop);
        if outcome == Outcome::Outside {
            break;
        }
    }

    Box::new(visited_pos.iter().map(|(pos, _)| pos).unique().count() + 1)
}

fn is_a_loop(starting_pos: (isize, isize), starting_dir: Direction, grid: &Grid<bool>) -> bool {
    let mut visited_pos: HashSet<((isize, isize), Direction)> = HashSet::new();
    let mut pos = starting_pos;
    let mut dir = starting_dir;

    loop {
        let outcome = step(&mut pos, &mut dir, grid, &mut visited_pos);
        if outcome == Outcome::Loop {
            return true;
        }
        if outcome == Outcome::Outside {
            return false;
        }
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let (starting_pos, starting_dir, grid) = process_input(input);

    let mut res = 0;
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            // Insert a new obstacle
            let mut new_grid = grid.clone();
            let r = new_grid.get_mut(y, x).unwrap();
            *r = true;

            if is_a_loop(starting_pos, starting_dir, &new_grid) {
                res += 1;
            }
        }
    }

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(41.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(6.to_string(), *solve_part2(INPUT).to_string());
    }
}
