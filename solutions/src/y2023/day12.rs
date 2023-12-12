use std::collections::HashMap;

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

fn count_valid_permutations(
    arrangement: &str,
    damaged_springs: &[i32],
    hashmap: &mut HashMap<(String, Vec<i32>), usize>,
) -> usize {
    if let Some(v) = hashmap.get(&(arrangement.to_owned(), damaged_springs.to_owned())) {
        return *v;
    }

    let result = if arrangement.is_empty() {
        usize::from(damaged_springs.is_empty())
    } else if let Some(stripped) = arrangement.strip_prefix('.') {
        count_valid_permutations(stripped, damaged_springs, hashmap)
    } else if arrangement.starts_with('?') {
        count_valid_permutations(&arrangement.replacen('?', ".", 1), damaged_springs, hashmap)
            + count_valid_permutations(&arrangement.replacen('?', "#", 1), damaged_springs, hashmap)
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
                        &arrangement[next_damaged_group_size + 1..],
                        &damaged_springs[1..],
                        hashmap,
                    )
                }
            } else {
                count_valid_permutations(
                    &arrangement[next_damaged_group_size..],
                    &damaged_springs[1..],
                    hashmap,
                )
            }
        }
    } else {
        unreachable!()
    };

    hashmap.insert((arrangement.to_owned(), damaged_springs.to_owned()), result);

    result
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut h = HashMap::new();

    let res: usize = input
        .iter()
        .map(|v| count_valid_permutations(v.0, &v.1, &mut h))
        .sum();

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let input = input
        .into_iter()
        .map(|(a, b)| (format!("{a}?{a}?{a}?{a}?{a}"), b.repeat(5)))
        .collect_vec();

    let mut h = HashMap::new();

    let res: usize = input
        .iter()
        .map(|v| count_valid_permutations(&v.0, &v.1, &mut h))
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
