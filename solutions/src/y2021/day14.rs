use std::collections::HashMap;

pub struct Day14;

crate::impl_day!("14", true);

fn process_input(input: &str) -> (String, HashMap<Vec<char>, String>) {
    let mut lines = input.trim().lines();

    let template = lines.next().unwrap().to_owned();
    lines.next();

    let mut h = HashMap::new();

    for line in lines {
        let mut split = line.split(" -> ");
        let k = split.next().unwrap().chars().collect::<Vec<char>>();
        let v = split.next().unwrap().to_owned();

        h.insert(k, v);
    }

    (template, h)
}

fn step(state: &str, rules: &HashMap<Vec<char>, String>) -> String {
    let mut s = state
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .fold(String::new(), |mut s, e| {
            let v = rules.get(e).unwrap();

            s.push_str(&format!("{}{}", e[0], v));

            s
        });

    s.push(state.chars().last().unwrap());

    s
}

fn solve_for_n_cycles(input: &str, cycles: usize) -> usize {
    let (mut polymer, h) = process_input(input);

    for _ in 1..=cycles {
        polymer = step(&polymer, &h);
    }

    let char_counts = polymer.chars().fold(HashMap::new(), |mut h, c| {
        h.entry(c).and_modify(|v| *v += 1).or_insert(1);

        h
    });

    let (_, most_frequent) = char_counts.iter().max_by_key(|e| e.1).unwrap();
    let (_, least_frequent) = char_counts.iter().min_by_key(|e| e.1).unwrap();

    most_frequent - least_frequent
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(solve_for_n_cycles(input, 10))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(solve_for_n_cycles(input, 40))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn test_part1() {
        assert_eq!(1588.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            2_188_189_693_529_usize.to_string(),
            *solve_part2(INPUT).to_string()
        );
    }
}
