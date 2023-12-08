use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;
use regex::Regex;

pub struct Day8;

crate::impl_day!("8", true);

fn process_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut s = input.trim_end().split("\n\n");
    let mut h = HashMap::new();

    let instructions = s.next().unwrap().to_owned();
    let rest = s.next().unwrap();
    let re = Regex::new(r"([A-Z1-9]{3})").unwrap();

    rest.lines().for_each(|l| {
        let (key, left, right) = re
            .captures_iter(l)
            .map(|v| v.get(1).unwrap().as_str().to_owned())
            .collect_tuple()
            .unwrap();

        h.insert(key, (left, right));
    });

    (instructions, h)
}

fn is_left(c: char) -> bool {
    match c {
        'L' => true,
        'R' => false,
        _ => unreachable!(),
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let (instructions, hashmap) = process_input(input);
    let mut instructions_loop = instructions.chars().cycle();
    let mut position = "AAA";
    let mut counter = 0;

    let res = loop {
        let instruction = instructions_loop.next().expect("no next instruction");
        let (left, right) = hashmap.get(position).expect("position not found");

        position = if is_left(instruction) { left } else { right };

        counter += 1;

        if position == "ZZZ" {
            break counter;
        }
    };

    Box::new(res)
}

fn collect_starting_positions(h: &HashMap<String, (String, String)>) -> Vec<&str> {
    h.iter()
        .filter_map(|(k, _)| {
            if k.ends_with('A') {
                Some(k.as_ref())
            } else {
                None
            }
        })
        .collect_vec()
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let (instructions, hashmap) = process_input(input);
    let starting_positions: Vec<&str> = collect_starting_positions(&hashmap);
    let mut cycle_lengths = vec![];

    for pos in starting_positions {
        let mut counter = 0;

        let mut current_position = pos;
        let mut instructions_cycle = instructions.chars().cycle();

        while !current_position.ends_with('Z') {
            counter += 1;

            let instruction = instructions_cycle.next().unwrap();
            let (left, right) = hashmap.get(pos).unwrap();

            current_position = if is_left(instruction) { left } else { right };

            if current_position.ends_with('Z') {
                cycle_lengths.push(counter);
                break;
            }
        }
    }

    Box::new(cycle_lengths.iter().rev().fold(1_i64, |acc, v| acc.lcm(v)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const INPUT2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const INPUT3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_part1() {
        assert_eq!(2.to_string(), *solve_part1(INPUT1).to_string());
        assert_eq!(6.to_string(), *solve_part1(INPUT2).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(6.to_string(), *solve_part2(INPUT3).to_string());
    }
}
