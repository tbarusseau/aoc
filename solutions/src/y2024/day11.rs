use std::collections::HashMap;

use itertools::Itertools;

pub struct Day11;

crate::impl_day!("11", true);

fn process_input(input: &str) -> HashMap<u64, u64> {
    let nums = input.trim().split(' ').flat_map(str::parse).collect_vec();

    let mut h = HashMap::new();

    for n in nums {
        h.insert(n, 1);
    }

    h
}

fn blink(h: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_h = HashMap::new();

    for (&k, &v) in h {
        if k == 0 {
            new_h.entry(1).and_modify(|e| *e += v).or_insert(v);
        } else if (k.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
            let as_str = k.to_string();
            let first_half = as_str
                .chars()
                .take(as_str.len() / 2)
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let second_half = as_str
                .chars()
                .skip(as_str.len() / 2)
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            new_h.entry(first_half).and_modify(|e| *e += v).or_insert(v);
            new_h
                .entry(second_half)
                .and_modify(|e| *e += v)
                .or_insert(v);
        } else {
            new_h.entry(k * 2024).and_modify(|e| *e += v).or_insert(v);
        }
    }

    new_h
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut stones = process_input(input);

    for _ in 0..25 {
        stones = blink(&stones);
    }

    Box::new(stones.values().sum::<u64>())
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut stones = process_input(input);

    for _ in 0..75 {
        stones = blink(&stones);
    }

    Box::new(stones.values().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"125 17";

    #[test]
    fn test_part1() {
        assert_eq!(55312.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
