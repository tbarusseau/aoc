use std::collections::HashMap;

use crate::solver::Solver;

pub struct Day6;

crate::impl_day!("6", true);

struct Fishes(HashMap<usize, u64>);

fn process_input(input: &str) -> Fishes {
    let mut h = HashMap::new();

    input.trim().split(',').for_each(|e| {
        let n = e.parse::<usize>().unwrap();
        *h.entry(n).or_insert(0) += 1;
    });

    Fishes(h)
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut input = process_input(input);
    let mut day = 0;

    while day < 80 {
        let mut new_fishes = HashMap::new();

        input.0.iter().for_each(|(index, count)| {
            if *index == 0 {
                *new_fishes.entry(6).or_insert(0) += *count;
                *new_fishes.entry(8).or_insert(0) += *count;
            } else {
                *new_fishes.entry(index - 1).or_insert(0) += *count;
            }
        });

        input.0 = new_fishes;

        day += 1;
    }

    let res = input.0.iter().fold(0, |acc, e| acc + e.1);
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut input = process_input(input);
    let mut day = 0;

    while day < 256 {
        let mut new_fishes = HashMap::new();

        input.0.iter().for_each(|(index, count)| {
            if *index == 0 {
                *new_fishes.entry(6).or_insert(0) += *count;
                *new_fishes.entry(8).or_insert(0) += *count;
            } else {
                *new_fishes.entry(index - 1).or_insert(0) += *count;
            }
        });

        input.0 = new_fishes;

        day += 1;
    }

    let res = input.0.iter().fold(0, |acc, e| acc + e.1);
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(5934.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(26984457539_u64.to_string(), *solve_part2(INPUT).to_string());
    }
}
