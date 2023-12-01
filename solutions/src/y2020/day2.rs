pub struct Day2;

crate::impl_day!("2", false);

fn process_input(input: &str) -> &str {
    input
}

#[allow(unused)]
fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new("Part 1 not done")
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new("Part 2 not done")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test_part1() {
        assert_eq!(0.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
