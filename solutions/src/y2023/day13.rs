use itertools::Itertools;

pub struct Day13;

crate::impl_day!("13", true);

fn process_pattern(pattern: &str) -> Vec<Vec<char>> {
    pattern
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
}

fn process_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .trim_end()
        .split("\n\n")
        .map(process_pattern)
        .collect_vec()
}

fn collect_col(pattern: &[Vec<char>], index: usize) -> Option<Vec<char>> {
    if pattern.is_empty() || index >= pattern[0].len() {
        return None;
    }

    Some(
        pattern
            .iter()
            .filter_map(|row| row.get(index).copied())
            .collect_vec(),
    )
}

fn check_mirror_at_index(
    pattern: &[Vec<char>],
    index: usize,
    bound: usize,
    vertical: bool,
) -> bool {
    let mut counter = 0;

    while let Some(left) = index.checked_sub(counter) {
        let right = index + counter + 1;

        let (a, b) = if vertical {
            (collect_col(pattern, left), collect_col(pattern, right))
        } else {
            (pattern.get(left).cloned(), pattern.get(right).cloned())
        };

        if a != b {
            return false;
        }

        if a.is_some() && b.is_some() && a == b && (left == 0 || right == bound - 1) {
            return true;
        }

        counter += 1;
    }

    false
}

fn compute_summarized_notes(pattern: &[Vec<char>]) -> Vec<(usize, bool, usize)> {
    let mut v = vec![];

    let height = pattern.len();
    let width = pattern[0].len();

    for i in 0..height {
        if check_mirror_at_index(pattern, i, height, false) {
            v.push((100 * (i + 1), false, i + 1));
        }
    }

    for i in 0..width {
        if check_mirror_at_index(pattern, i, width, true) {
            v.push((i + 1, true, i + 1));
        }
    }

    v
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let res: usize = process_input(input)
        .iter()
        .map(|p| {
            *compute_summarized_notes(p)
                .first()
                .expect("no valid result")
        })
        .map(|(v, _, _)| v)
        .sum();
    Box::new(res)
}

fn compute_fixed(pattern: &[Vec<char>]) -> usize {
    let height = pattern.len();
    let width = pattern[0].len();

    let initial_result = *compute_summarized_notes(pattern)
        .first()
        .expect("no valid result");

    for y in 0..height {
        for x in 0..width {
            let mut cloned_pattern = pattern.to_owned();

            if pattern[y][x] == '.' {
                cloned_pattern[y][x] = '#';
            } else {
                cloned_pattern[y][x] = '.';
            }

            let results = compute_summarized_notes(&cloned_pattern);
            match results.len() {
                1 | 2 => {
                    if let Some(result) = results.iter().find(|v| **v != initial_result) {
                        return result.0;
                    }
                }
                v if v > 2 => {
                    unreachable!("more than 2 valid reflection lines");
                }
                _ => {}
            }
        }
    }

    unreachable!("couldn't find a reflection on fixed patterns")
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let res: usize = process_input(input).iter().map(|p| compute_fixed(p)).sum();

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_part1() {
        assert_eq!(405.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(400.to_string(), *solve_part2(INPUT).to_string());
    }

    #[test]
    fn validation_tests() {
        let pattern = process_pattern(
            "##.##.####.#.#.#.
....#...#.###.###
#####.#..#.......
.......##.#...##.
....#..#..#.###.#
......#.....##.#.
####..#.#...#.##.
....##..####.###.
####.#...#.###...",
        );

        assert!(check_mirror_at_index(&pattern, 0, pattern[0].len(), true));
    }
}
