use std::collections::HashSet;

use grid::Grid;
use itertools::Itertools;

pub struct Day8;

crate::impl_day!("8", true);

fn process_input(input: &str) -> Grid<Option<char>> {
    let cols = input.trim().lines().next().unwrap().len();

    Grid::from_vec(
        input
            .trim()
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|c| if c == '.' { None } else { Some(c) })
                    .collect_vec()
            })
            .collect_vec(),
        cols,
    )
}

fn get_all_pos_for_char(input: &Grid<Option<char>>, c: char) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .filter_map(|(index, cell)| {
            if *cell == Some(c) {
                Some((index % input.cols(), index / input.cols()))
            } else {
                None
            }
        })
        .collect_vec()
}

fn process_current_pos(
    input: &Grid<Option<char>>,
    (x, y): (usize, usize),
    h: &mut HashSet<(usize, usize)>,
    is_part2: bool,
) {
    let current = input[(y, x)];

    if let Some(c) = current {
        let all_pos = get_all_pos_for_char(input, c);
        let filtered_pos = all_pos
            .iter()
            .filter(|p| p.1 != y && p.0 != x)
            .collect_vec();

        if !filtered_pos.is_empty() {
            h.insert((x, y));
        }

        for pos in filtered_pos {
            let offset = (pos.0 as isize - x as isize, pos.1 as isize - y as isize);
            let mut antinode_pos = (x as isize - offset.0, y as isize - offset.1);

            if antinode_pos.0.is_negative()
                || antinode_pos.1.is_negative()
                || antinode_pos.0 as usize >= input.cols()
                || antinode_pos.1 as usize >= input.rows()
            {
                continue;
            }

            h.insert((antinode_pos.0 as usize, antinode_pos.1 as usize));

            if is_part2 {
                while !antinode_pos.0.is_negative()
                    && !antinode_pos.1.is_negative()
                    && (antinode_pos.0 as usize) < input.cols()
                    && (antinode_pos.1 as usize) < input.rows()
                {
                    h.insert((antinode_pos.0 as usize, antinode_pos.1 as usize));

                    antinode_pos = (antinode_pos.0 - offset.0, antinode_pos.1 - offset.1);
                }
            }
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut h: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..input.rows() {
        for x in 0..input.cols() {
            process_current_pos(&input, (x, y), &mut h, false);
        }
    }

    let res = h.len();
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut h: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..input.rows() {
        for x in 0..input.cols() {
            process_current_pos(&input, (x, y), &mut h, true);
        }
    }

    let res = h.len();
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(14.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(34.to_string(), *solve_part2(INPUT).to_string());
    }
}
