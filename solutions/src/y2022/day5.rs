use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day5;

crate::impl_day!("5", true);

lazy_static! {
    static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

fn parse_input(input: &str) -> Vec<VecDeque<char>> {
    let mut v: Vec<VecDeque<char>> = vec![];

    let first_line = input.lines().next().unwrap();
    let len = (first_line.len() as f32 / 4.0).ceil() as usize;
    for _ in 0..len {
        v.push(VecDeque::new());
    }

    let mut lines = input.lines();

    loop {
        let line = lines.next().unwrap();
        if line.starts_with(" 1") {
            break;
        }

        for i in 0..len {
            let ch_index = 4 * i + 1;
            if let Some(ch) = line.chars().nth(ch_index) {
                if ch != ' ' {
                    v[i].push_back(ch);
                }
            }
        }
    }

    v
}

fn parse_order(line: &str) -> (usize, usize, usize) {
    let caps = RE.captures(line).unwrap();
    let count = usize::from_str_radix(caps.get(1).unwrap().as_str(), 10).unwrap();
    let from = usize::from_str_radix(caps.get(2).unwrap().as_str(), 10).unwrap() - 1;
    let to = usize::from_str_radix(caps.get(3).unwrap().as_str(), 10).unwrap() - 1;

    (count, from, to)
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut v = parse_input(input);

    for line in input.lines().skip_while(|p| !p.starts_with(" 1")).skip(2) {
        let (count, from, to) = parse_order(line);

        for _ in 0..count {
            let crat = v[from].pop_front().unwrap();
            v[to].push_front(crat);
        }
    }

    let res: String = v.iter().flat_map(|vd| vd.front()).collect();
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut v = parse_input(input);
    let mut inter_storage = vec![];

    for line in input.lines().skip_while(|p| !p.starts_with(" 1")).skip(2) {
        inter_storage.clear();

        let (count, from, to) = parse_order(line);

        for _ in 0..count {
            let crat = v[from].pop_front().unwrap();
            inter_storage.push(crat);
        }

        for ch in inter_storage.iter().rev() {
            v[to].push_front(*ch);
        }
    }

    let res: String = v.iter().flat_map(|vd| vd.front()).collect();
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_part1() {
        assert_eq!(String::from("CMZ"), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(String::from("MCD"), *solve_part2(INPUT).to_string());
    }
}
