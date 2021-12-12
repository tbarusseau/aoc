use std::convert::TryFrom;

use crate::solver::Solver;

pub struct Day11;

crate::impl_day!("11", true);

fn process_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .replace('\n', "")
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect()
}

const NEIGHBOUR_INDICES: &[isize] = &[-11, -10, -9, -1, 1, 9, 10, 11];

fn increment_neighbours(input: &mut Vec<u32>, index: usize) {
    for &neighbour_offset in NEIGHBOUR_INDICES {
        if let Ok(i) = usize::try_from(index as isize + neighbour_offset) {
            input.get_mut(i).map(|e| {
                println!(
                    "Incrementing neighbour at index {} from {} to {}",
                    i,
                    *e,
                    *e + 1
                );
                *e += 1;
            });
        }
    }
}

fn step(input: &mut Vec<u32>, flashes: &mut i32) {
    // Increment all elements
    input.iter_mut().for_each(|n| *n += 1);

    for (index, n) in input.clone().iter().enumerate() {
        if *n > 9 {
            increment_neighbours(input, index);
        }
    }

    for n in input.iter_mut() {
        if *n > 9 {
            *n = 0;
            *flashes += 1;
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut input = process_input(input);
    let mut flashes = 0;

    let mut iter = input.chunks(10);
    println!("Before any steps:");
    while let Some(elems) = iter.next() {
        println!("{:?}", elems);
    }
    println!("");

    for i in 0..100 {
        step(&mut input, &mut flashes);

        let mut iter = input.chunks(10);
        println!("After step {}:", i + 1);
        while let Some(elems) = iter.next() {
            println!("{:?}", elems);
        }
        println!("");
    }

    Box::new(flashes)
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
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
