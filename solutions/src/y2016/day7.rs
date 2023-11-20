use std::collections::HashSet;

use itertools::Itertools;

use crate::solver::Solver;

pub struct Day7;

crate::impl_day!("7", true);

#[derive(Debug)]
enum IpSequence {
    Regular(String),
    Hypernet(String),
}

fn to_ip(input: &str) -> Vec<IpSequence> {
    input
        .replace("[", " ")
        .replace("]", " ")
        .trim()
        .split(" ")
        .enumerate()
        .map(|(i, s)| {
            if i % 2 == 0 {
                IpSequence::Regular(s.to_owned())
            } else {
                IpSequence::Hypernet(s.to_owned())
            }
        })
        .collect()
}

fn process_input(input: &str) -> Vec<Vec<IpSequence>> {
    input.lines().map(to_ip).collect()
}

fn is_abba(s: &str) -> bool {
    for (a, b, c, d) in s.chars().tuple_windows() {
        if a != b && b == c && a == d {
            return true;
        }
    }

    false
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res: i32 = input
        .iter()
        .map(|sequence| {
            let regulars = sequence
                .into_iter()
                .filter_map(|v| match v {
                    IpSequence::Regular(r) => Some(r),
                    _ => None,
                })
                .collect_vec();

            let hypernets = sequence
                .into_iter()
                .filter_map(|v| match v {
                    IpSequence::Hypernet(h) => Some(h),
                    _ => None,
                })
                .collect_vec();

            if regulars.iter().any(|r| is_abba(r)) && hypernets.iter().all(|h| !is_abba(h)) {
                1
            } else {
                0
            }
        })
        .sum();

    Box::new(res)
}

fn supports_ssl(sequence: &[IpSequence]) -> bool {
    let regulars = sequence
        .into_iter()
        .filter_map(|v| match v {
            IpSequence::Regular(r) => Some(r),
            _ => None,
        })
        .collect_vec();

    let hypernets = sequence
        .into_iter()
        .filter_map(|v| match v {
            IpSequence::Hypernet(h) => Some(h),
            _ => None,
        })
        .collect_vec();

    let mut aba_collection = HashSet::new();

    regulars.iter().for_each(|r| {
        for (a, b, c) in r.chars().tuple_windows() {
            if a != b && a == c && !aba_collection.contains(&(a, b)) {
                aba_collection.insert((a, b));
            }
        }
    });

    for hyper in hypernets.iter() {
        for (a, b, c) in hyper.chars().tuple_windows() {
            if a != b && a == c && aba_collection.contains(&(b, a)) {
                return true;
            }
        }
    }

    false
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input.iter().filter(|s| supports_ssl(s)).count();
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abba() {
        assert!(is_abba("abba"));
        assert!(is_abba("ioxxoj"));
        assert!(!is_abba("asdfgh"));
        assert!(!is_abba("a"));
        assert!(!is_abba(""));
    }

    #[test]
    fn test_ssl() {
        assert!(supports_ssl(&to_ip("aba[bab]xyz")));
        assert!(supports_ssl(&to_ip("aaa[kek]eke")));
        assert!(supports_ssl(&to_ip("zazbz[bzb]cdb")));

        assert!(!supports_ssl(&to_ip("xyx[xyx]xyx")));
    }
}
