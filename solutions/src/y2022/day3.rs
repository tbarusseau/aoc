use std::collections::HashSet;

use itertools::Itertools;

use crate::solver::Solver;

pub struct Day3;

crate::impl_day!("3", true);

fn char_to_prio(c: &char) -> i32 {
    if c.is_lowercase() {
        *c as i32 - 'a' as i32 + 1
    } else {
        *c as i32 - 'A' as i32 + 27
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut res: Vec<char> = vec![];
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();

    for line in input.lines() {
        s1.clear();
        s2.clear();

        let half_len = line.len() / 2;
        for (i, c) in line.chars().enumerate() {
            if i < half_len {
                s1.insert(c);
            } else {
                s2.insert(c);
            }
        }

        res.push(*s1.intersection(&s2).next().unwrap());
    }

    Box::new(res.iter().map(char_to_prio).sum::<i32>())
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut res: Vec<char> = vec![];
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();
    let mut s3 = HashSet::new();

    for mut chunk in &input.lines().chunks(3) {
        s1.clear();
        s2.clear();
        s3.clear();

        let c1 = chunk.next().unwrap();
        let c2 = chunk.next().unwrap();
        let c3 = chunk.next().unwrap();

        for c in c1.chars() {
            s1.insert(c);
        }

        for c in c2.chars() {
            s2.insert(c);
        }

        for c in c3.chars() {
            s3.insert(c);
        }

        let s1c = s1.clone();
        let mut inter = s1c.intersection(&s2);
        s1.clear();

        while let Some(n) = inter.next() {
            s1.insert(*n);
        }

        let item = *s1.intersection(&s3).next().unwrap();
        res.push(item);
    }

    Box::new(res.iter().map(char_to_prio).sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_part1() {
        assert_eq!(157.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(70.to_string(), *solve_part2(INPUT).to_string());
    }
}
