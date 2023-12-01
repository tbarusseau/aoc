use crate::solver::Solver;

pub struct Day1;

crate::impl_day!("1", true);

fn process_input(input: &str) -> &str {
    input.trim_end()
}

fn compute_result(input: &str) -> u32 {
    input
        .replace(char::is_alphabetic, "")
        .lines()
        .map(|l| {
            let mut iter = l.chars().flat_map(|c| char::to_digit(c, 10)).peekable();

            let first = *iter.peek().expect("no first numerical character");
            let last = iter.last().expect("no last numerical character");

            first * 10 + last
        })
        .sum()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = compute_result(input);

    Box::new(res)
}

fn replace_spelled_out_digits(input: &str) -> String {
    let mut ret = String::new();
    let mut slice = input;

    let numerals = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    loop {
        let len = slice.len();

        if len < 3 {
            ret.push_str(slice);

            break;
        }

        let mut found = false;
        for (s, c) in numerals.iter() {
            if slice.starts_with(s) {
                ret.push(*c);
                found = true;
                break;
            }
        }

        if !found {
            // For simplicity sake, discard anything that we don't want.
            let next_char = slice.chars().next().expect("no next char");

            if next_char.is_numeric() || next_char == '\n' {
                ret.push(next_char);
            }
        }

        // Advance the slice char by char to make sure we don't miss cases where
        // the last letters of a numeral form the first letters of another one.
        slice = &slice[1..];
    }

    ret
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let replaced = replace_spelled_out_digits(input);
    let res = compute_result(&replaced);

    Box::new(res)
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
