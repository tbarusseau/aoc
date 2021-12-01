use crate::solver::Solver;

pub struct Day11;

crate::impl_day!("11", false);

fn process_input(input: String) -> String {
    input
}

#[allow(unused)]
fn solve_part1(input: String) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new("Part 1 not done")
}

#[allow(unused)]
fn solve_part2(input: String) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new("Part 2 not done")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test_part1() {
        assert_eq!(0.to_string(), *solve_part1(INPUT.to_string()).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT.to_string()).to_string());
    }
}
