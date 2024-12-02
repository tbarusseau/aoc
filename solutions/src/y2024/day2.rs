use itertools::Itertools;
use num::abs;

pub struct Day2;

crate::impl_day!("2", true);

fn process_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse::<i32>().unwrap()).collect())
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let mut increasing = false;

    report
        .iter()
        .tuple_windows()
        .enumerate()
        .all(|(i, (a, b))| {
            if i == 0 {
                increasing = a < b;
            }

            let dist = abs(b - a);
            let safe_distance = (1..=3).contains(&dist);
            let proper_ordering = (increasing && (a < b)) || (!increasing && (b < a));

            safe_distance && proper_ordering
        })
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input.iter().filter(|v| is_safe(v)).count();
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input
        .iter()
        .filter(|&v| {
            if is_safe(v) {
                return true;
            }

            for i in 0..v.len() {
                let mut new_v = v.clone();
                new_v.remove(i);

                if is_safe(&new_v) {
                    return true;
                }
            }

            false
        })
        .count();
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(2.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(4.to_string(), *solve_part2(INPUT).to_string());
    }
}
