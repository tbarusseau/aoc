use md5::Digest;

use crate::solver::Solver;

pub struct Day5;

crate::impl_day!("5", true);

fn hash(input: &str, index: i32) -> Digest {
    let data = format!("{input}{index}");

    md5::compute(data)
}

fn get_valid_digit(hash: &[u8; 16]) -> Option<char> {
    let s = hex::encode(hash);

    if s.starts_with("00000") {
        Some(s.chars().nth(5)).expect("no char at index 7")
    } else {
        None
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = input.trim();

    let mut password = String::new();
    let mut index = 0;

    loop {
        let hash = hash(input, index);

        if let Some(digit) = get_valid_digit(&hash) {
            password.push(digit);

            if password.len() == 8 {
                break;
            }
        }

        index += 1;
    }

    Box::new(password)
}

fn get_valid_digit_and_pos(hash: &[u8; 16]) -> Option<(char, usize)> {
    let s = hex::encode(hash);

    if s.starts_with("00000") {
        let pos = s.chars().nth(5).expect("no char at index 5");
        let pos = usize::from_str_radix(&format!("{}", pos), 16).expect("invalid pos");

        let value = s.chars().nth(6).expect("no chat at index 6");

        Some((value, pos))
    } else {
        None
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = input.trim();

    let mut password: Vec<Option<char>> = vec![None; 8];
    let mut index = 0;

    loop {
        let hash = hash(input, index);

        if let Some((value, pos)) = get_valid_digit_and_pos(&hash) {
            if pos > 7 {
                index += 1;
                continue;
            }

            if password[pos].is_none() {
                password[pos] = Some(value);
            }

            if password.iter().all(|v| v.is_some()) {
                break;
            }
        }

        index += 1;
    }

    Box::new(password.iter().flatten().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("18f47a30".to_string(), *solve_part1("abc").to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!("05ace8e3".to_string(), *solve_part2("abc").to_string());
    }
}
