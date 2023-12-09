use std::borrow::Cow;

use itertools::Itertools;

pub struct Day9;

crate::impl_day!("9", true);

fn process_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim_end()
        .lines()
        .map(|l| l.split_whitespace().flat_map(str::parse).collect_vec())
        .collect_vec()
}

fn compute_value(input: &[i32], previous: bool) -> i32 {
    let mut sequences = vec![];

    sequences.push(input.to_owned());
    let mut last_sequence = Cow::Borrowed(input);

    loop {
        let new_sequence = last_sequence
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        sequences.push(new_sequence.clone());
        last_sequence = Cow::Owned(new_sequence);

        if last_sequence.iter().all(num::Zero::is_zero) {
            break;
        }
    }

    sequences.iter().rev().fold(0, |acc, s| {
        if previous {
            s.first().expect("no last element") - acc
        } else {
            acc + s.last().expect("no last element")
        }
    })
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input.iter().fold(0, |acc, v| acc + compute_value(v, false));

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input.iter().fold(0, |acc, v| acc + compute_value(v, true));

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(114.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2.to_string(), *solve_part2(INPUT).to_string());
    }
}
