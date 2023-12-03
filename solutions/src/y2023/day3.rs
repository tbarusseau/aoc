use std::collections::HashSet;

use itertools::Itertools;

pub struct Day3;

crate::impl_day!("3", true);

fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim_end()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn is_symbol(c: char) -> bool {
    !c.is_alphanumeric() && c != '.'
}

fn get_neighbouring_pos(pos: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    const OFFSETS: &[(isize, isize)] = &[
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let (px, py) = pos;

    OFFSETS
        .iter()
        .map(|offset| {
            let (dx, dy) = offset;

            let x = (px as isize + dx).clamp(0, width as isize - 1) as usize;
            let y = (py as isize + dy).clamp(0, height as isize - 1) as usize;

            (x, y)
        })
        .dedup()
        .collect_vec()
}

fn is_part_number(input: &[Vec<char>], pos: (usize, usize)) -> bool {
    let height = input.len();
    let width = input[0].len();

    let (px, py) = pos;

    if !input[py][px].is_numeric() {
        return false;
    }

    for offset in &get_neighbouring_pos(pos, width, height) {
        let (x, y) = *offset;

        if is_symbol(input[y][x]) {
            return true;
        }
    }

    false
}

fn extract_number(input: &[Vec<char>], pos: (usize, usize)) -> (i32, usize) {
    let mut start = pos.0;
    let mut end = pos.0;

    let y = pos.1;
    let line = &input[y];

    while start > 0 && line[start - 1] != '.' && !is_symbol(line[start - 1]) {
        start -= 1;
    }

    while end < line.len() - 1 && line[end + 1] != '.' && !is_symbol(line[end + 1]) {
        end += 1;
    }

    let extracted_slice: String = line[start..=end].iter().collect();

    (extracted_slice.parse().unwrap(), end - pos.0)
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut part_numbers = vec![];

    let mut x;

    for y in 0..input.len() {
        x = 0;

        while x < input[0].len() {
            if is_part_number(&input, (x, y)) {
                let (part_number, skip) = extract_number(&input, (x, y));
                part_numbers.push(part_number);

                x += skip + 1;
            } else {
                x += 1;
            }
        }
    }

    let res: i32 = part_numbers.iter().sum();
    Box::new(res)
}

fn extract_surrounding_numbers(input: &[Vec<char>], pos: (usize, usize)) -> Vec<i32> {
    let width = input[0].len();
    let height = input.len();

    let mut h = HashSet::new();

    for offset in &get_neighbouring_pos(pos, width, height) {
        let (x, y) = *offset;

        if input[y][x].is_numeric() {
            let (num, _) = extract_number(input, (x, y));

            h.insert(num);
        }
    }

    h.into_iter().collect_vec()
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut res = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] != '*' {
                continue;
            }

            let surrounding_numbers = extract_surrounding_numbers(&input, (x, y));

            if surrounding_numbers.len() != 2 {
                continue;
            }

            res += surrounding_numbers[0] * surrounding_numbers[1];
        }
    }

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part1() {
        assert_eq!(4361.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(467_835.to_string(), *solve_part2(INPUT).to_string());
    }
}
