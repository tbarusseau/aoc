use std::collections::HashSet;

use crate::solver::Solver;

pub struct Day11;

crate::impl_day!("11", true);

fn process_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

const NEIGHBOUR_INDICES: &[(isize, isize)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn modify_neighbours(input: &mut [Vec<u32>], pos: (isize, isize), f: fn(&mut u32)) {
    let (x, y) = pos;

    for (xi, yi) in NEIGHBOUR_INDICES {
        if let Some(l) = input.get_mut((y + yi) as usize) {
            if let Some(v) = l.get_mut((x + xi) as usize) {
                f(v);
            }
        }
    }
}

fn step(input: &mut [Vec<u32>]) -> usize {
    let mut flashes = 0;

    // Increment all energy levels
    for row in input.iter_mut().take(10) {
        for v in row.iter_mut().take(10) {
            *v += 1;
        }
    }

    // Flash octopus
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    loop {
        let mut mutated = false;

        for y in 0..10 {
            for x in 0..10 {
                let v = input[y][x];
                if v > 9 {
                    let pos = (x, y);

                    if flashed.contains(&pos) {
                        continue;
                    }

                    mutated = true;

                    // Flash current octopus, increment neighbours
                    flashed.insert(pos);
                    flashes += 1;
                    modify_neighbours(input, (pos.0 as isize, pos.1 as isize), |e| {
                        *e += 1;
                    });
                }
            }
        }

        if !mutated {
            break;
        }
    }

    // Reset flashed octopus
    for (x, y) in flashed {
        input[y][x] = 0;
    }

    flashes
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut input = process_input(input);
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step(&mut input);
    }

    Box::new(flashes)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut input = process_input(input);
    let mut steps = 0;

    loop {
        step(&mut input);
        steps += 1;

        if input.iter().flatten().filter(|&v| *v == 0).count() == 100 {
            break;
        }
    }

    Box::new(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_part1() {
        assert_eq!(1656.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(195.to_string(), *solve_part2(INPUT).to_string());
    }
}
