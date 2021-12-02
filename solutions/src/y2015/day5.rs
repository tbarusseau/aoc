use std::collections::HashMap;

use itertools::Itertools;

use crate::solver::Solver;

pub struct Day5;

crate::impl_day!("5", true);

fn process_input(input: &str) -> &str {
    input
}

fn is_nice(input: &str) -> bool {
    let input = format!("{input} ");
    let blacklist = ["ab", "cd", "pq", "xy"];

    for b in blacklist {
        if input.contains(b) {
            return false;
        }
    }

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut letter_appears_twice_in_a_row = false;
    let mut vowels_count = 0;

    for (a, b) in input.chars().tuple_windows() {
        if vowels.contains(&a) {
            vowels_count += 1;
        }

        if a == b {
            letter_appears_twice_in_a_row = true;
        }
    }

    letter_appears_twice_in_a_row && vowels_count >= 3
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input
        .lines()
        .fold(0, |acc, l| if is_nice(l) { acc + 1 } else { acc });
    Box::new(res)
}

fn is_nice2(input: &str) -> bool {
    use std::collections::hash_map::Entry;

    let input = format!("{input} ");

    let mut two_letters_condition = false;
    let mut last_appeared: HashMap<(char, char), usize> = HashMap::new();

    for (i, (a, b)) in input.chars().tuple_windows().enumerate() {
        if let Entry::Vacant(e) = last_appeared.entry((a, b)) {
            e.insert(i);
        } else if i > *last_appeared.get(&(a, b)).unwrap() + 1 {
            two_letters_condition = true;
            break;
        }
    }

    let mut repeat_condition = false;
    for (a, _, c) in input.chars().tuple_windows() {
        if a == c {
            repeat_condition = true;
            break;
        }
    }

    repeat_condition && two_letters_condition
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input
        .lines()
        .fold(0, |acc, l| if is_nice2(l) { acc + 1 } else { acc });
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(true.to_string(), is_nice("ugknbfddgicrmopn").to_string());
        assert_eq!(true.to_string(), is_nice("aaa").to_string());
        assert_eq!(false.to_string(), is_nice("jchzalrnumimnmhp").to_string());
        assert_eq!(false.to_string(), is_nice("haegwjzuvuyypxyu").to_string());
        assert_eq!(false.to_string(), is_nice("dvszwmarrgswjxmb").to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(true.to_string(), is_nice2("qjhvhtzxzqqjkmpb").to_string());
        assert_eq!(true.to_string(), is_nice2("xxyxx").to_string());
        assert_eq!(false.to_string(), is_nice2("uurcxstgmygtbstg").to_string());
        assert_eq!(false.to_string(), is_nice2("ieodomkazucvgmuy").to_string());
    }
}
