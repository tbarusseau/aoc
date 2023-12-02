use std::collections::VecDeque;

pub struct Day10;

crate::impl_day!("10", true);

fn process_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

const LEGAL_OPENINGS: &[char] = &['(', '[', '{', '<'];
const LEGAL_CLOSINGS: &[char] = &[')', ']', '}', '>'];

fn illegal_value(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

type ProcessResult<'a> = (&'a str, bool, Option<char>, VecDeque<char>);

fn process(line: &str) -> ProcessResult {
    let mut v = VecDeque::new();

    for c in line.chars() {
        if LEGAL_OPENINGS.contains(&c) {
            v.push_back(c);
            continue;
        }

        if LEGAL_CLOSINGS.contains(&c) {
            unreachable!("Non-legal character")
        }

        let last = v.pop_back();

        if let Some(last) = last {
            let index_closing = LEGAL_CLOSINGS.iter().position(|&e| e == c).unwrap();
            if last != LEGAL_OPENINGS[index_closing] {
                return (line, false, Some(c), v);
            }
        } else {
            unreachable!()
        }
    }

    (line, true, None, v)
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input
        .iter()
        .filter_map(|&l| process(l).2)
        .fold(0, |acc, c| acc + illegal_value(c));
    Box::new(res)
}

fn legal_value(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!("Invalid autocomplete char"),
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut process_results: Vec<ProcessResult> = process_input(input)
        .iter()
        .map(|&l| process(l))
        .filter(|(_, b, _, _)| *b)
        .collect();

    let mut scores = vec![];
    for result in &mut process_results {
        let (_, _, _, ref mut v) = result;
        let mut total_score: u64 = 0;
        while let Some(c) = v.pop_back() {
            let index = LEGAL_OPENINGS.iter().position(|e| *e == c).unwrap();
            let autocomplete_char = LEGAL_CLOSINGS[index];
            total_score = total_score * 5 + legal_value(autocomplete_char);
        }
        scores.push(total_score);
    }

    scores.sort_unstable();
    Box::new(scores[scores.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_part1() {
        assert_eq!(26397.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(288_957.to_string(), *solve_part2(INPUT).to_string());
    }
}
