pub struct Day4;

crate::impl_day!("4", true);

fn process_input(input: &str) -> &str {
    input.strip_suffix('\n').unwrap()
}

fn hash(secret_key: &str, n: i32) -> String {
    format!("{:x}", md5::compute(format!("{}{}", secret_key, n)))
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut i = 0;

    let res = loop {
        if hash(input, i).starts_with("00000") {
            break i;
        }

        i += 1;
    };
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut i = 0;

    let res = loop {
        if hash(input, i).starts_with("000000") {
            break i;
        }

        i += 1;
    };
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(609043.to_string(), *solve_part1("abcdef").to_string());
    }
}
