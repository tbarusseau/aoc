use std::collections::HashMap;

pub struct Day16;

crate::impl_day!("16", false);

#[allow(unused)]
fn process_input(input: &str) -> (String, String) {
    let encoded = input.to_owned();
    let mut decoded = String::new();

    let table: HashMap<char, String> = [
        ('0', "0000".to_string()),
        ('1', "0001".to_string()),
        ('2', "0010".to_string()),
        ('3', "0011".to_string()),
        ('4', "0100".to_string()),
        ('5', "0101".to_string()),
        ('6', "0110".to_string()),
        ('7', "0111".to_string()),
        ('8', "1000".to_string()),
        ('9', "1001".to_string()),
        ('A', "1010".to_string()),
        ('B', "1011".to_string()),
        ('C', "1100".to_string()),
        ('D', "1101".to_string()),
        ('E', "1110".to_string()),
        ('F', "1111".to_string()),
    ]
    .into();

    for c in encoded.chars() {
        decoded.push_str(table.get(&c).unwrap());
    }

    decoded.trim_end_matches('0');

    (encoded, decoded)
}

#[allow(unused)]
fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let (encoded, decoded) = process_input(input);

    let res = "Part 1 not done";
    Box::new(res)
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = "Part 2 not done";
    Box::new(res)
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
