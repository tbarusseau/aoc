use std::collections::{HashMap, HashSet};

use crate::solver::Solver;

pub struct Day6;

crate::impl_day!("6", true);

#[derive(Debug)]
enum Operation {
    TurnOff,
    TurnOn,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    start: (i32, i32),
    end: (i32, i32),
}

fn process_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|l| {
            let operation = if l.starts_with("turn off") {
                Operation::TurnOff
            } else if l.starts_with("turn on") {
                Operation::TurnOn
            } else if l.starts_with("toggle") {
                Operation::Toggle
            } else {
                panic!("unknown operation: {}", input);
            };

            let mut split = l.split(" through ");
            let mut start_str = split
                .next()
                .unwrap()
                .split(" ")
                .into_iter()
                .last()
                .unwrap()
                .split(",");
            let mut end_str = split.next().unwrap().split(",");

            let start = (
                start_str.next().unwrap().parse().unwrap(),
                start_str.next().unwrap().parse().unwrap(),
            );
            let end = (
                end_str.next().unwrap().parse().unwrap(),
                end_str.next().unwrap().parse().unwrap(),
            );

            Instruction {
                operation,
                start,
                end,
            }
        })
        .collect()
}

fn step(op: &Instruction, lights: &mut HashSet<(i32, i32)>) {
    let Instruction {
        operation,
        start: (xs, ys),
        end: (xe, ye),
    } = op;

    for y in *ys..=*ye {
        for x in *xs..=*xe {
            let pos = (x, y);

            match operation {
                Operation::TurnOff => lights.remove(&pos),
                Operation::TurnOn => lights.insert(pos),
                Operation::Toggle => {
                    if lights.get(&pos).is_some() {
                        lights.remove(&pos)
                    } else {
                        lights.insert(pos)
                    }
                }
            };
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut lights: HashSet<(i32, i32)> = HashSet::new();

    for op in input {
        step(&op, &mut lights);
    }

    let res = lights.iter().count();
    Box::new(res)
}

fn step2(op: &Instruction, lights: &mut HashMap<(i32, i32), i32>) {
    let Instruction {
        operation,
        start: (xs, ys),
        end: (xe, ye),
    } = op;

    for y in *ys..=*ye {
        for x in *xs..=*xe {
            let pos = (x, y);

            match operation {
                Operation::TurnOn => lights.entry(pos).and_modify(|v| *v += 1).or_insert(1),
                Operation::TurnOff => lights
                    .entry(pos)
                    .and_modify(|v| {
                        if *v - 1 < 0 {
                            *v = 0;
                        } else {
                            *v -= 1;
                        }
                    })
                    .or_insert(0),
                Operation::Toggle => lights.entry(pos).and_modify(|v| *v += 2).or_insert(2),
            };
        }
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut lights: HashMap<(i32, i32), i32> = HashMap::new();

    for op in input {
        step2(&op, &mut lights);
    }

    let res = lights.values().sum::<i32>();
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
"#;

    #[test]
    fn test_part1() {
        assert_eq!((999000 - 4).to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
