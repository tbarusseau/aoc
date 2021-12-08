use crate::solver::Solver;

pub struct Day1;

crate::impl_day!("1", true);

use itertools::Itertools;

fn process_input(input: &str) -> Vec<i32> {
    input.lines().flat_map(|s| s.parse::<i32>()).collect()
}

#[allow(unused)]
fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut res = None;

    for v in input.iter().permutations(2) {
        if v[0] + v[1] == 2020 {
            res = Some(v[0] * v[1]);
        }
    }

    Box::new(res.unwrap())
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut res = None;

    for v in input.iter().permutations(3) {
        if v[0] + v[1] + v[2] == 2020 {
            res = Some(v[0] * v[1] * v[2]);
        }
    }

    Box::new(res.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_generator() {
        assert_eq!(
            process_input(
                "1
2
3"
            ),
            [1, 2, 3]
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(
            *solve_part1(
                "1721
979
366
299
675
1456"
            )
            .to_string(),
            514579.to_string()
        );
    }

    #[test]
    pub fn test2() {
        assert_eq!(
            *solve_part2(
                "1721
979
366
299
675
1456"
            )
            .to_string(),
            241861950.to_string()
        )
    }
}
