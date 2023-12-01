pub struct Day1;

crate::impl_day!("1", true);

fn find_side_digit(digits: &[&str], line: &str, left: bool) -> usize {
    let map = digits.iter().enumerate().flat_map(|(i, d)| {
        if left {
            if let Some(min_index) = line.find(d) {
                Some((min_index, i % 9 + 1))
            } else {
                None
            }
        } else {
            if let Some(max_index) = line.rfind(d) {
                Some((max_index, i % 9 + 1))
            } else {
                None
            }
        }
    });

    let extreme = if left {
        map.min_by(|(i1, _), (i2, _)| i1.cmp(i2))
    } else {
        map.max_by(|(i1, _), (i2, _)| i1.cmp(i2))
    };

    extreme.map(|(_, v)| v).unwrap()
}

fn compute_solution(input: &str, digits: &[&str]) -> usize {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let min = find_side_digit(digits, line, true);
            let max = find_side_digit(digits, line, false);

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
