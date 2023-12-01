use lazy_static::lazy_static;
use regex::Regex;

pub struct Day4;

crate::impl_day!("4", true);

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

fn parse(l: &str) -> (i32, i32, i32, i32) {
    let caps = RE.captures(l).unwrap();
    let s1 = i32::from_str_radix(caps.get(1).unwrap().as_str(), 10).unwrap();
    let e1 = i32::from_str_radix(caps.get(2).unwrap().as_str(), 10).unwrap();
    let s2 = i32::from_str_radix(caps.get(3).unwrap().as_str(), 10).unwrap();
    let e2 = i32::from_str_radix(caps.get(4).unwrap().as_str(), 10).unwrap();

    (s1, e1, s2, e2)
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut res = 0;

    for l in input.lines() {
        let (s1, e1, s2, e2) = parse(l);

        if (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1) {
            res += 1;
        }
    }

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut res = 0;

    for l in input.lines() {
        let (s1, e1, s2, e2) = parse(l);

        if (s1 >= s2 && s1 <= e2)
            || (s2 >= s1 && s2 <= e1)
            || (e1 <= e2 && e1 >= s2)
            || (e2 <= e1 && e2 >= s1)
        {
            res += 1;
        }
    }

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_part1() {
        assert_eq!(2.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(4.to_string(), *solve_part2(INPUT).to_string());
    }
}
