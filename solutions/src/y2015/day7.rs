use std::{collections::HashMap, convert::TryFrom};

use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day7;

crate::impl_day!("7", true);

fn process_input(input: &str) -> Vec<Instruction> {
    input.lines().flat_map(Instruction::try_from).collect()
}

type State = HashMap<String, Option<i32>>;

#[derive(Debug)]
enum Operation {
    Assignment(i32),
    RegisterAssignment(String),
    And(String, String),
    Or(String, String),
    LShift(String, i32),
    RShift(String, i32),
    Not(String),
    BitwiseWithOne(String),
}

lazy_static! {
    static ref RE_ASSIGNMENT: Regex = Regex::new(r"^(\d+|[a-z]+) -> ([a-z]+)$").unwrap();
    static ref RE_AND: Regex = Regex::new(r"^([a-z]+) AND ([a-z]+) -> ([a-z]+)$").unwrap();
    static ref RE_OR: Regex = Regex::new(r"^([a-z]+) OR ([a-z]+) -> ([a-z]+)$").unwrap();
    static ref RE_LSHIFT: Regex = Regex::new(r"^([a-z]+) LSHIFT (\d+) -> ([a-z]+)$").unwrap();
    static ref RE_RSHIFT: Regex = Regex::new(r"^([a-z]+) RSHIFT (\d+) -> ([a-z]+)$").unwrap();
    static ref RE_NOT: Regex = Regex::new(r"^NOT ([a-z]+) -> ([a-z]+)$").unwrap();
    static ref RE_COPY: Regex = Regex::new(r"^1 AND ([a-z]+) -> ([a-z]+)$").unwrap();
}

#[derive(Debug)]
struct Instruction(Operation, String);

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Operation::{
            And, Assignment, BitwiseWithOne, LShift, Not, Or, RShift, RegisterAssignment,
        };

        if let Some(captures) = RE_ASSIGNMENT.captures(value) {
            let value = captures.get(1).and_then(|s| s.as_str().parse().ok());

            let target = captures
                .get(2)
                .ok_or_else(|| anyhow!("invalid assignment target"))?
                .as_str();

            if let Some(v) = value {
                Ok(Self(Assignment(v), target.to_owned()))
            } else {
                let register = captures
                    .get(1)
                    .ok_or_else(|| anyhow!("invalid assignment register"))?
                    .as_str()
                    .to_owned();
                Ok(Self(RegisterAssignment(register), target.to_owned()))
            }
        } else if let Some(captures) = RE_AND.captures(value) {
            let a = captures
                .get(1)
                .ok_or_else(|| anyhow!("invalid AND first operand"))?
                .as_str()
                .to_owned();
            let b = captures
                .get(2)
                .ok_or_else(|| anyhow!("invalid AND second operand"))?
                .as_str()
                .to_owned();
            let target = captures
                .get(3)
                .ok_or_else(|| anyhow!("invalid AND target"))?
                .as_str()
                .to_owned();

            Ok(Self(And(a, b), target))
        } else if let Some(captures) = RE_OR.captures(value) {
            let a = captures
                .get(1)
                .ok_or_else(|| anyhow!("invalid OR first operand"))?
                .as_str()
                .to_owned();
            let b = captures
                .get(2)
                .ok_or_else(|| anyhow!("invalid OR second operand"))?
                .as_str()
                .to_owned();
            let target = captures
                .get(3)
                .ok_or_else(|| anyhow!("invalid OR target"))?
                .as_str()
                .to_owned();

            Ok(Self(Or(a, b), target))
        } else if let Some(captures) = RE_LSHIFT.captures(value) {
            let register = captures
                .get(1)
                .ok_or_else(|| anyhow!("invalid LSHIFT register"))?
                .as_str()
                .to_owned();
            let shift_value = captures
                .get(2)
                .and_then(|s| s.as_str().parse().ok())
                .ok_or_else(|| anyhow!("invalid LSHIFT value"))?;
            let target = captures
                .get(3)
                .ok_or_else(|| anyhow!("invalid LSHIFT target"))?
                .as_str()
                .to_owned();

            Ok(Self(LShift(register, shift_value), target))
        } else if let Some(captures) = RE_RSHIFT.captures(value) {
            let register = captures
                .get(1)
                .ok_or_else(|| anyhow!("invalid RSHIFT register"))?
                .as_str()
                .to_owned();
            let shift_value = captures
                .get(2)
                .and_then(|s| s.as_str().parse().ok())
                .ok_or_else(|| anyhow!("invalid RSHIFT value"))?;
            let target = captures
                .get(3)
                .ok_or_else(|| anyhow!("invalid RSHIFT target"))?
                .as_str()
                .to_owned();

            Ok(Self(RShift(register, shift_value), target))
        } else if let Some(captures) = RE_NOT.captures(value) {
            let register = captures
                .get(1)
                .ok_or_else(|| anyhow!("invalid NOT register"))?
                .as_str()
                .to_owned();
            let target = captures
                .get(2)
                .ok_or_else(|| anyhow!("invalid NOT target"))?
                .as_str()
                .to_owned();

            Ok(Self(Not(register), target))
        } else if let Some(captures) = RE_COPY.captures(value) {
            let register = captures
                .get(1)
                .ok_or_else(|| anyhow!("invalid COPY register"))?
                .as_str()
                .to_owned();
            let target = captures
                .get(2)
                .ok_or_else(|| anyhow!("invalid COPY target"))?
                .as_str()
                .to_owned();

            Ok(Self(BitwiseWithOne(register), target))
        } else {
            Err(anyhow!("unsupported value: {}", value))
        }
    }
}

