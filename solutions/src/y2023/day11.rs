use std::collections::HashSet;

use itertools::Itertools;

pub struct Day11;

crate::impl_day!("11", true);

fn process_input(input: &str) -> Vec<Vec<bool>> {
    input
        .trim_end()
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect_vec())
        .collect_vec()
}

fn collect_galaxies(input: &[Vec<bool>]) -> HashSet<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, galaxy_present)| {
                    if *galaxy_present {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

fn build_distances_map(input: &[Vec<bool>], expansion_rate: usize) -> Vec<Vec<usize>> {
    let input_height = input.len();
    let input_width = input[0].len();

    let mut distances_map = vec![vec![1; input_width]; input_height];

    (0..input_height).for_each(|y| {
        let row = &input[y];
        let row_empty = row.iter().all(|v| !v);

        if row_empty {
            (0..input_width).for_each(|x| distances_map[y][x] *= expansion_rate);
        }
    });

    (0..input_width).for_each(|x| {
        let col_empty = (0..input_height).all(|y| !input[y][x]);
        if col_empty {
            (0..input_height).for_each(|y| distances_map[y][x] *= expansion_rate);
        }
    });

    distances_map
}

fn galactic_manhattan_distance(
    a: (usize, usize),
    b: (usize, usize),
    distances_map: &[Vec<usize>],
) -> usize {
    let mut dist = 0;

    let a = (a.0 as isize, a.1 as isize);
    let b = (b.0 as isize, b.1 as isize);

    let mut curr_pos = a;
    let mut offset: (isize, isize) = if a.0 < b.0 { (1, 0) } else { (-1, 0) };

    while curr_pos != b {
        if curr_pos.0 == b.0 {
            offset = if a.1 < b.1 { (0, 1) } else { (0, -1) };
        }

        curr_pos = (curr_pos.0 + offset.0, curr_pos.1 + offset.1);
        dist += distances_map[curr_pos.1 as usize][curr_pos.0 as usize];
    }

    dist
}

fn solve(input: &str, expansion_rate: usize) -> usize {
    let input = process_input(input);
    let distances_map = build_distances_map(&input, expansion_rate);
    let galaxies = collect_galaxies(&input);

    galaxies.iter().permutations(2).fold(0, |acc, v| {
        let a = v[0];
        let b = v[1];

        let dist = galactic_manhattan_distance(*a, *b, &distances_map);

        acc + dist
    }) / 2
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(solve(input, 2))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(solve(input, 1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_part1() {
        assert_eq!(374.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1030, solve(INPUT, 10));
        assert_eq!(8410, solve(INPUT, 100));
    }
}
