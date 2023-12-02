pub struct Day8;

crate::impl_day!("8", true);

fn count_code_characters(input: &str) -> usize {
    input.len()
}

fn count_string_characters(input: &str) -> usize {
    let mut count = 0;
    let mut i = 1;

    while i < input.len() - 1 {
        let c = input.chars().nth(i).unwrap();
        let c_next = input.chars().nth(i + 1).unwrap();

        if c == '\\' {
            if c_next == 'x' {
                i += 4;
            } else {
                i += 2;
            }
        } else {
            i += 1;
        }

        count += 1;
    }

    count
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let code_characters_total: usize = input.lines().map(count_code_characters).sum();
    let string_characters_total: usize = input.lines().map(count_string_characters).sum();

    Box::new(code_characters_total - string_characters_total)
}

fn encode(input: &str) -> String {
    let mut s = String::new();

    s.push('"');
    for c in input.chars() {
        if c == '\\' || c == '"' {
            s.push('\\');
        }

        s.push(c);
    }
    s.push('"');

    s
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let code_characters_total: usize = input.lines().map(count_code_characters).sum();
    let encoded_len: usize = input.lines().map(|l| encode(l).len()).sum();

    Box::new(encoded_len - code_characters_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const S1: &str = r#""""#;
        const S2: &str = r#""abc""#;
        const S3: &str = r#""aaa\"aaa""#;
        const S4: &str = r#""\x27""#;
        const S5: &str = r#""\\""#;
        const S6: &str = r#""\\\\""#;
        const S7: &str = r#""\x27\\\\\\""#;

        assert_eq!(count_code_characters(S1), 2);
        assert_eq!(count_string_characters(S1), 0);

        assert_eq!(count_code_characters(S2), 5);
        assert_eq!(count_string_characters(S2), 3);

        assert_eq!(count_code_characters(S3), 10);
        assert_eq!(count_string_characters(S3), 7);

        assert_eq!(count_code_characters(S4), 6);
        assert_eq!(count_string_characters(S4), 1);

        assert_eq!(count_code_characters(S5), 4);
        assert_eq!(count_string_characters(S5), 1);

        assert_eq!(count_code_characters(S6), 6);
        assert_eq!(count_string_characters(S6), 2);

        assert_eq!(count_code_characters(S7), 12);
        assert_eq!(count_string_characters(S7), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(encode(r#""""#), r#""\"\"""#);
        assert_eq!(encode(r#""abc""#), r#""\"abc\"""#);
        assert_eq!(encode(r#""aaa\"aaa""#), r#""\"aaa\\\"aaa\"""#);
        assert_eq!(encode(r#""\x27""#), r#""\"\\x27\"""#);
    }
}
