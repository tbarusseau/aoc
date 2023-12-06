use std::sync::OnceLock;

use itertools::Itertools;
use regex::Regex;

pub struct Day6;

crate::impl_day!("6", true);

static RE: OnceLock<Regex> = OnceLock::new();

fn process_input(input: &str) -> Vec<(i64, i64)> {
    let re = RE.get_or_init(|| Regex::new(r"(\d+)\s*").unwrap());

    let r: Vec<Vec<i64>> = input
        .trim_end()
        .lines()
        .map(|l| {
            re.captures_iter(l)
                .filter_map(|c| c.get(1).map(|s| s.as_str().parse::<i64>().unwrap()))
                .collect_vec()
        })
        .collect_vec();

    let v1 = &r[0];
    let v2 = &r[1];

    assert!(v1.len() == v2.len());

    v1.iter().copied().zip(v2.iter().copied()).collect_vec()
}

fn count_ways_to_win(time: i64, distance: i64) -> i64 {
    (0..time).fold(0, |acc, i| {
        let speed = i;
        let remaining_time = time - i;

        let result = speed * remaining_time;

        if result > distance {
            acc + 1
        } else {
            acc
        }
    })
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input
        .iter()
        .fold(1, |acc, v| acc * count_ways_to_win(v.0, v.1));

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(
        &input
            .chars()
            .filter(|c| c.is_numeric() || *c == '\n')
            .collect::<String>(),
    );

    println!("{input:?}");

    let res = input
        .iter()
        .fold(1, |acc, v| acc * count_ways_to_win(v.0, v.1));

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_part1() {
        assert_eq!(288.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(71503.to_string(), *solve_part2(INPUT).to_string());
    }
}
