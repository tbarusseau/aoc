use colored::Colorize;
use grid::Grid;
use regex::Regex;

use crate::solver::Solver;

pub struct Day5;

crate::impl_day!("5", true);

struct VentsLine {
    start: (i32, i32),
    end: (i32, i32),
}

impl VentsLine {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }
}

fn process_input(input: &str) -> Vec<VentsLine> {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let cap = re.captures(l).unwrap();

            VentsLine {
                start: (cap[2].parse().unwrap(), cap[1].parse().unwrap()),
                end: (cap[4].parse().unwrap(), cap[3].parse().unwrap()),
            }
        })
        .collect()
}

fn step_towards(pos: &mut (i32, i32), end: &(i32, i32)) {
    let mut multiplier = 1;

    if pos.0 == end.0 {
        if end.1 < pos.1 {
            multiplier = -1;
        }

        pos.1 += multiplier;
    } else if pos.1 == end.1 {
        if end.0 < pos.0 {
            multiplier = -1;
        }

        pos.0 += multiplier;
    } else {
        let x = if end.1 < pos.1 { -1 } else { 1 };
        let y = if end.0 < pos.0 { -1 } else { 1 };

        pos.0 += y;
        pos.1 += x;
    }
}

struct ResultGrid(Grid<i32>);

impl std::fmt::Display for ResultGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for y in 0..self.0.rows() {
            for x in 0..self.0.cols() {
                let n = self.0[x][y];
                if n == 0 {
                    s.push('.');
                } else if n >= 2 {
                    s.push_str(&format!("{}", n).blue());
                } else {
                    s.push_str(&format!("{}", n));
                }
            }

            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

#[allow(unused)]
fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut grid: Grid<i32> = Grid::new(1000, 1000);

    for line in input {
        if !line.is_horizontal() && !line.is_vertical() {
            continue;
        }

        grid[line.start.1 as usize][line.start.0 as usize] += 1;
        let mut pos = line.start;
        while pos != line.end {
            step_towards(&mut pos, &line.end);
            grid[pos.1 as usize][pos.0 as usize] += 1;
        }
    }

    let res = grid
        .iter()
        .fold(0, |acc, &n| if n >= 2 { acc + 1 } else { acc });
    Box::new(res)
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut grid: Grid<i32> = Grid::new(1000, 1000);

    for line in input {
        grid[line.start.1 as usize][line.start.0 as usize] += 1;
        let mut pos = line.start;
        while pos != line.end {
            step_towards(&mut pos, &line.end);
            grid[pos.1 as usize][pos.0 as usize] += 1;
        }
    }

    let res = grid
        .iter()
        .fold(0, |acc, &n| if n >= 2 { acc + 1 } else { acc });
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        assert_eq!(5.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(12.to_string(), *solve_part2(INPUT).to_string());
    }
}
