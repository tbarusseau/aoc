use std::{borrow::Cow, collections::HashMap};

use itertools::Itertools;

pub struct Day5;

crate::impl_day!("5", true);

fn process_input(input: &str) -> (String, Vec<Vec<i32>>) {
    let (first, second) = input
        .trim()
        .split("\n\n")
        .map(std::string::ToString::to_string)
        .collect_tuple()
        .unwrap();

    (
        first,
        second
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    )
}

fn build_dependencies(rules: &str) -> (HashMap<i32, Vec<i32>>, HashMap<i32, Vec<i32>>) {
    let mut dependencies: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut reverse_dependencies: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules.lines() {
        let first = &rule[0..2].parse::<i32>().unwrap();
        let second = &rule[3..5].parse::<i32>().unwrap();

        dependencies.entry(*second).or_default().push(*first);
        reverse_dependencies
            .entry(*first)
            .or_default()
            .push(*second);
    }

    (dependencies, reverse_dependencies)
}

fn update_is_valid(
    dependencies: &HashMap<i32, Vec<i32>>,
    reverse_dependencies: &HashMap<i32, Vec<i32>>,
    numbers: &[i32],
) -> bool {
    let mut encountered_numbers = &numbers[0..0];
    let mut remaining_numbers = &numbers[1..];

    for (i, number) in numbers.iter().enumerate() {
        let num_deps = dependencies
            .get(number)
            .map_or(Cow::Owned(vec![]), Cow::Borrowed);
        let num_rev_deps = reverse_dependencies
            .get(number)
            .map_or(Cow::Owned(vec![]), Cow::Borrowed);

        if encountered_numbers.iter().any(|n| !num_deps.contains(n)) {
            return false;
        }
        if remaining_numbers.iter().any(|n| !num_rev_deps.contains(n)) {
            return false;
        }

        encountered_numbers = &numbers[0..i];

        if i < numbers.len() - 1 {
            remaining_numbers = &numbers[i + 2..];
        } else {
            remaining_numbers = &[];
        }
    }

    true
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let (rules, updates) = process_input(input);

    let (dependencies, reverse_dependencies) = build_dependencies(&rules);
    let res = updates
        .iter()
        .filter(|u| update_is_valid(&dependencies, &reverse_dependencies, u))
        .map(|u| u[u.len() / 2])
        .sum::<i32>();

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let (rules, updates) = process_input(input);
    let (dependencies, reverse_dependencies) = build_dependencies(&rules);

    let mut res = 0;

    for update in updates {
        if update_is_valid(&dependencies, &reverse_dependencies, &update) {
            continue;
        }

        // Sort the page numbers
        let mut sorted_update = update.clone();
        sorted_update.sort_by_key(|n| {
            let deps = dependencies
                .get(n)
                .map_or(Cow::Owned(vec![]), Cow::Borrowed);

            deps.iter().filter(|n| update.contains(n)).count()
        });

        res += sorted_update[sorted_update.len() / 2];
    }

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(143.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(123.to_string(), *solve_part2(INPUT).to_string());
    }
}
