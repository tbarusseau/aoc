#![allow(unused)]

use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use regex::Regex;

use crate::utils::matrix::Matrix;

pub struct Day20;

crate::impl_day!("20", true);

struct Tile {
    pub id: i32,
    pub tile: Matrix<bool>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;

        for l in &self.tile.0 {
            for b in l {
                if *b {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn process_input(input: &str) -> Vec<Tile> {
    let re = Regex::new(r"^Tile (\d+):$").expect("Invalid regex");
    let mut tiles = Vec::with_capacity(144);

    for split in input.split("\n\n") {
        let mut lines = split.lines();

        if let Some(first_line) = lines.next() {
            let rest: Vec<&str> = lines.collect();

            let id_str = re
                .captures(first_line)
                .expect("doesn't match regex")
                .get(1)
                .expect("no capture group at index 1")
                .as_str();
            let id = id_str.parse().expect("tile id is not a valid i32");

            let tile: Matrix<bool> = rest
                .into_iter()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect::<Vec<Vec<bool>>>()
                .into();

            let tile = Tile { id, tile };
            tiles.push(tile);
        }
    }

    tiles
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut borders: HashMap<i32, Vec<Vec<bool>>> = HashMap::new();
    let tile_len = input[0].tile.0.len();

    for tile in input {
        let key = tile.id;
        let mut v: Vec<Vec<bool>> = vec![Vec::with_capacity(10); 4];

        v.push(tile.tile.0[0].clone());
        v.push(tile.tile.0[tile_len - 1].clone());
    }

    Box::new("Part 1 not done")
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new("Part 2 not done")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...

";

    #[test]
    fn test_part1() {
        assert_eq!(
            20_899_048_083_289_i64.to_string(),
            *solve_part1(INPUT).to_string()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
