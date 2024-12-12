use std::collections::HashSet;

use grid::Grid;
use itertools::Itertools;

use crate::utils::direction::Direction;

pub struct Day12;

crate::impl_day!("12", true);

type Area = usize;
type Perimeter = usize;
type Sides = usize;
#[allow(unused)]
struct Region(char, Area, Perimeter, Sides);

fn explore_region_rec(
    input_grid: &Grid<char>,
    pos: (usize, usize),
    region_char: char,
    visited_cells: &mut HashSet<(usize, usize)>,
    rec_visited_cells: &mut HashSet<(usize, usize)>,
) {
    let current_char = input_grid[(pos.1, pos.0)];
    if current_char != region_char {
        return;
    }

    if rec_visited_cells.contains(&(pos.0, pos.1)) {
        assert!(visited_cells.contains(&(pos.0, pos.1)));
        return;
    }

    visited_cells.insert((pos.0, pos.1));
    rec_visited_cells.insert((pos.0, pos.1));

    for &dir in Direction::iterator() {
        let maybe_offset =
            dir.checked_offset_with_dimensions(pos, (input_grid.rows(), input_grid.cols()));
        if let Some(offset) = maybe_offset {
            let neighbour_char = input_grid[(offset.1, offset.0)];
            if neighbour_char != region_char {
                continue;
            }

            explore_region_rec(
                input_grid,
                offset,
                region_char,
                visited_cells,
                rec_visited_cells,
            );
        }
    }
}

#[allow(clippy::too_many_lines)]
fn explore_region(
    input_grid: &Grid<char>,
    pos: (usize, usize),
    visited_cells: &mut HashSet<(usize, usize)>,
) -> Region {
    let region_char = input_grid[(pos.1, pos.0)];
    let mut rec_visited_cells: HashSet<(usize, usize)> = HashSet::new();

    explore_region_rec(
        input_grid,
        pos,
        region_char,
        visited_cells,
        &mut rec_visited_cells,
    );

    let width = input_grid.cols();
    let height = input_grid.rows();

    // Compute perimeter
    let mut perimeter = 0;

    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);

            if !rec_visited_cells.contains(&pos) {
                continue;
            }

            let mut count = 0;

            for dir in Direction::iterator() {
                if let Some(neighbour_pos) =
                    dir.checked_offset_with_dimensions((x, y), (width, height))
                {
                    let neighbour_char = input_grid[(neighbour_pos.1, neighbour_pos.0)];
                    if neighbour_char != region_char {
                        count += 1;
                    }
                } else {
                    count += 1;
                }
            }

            perimeter += count;
        }
    }

    // Compute sides (or corners)
    let mut corners = 0;

    for y in 0..height {
        for x in 0..width {
            if !rec_visited_cells.contains(&(x, y)) {
                continue;
            }

            // Are neighbours the same?
            let [ul, u, ur, l, r, dl, d, dr] = [
                (-1, 1),
                (0, 1),
                (1, 1),
                (-1, 0),
                (1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ]
            .iter()
            .map(|(dx, dy)| {
                let offset = (x as i32 - *dx, y as i32 - *dy);
                if offset.0 < 0
                    || offset.0 as usize >= width
                    || offset.1 < 0
                    || offset.1 as usize >= height
                {
                    false
                } else {
                    rec_visited_cells.contains(&(offset.0 as usize, offset.1 as usize))
                        && region_char == input_grid[(offset.1 as usize, offset.0 as usize)]
                }
            })
            .collect_vec()[..] else {
                unreachable!()
            };

            if (!u && !l) || (u && l && !ul) {
                corners += 1;
            }
            if (!u && !r) || (u && r && !ur) {
                corners += 1;
            }
            if (!r && !d) || (r && d && !dr) {
                corners += 1;
            }
            if (!d && !l) || (d && l && !dl) {
                corners += 1;
            }
        }
    }

    Region(region_char, rec_visited_cells.len(), perimeter, corners)
}

fn process_input(input: &str) -> Vec<Region> {
    let input_grid: Grid<char> = Grid::from_vec(
        input
            .trim()
            .lines()
            .flat_map(|l| l.chars().collect_vec())
            .collect_vec(),
        input.lines().next().unwrap().len(),
    );

    let mut visited_cells: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = vec![];

    for y in 0..input_grid.cols() {
        for x in 0..input_grid.rows() {
            if visited_cells.contains(&(x, y)) {
                continue;
            }

            regions.push(explore_region(&input_grid, (x, y), &mut visited_cells));
        }
    }

    regions
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new(input.iter().fold(0, |acc, Region(_, area, perimeter, _)| {
        acc + area * perimeter
    }))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new(
        input
            .iter()
            .fold(0, |acc, Region(_, area, _, sides)| acc + area * sides),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(1930.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1206.to_string(), *solve_part2(INPUT).to_string());
    }
}