impl Instruction {
    fn execute(&self, state: &mut State) {
        match &self.0 {
            Operation::Assignment(v) => {
                state.entry(self.1.clone()).and_modify(|e| *e = Some(*v));
            }
            Operation::And(a, b) => {
                let a = *state.get(a).unwrap();
                let b = *state.get(b).unwrap();

                if let (Some(va), Some(vb)) = (a, b) {
                    state
                        .entry(self.1.clone())
                        .and_modify(|e| *e = Some(va & vb));
                }
            }
            Operation::Or(a, b) => {
                let a = *state.get(a).unwrap();
                let b = *state.get(b).unwrap();

                if let (Some(va), Some(vb)) = (a, b) {
                    state
                        .entry(self.1.clone())
                        .and_modify(|e| *e = Some(va | vb));
                }
            }
            Operation::LShift(a, v) => {
                let a = *state.get(a).unwrap();

                if let Some(va) = a {
                    state
                        .entry(self.1.clone())
                        .and_modify(|e| *e = Some(va << v));
                }
            }
            Operation::RShift(a, v) => {
                let a = *state.get(a).unwrap();

                if let Some(va) = a {
                    state
                        .entry(self.1.clone())
                        .and_modify(|e| *e = Some(va >> v));
                }
            }
            Operation::Not(a) => {
                let a = *state.get(a).unwrap();

                if let Some(va) = a {
                    state.entry(self.1.clone()).and_modify(|e| *e = Some(!va));
                }
            }
            Operation::BitwiseWithOne(a) => {
                let a = *state.get(a).unwrap();

                if let Some(va) = a {
                    state
                        .entry(self.1.clone())
                        .and_modify(|e| *e = Some(va & 1));
                }
            }
            Operation::RegisterAssignment(a) => {
                let a = *state.get(a).unwrap();

                if let Some(va) = a {
                    state.entry(self.1.clone()).and_modify(|e| *e = Some(va));
                }
            }
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut state: State = HashMap::new();

    for e in &input {
        state.insert(e.1.clone(), None);
    }

    while state.iter().any(|(_, v)| v.is_none()) {
        for instruction in &input {
            instruction.execute(&mut state);
        }
    }

    let res = state.get("a").unwrap().expect("No solution for 'a'");
    Box::new(res)
}

fn solve_part2(input_orig: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input_orig);
    let mut state: State = HashMap::new();

    // Initialize state based on input...
    for e in &input {
        state.insert(e.1.clone(), None);
    }

    // Run the algorithm once
    while state.iter().any(|(_, v)| v.is_none()) {
        for instruction in &input {
            instruction.execute(&mut state);
        }
    }

    let a_final_value = state.get("a").unwrap().expect("No solution for 'a'");

    // Reset everything
    let input = process_input(input_orig);

    // Remove instruction targetting wire `b`
    let identifier = "b".to_owned();
    let input: Vec<Instruction> = input.into_iter().filter(|e| e.1 != identifier).collect();

    let mut state: State = HashMap::new();

    // Take `a` final value, assign it to `b`
    state.insert("b".to_owned(), Some(a_final_value));

    // Insert default values for all wires except `b`
    for e in &input {
        state.insert(e.1.clone(), None);
    }

    // Run the algorithm once more
    while state.iter().any(|(_, v)| v.is_none()) {
        for instruction in &input {
            instruction.execute(&mut state);
        }
    }

    let res = state.get("a").unwrap().expect("No solution for 'a'");

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
y AND y -> a";

    #[test]
    fn test_part1() {
        assert_eq!(456.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
