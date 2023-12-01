use itertools::Itertools;

pub struct Day1;

crate::impl_day!("1", true);

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(
        input
            .split("\n\n")
            .map(|s| s.lines().fold(0, |acc, l| acc + l.parse::<u32>().unwrap()))
            .max()
            .unwrap(),
    )
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(
        input
            .split("\n\n")
            .map(|s| s.lines().fold(0, |acc, l| acc + l.parse::<u32>().unwrap()))
            .sorted()
            .rev()
            .take(3)
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_part1() {
        assert_eq!(24000.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(45000.to_string(), *solve_part2(INPUT).to_string());
    }
}
