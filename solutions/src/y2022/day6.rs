use std::collections::{HashSet, VecDeque};

use crate::solver::Solver;

pub struct Day6;

crate::impl_day!("6", true);

fn find_marker_index(input: &str, len: usize) -> Option<usize> {
    let mut v = VecDeque::new();
    let mut set = HashSet::new();

    for (i, c) in input.chars().enumerate() {
        set.clear();

        v.push_back(c);

        if v.len() > len {
            v.pop_front();
        }

        for val in v.iter().cloned() {
            set.insert(val);
        }

        if set.len() == len {
            return Some(i + 1);
        }
    }

    None
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(find_marker_index(input, 4).unwrap())
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(find_marker_index(input, 14).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            7.to_string(),
            *solve_part1(r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#).to_string()
        );
        assert_eq!(
            5.to_string(),
            *solve_part1(r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#).to_string()
        );
        assert_eq!(
            6.to_string(),
            *solve_part1(r#"nppdvjthqldpwncqszvftbrmjlhg"#).to_string()
        );
        assert_eq!(
            10.to_string(),
            *solve_part1(r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#).to_string()
        );
        assert_eq!(
            11.to_string(),
            *solve_part1(r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#).to_string()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            19.to_string(),
            *solve_part2(r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#).to_string()
        );
        assert_eq!(
            23.to_string(),
            *solve_part2(r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#).to_string()
        );
        assert_eq!(
            23.to_string(),
            *solve_part2(r#"nppdvjthqldpwncqszvftbrmjlhg"#).to_string()
        );
        assert_eq!(
            29.to_string(),
            *solve_part2(r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#).to_string()
        );
        assert_eq!(
            26.to_string(),
            *solve_part2(r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#).to_string()
        );
    }
}
