use itertools::Itertools;

pub struct Day7;

crate::impl_day!("7", true);

fn process_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .filter_map(|l| {
            l.split(": ").collect_tuple().map(|(a, b)| {
                (
                    a.parse::<i64>().unwrap(),
                    b.split(' ')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_vec(),
                )
            })
        })
        .collect_vec()
}

fn solve_for_part(input: &[(i64, Vec<i64>)], is_part2: bool) -> i64 {
    let ops = if is_part2 {
        vec!['*', '+', '|']
    } else {
        vec!['*', '+']
    };

    input
        .iter()
        .filter_map(|(goal, terms)| {
            for perm in (0..terms.len() - 1)
                .map(|_| ops.iter())
                .multi_cartesian_product()
            {
                let mut res = terms[0];
                for i in 0..perm.len() {
                    res = match perm.get(i) {
                        Some('*') => res * terms[i + 1],
                        Some('+') => res + terms[i + 1],
                        Some('|') => format!("{}{}", res, terms[i + 1]).parse::<i64>().unwrap(),
                        _ => unreachable!(),
                    };
                }

                if res == *goal {
                    return Some(goal);
                }
            }

            None
        })
        .sum()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new(solve_for_part(&input, false))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    Box::new(solve_for_part(&input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(3749.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(11387.to_string(), *solve_part2(INPUT).to_string());
    }
}
