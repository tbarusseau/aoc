pub struct Day1;

crate::impl_day!("1", true);

fn find_side_digit<'a>(
    digits: &'a [&str],
    line: &'a str,
    left: bool,
    find_fn: fn(&'a str, &'a str) -> Option<usize>,
) -> usize {
    let map = digits
        .iter()
        .enumerate()
        .flat_map(|(i, d)| find_fn(line, d).map(|v| (v, i % 9 + 1)));

    let extreme = if left {
        map.min_by(|(i1, _), (i2, _)| i1.cmp(i2))
    } else {
        map.min_by(|(i1, _), (i2, _)| i2.cmp(i1))
    };

    extreme.map(|(_, v)| v).unwrap()
}

fn compute_solution(input: &str, digits: &[&str]) -> usize {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let min = find_side_digit(digits, line, true, str::find);
            let max = find_side_digit(digits, line, false, str::rfind);

            min * 10 + max
        })
        .sum()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let digits = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    Box::new(compute_solution(input, digits))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let digits = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    Box::new(compute_solution(input, digits))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

        assert_eq!(142.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        assert_eq!(281.to_string(), *solve_part2(INPUT).to_string());
    }
}
