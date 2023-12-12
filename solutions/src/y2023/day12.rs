use cached::proc_macro::cached;
use itertools::Itertools;

pub struct Day12;

crate::impl_day!("12", true);

fn process_input(input: &str) -> Vec<(&str, Vec<i32>)> {
    input
        .trim_end()
        .lines()
        .map(|l| {
            let (start, end) = l.split(' ').collect_tuple().unwrap();
            let end = end.split(',').flat_map(str::parse).collect_vec();

            (start, end)
        })
        .collect_vec()
}

#[cached]
fn count_valid_permutations(arrangement: String, damaged_springs: Vec<i32>) -> usize {
    if arrangement.is_empty() {
        usize::from(damaged_springs.is_empty())
    } else if let Some(stripped) = arrangement.strip_prefix('.') {
        count_valid_permutations(stripped.to_owned(), damaged_springs)
    } else if arrangement.starts_with('?') {
        count_valid_permutations(arrangement.replacen('?', ".", 1), damaged_springs.clone())
            + count_valid_permutations(arrangement.replacen('?', "#", 1), damaged_springs)
    } else if arrangement.starts_with('#') {
        if damaged_springs.is_empty() {
            0
        } else {
            let next_damaged_group_size = damaged_springs[0] as usize;

            if arrangement.len() < next_damaged_group_size
                || (arrangement[0..next_damaged_group_size])
                    .chars()
                    .any(|c| c == '.')
            {
                0
            } else if damaged_springs.len() > 1 {
                if arrangement.len() < damaged_springs[0] as usize + 1
                    || arrangement.as_bytes()[next_damaged_group_size] == b'#'
                {
                    0
                } else {
                    count_valid_permutations(
                        arrangement[next_damaged_group_size + 1..].to_owned(),
                        damaged_springs[1..].to_owned(),
                    )
                }
            } else {
                count_valid_permutations(
                    arrangement[next_damaged_group_size..].to_owned(),
                    damaged_springs[1..].to_owned(),
                )
            }
        }
    } else {
        unreachable!()
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res: usize = input
        .into_iter()
        .map(|v| count_valid_permutations(v.0.to_string(), v.1))
        .sum();

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let input = input
        .into_iter()
        .map(|(a, b)| (format!("{a}?{a}?{a}?{a}?{a}"), b.repeat(5)))
        .collect_vec();

    let res: usize = input
        .into_iter()
        .map(|v| count_valid_permutations(v.0.to_string(), v.1))
        .sum();

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_part1() {
        assert_eq!(21.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(525_152.to_string(), *solve_part2(INPUT).to_string());
    }
}
