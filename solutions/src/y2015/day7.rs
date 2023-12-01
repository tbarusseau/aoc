use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day7;

crate::impl_day!("7", true);

fn process_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
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

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        use Operation::*;

        if let Some(captures) = RE_ASSIGNMENT.captures(value) {
            let value = captures
                .get(1)
                .and_then(|s| i32::from_str_radix(s.into(), 10).ok());

            let target = captures.get(2).unwrap().as_str();

            if let Some(v) = value {
                Instruction(Assignment(v), target.to_owned())
            } else {
                let register = captures.get(1).unwrap().as_str().to_owned();
                Instruction(RegisterAssignment(register), target.to_owned())
            }
        } else if let Some(captures) = RE_AND.captures(value) {
            let a = captures.get(1).unwrap().as_str().to_owned();
            let b = captures.get(2).unwrap().as_str().to_owned();
            let target = captures.get(3).unwrap().as_str().to_owned();

            Instruction(And(a, b), target)
        } else if let Some(captures) = RE_OR.captures(value) {
            let a = captures.get(1).unwrap().as_str().to_owned();
            let b = captures.get(2).unwrap().as_str().to_owned();
            let target = captures.get(3).unwrap().as_str().to_owned();

            Instruction(Or(a, b), target)
        } else if let Some(captures) = RE_LSHIFT.captures(value) {
            let register = captures.get(1).unwrap().as_str().to_owned();
            let shift_value = captures
                .get(2)
                .and_then(|s| i32::from_str_radix(s.into(), 10).ok())
                .unwrap();
            let target = captures.get(3).unwrap().as_str().to_owned();

            Instruction(LShift(register, shift_value), target)
        } else if let Some(captures) = RE_RSHIFT.captures(value) {
            let register = captures.get(1).unwrap().as_str().to_owned();
            let shift_value = captures
                .get(2)
                .and_then(|s| i32::from_str_radix(s.into(), 10).ok())
                .unwrap();
            let target = captures.get(3).unwrap().as_str().to_owned();

            Instruction(RShift(register, shift_value), target)
        } else if let Some(captures) = RE_NOT.captures(value) {
            let register = captures.get(1).unwrap().as_str().to_owned();
            let target = captures.get(2).unwrap().as_str().to_owned();

            Instruction(Not(register), target)
        } else if let Some(captures) = RE_COPY.captures(value) {
            let register = captures.get(1).unwrap().as_str().to_owned();
            let target = captures.get(2).unwrap().as_str().to_owned();

            Instruction(BitwiseWithOne(register), target)
        } else {
            panic!("Unsupported value: {}", value);
        }
    }
}

impl Instruction {
    fn execute(&self, state: &mut State) {
        match &self.0 {
            Operation::Assignment(v) => {
                state.entry(self.1.to_owned()).and_modify(|e| *e = Some(*v));
            }
            Operation::And(a, b) => {
                let a = state.get(a).unwrap().clone();
                let b = state.get(b).unwrap().clone();

                if let (Some(va), Some(vb)) = (a, b) {
                    state
                        .entry(self.1.to_owned())
                        .and_modify(|e| *e = Some(va & vb));
                }
            }
            Operation::Or(a, b) => {
                let a = state.get(a).unwrap().clone();
                let b = state.get(b).unwrap().clone();

                if let (Some(va), Some(vb)) = (a, b) {
                    state
                        .entry(self.1.to_owned())
                        .and_modify(|e| *e = Some(va | vb));
                }
            }
            Operation::LShift(a, v) => {
                let a = state.get(a).unwrap().clone();

                if let Some(va) = a {
                    state
                        .entry(self.1.to_owned())
                        .and_modify(|e| *e = Some(va << v));
                }
            }
            Operation::RShift(a, v) => {
                let a = state.get(a).unwrap().clone();

                if let Some(va) = a {
                    state
                        .entry(self.1.to_owned())
                        .and_modify(|e| *e = Some(va >> v));
                }
            }
            Operation::Not(a) => {
                let a = state.get(a).unwrap().clone();

                if let Some(va) = a {
                    state
                        .entry(self.1.to_owned())
                        .and_modify(|e| *e = Some(!va));
                }
            }
            Operation::BitwiseWithOne(a) => {
                let a = state.get(a).unwrap().clone();

                if let Some(va) = a {
                    state
                        .entry(self.1.to_owned())
                        .and_modify(|e| *e = Some(va & 1));
                }
            }
            Operation::RegisterAssignment(a) => {
                let a = state.get(a).unwrap().clone();

                if let Some(va) = a {
                    state.entry(self.1.to_owned()).and_modify(|e| *e = Some(va));
                }
            }
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut state: State = HashMap::new();

    input.iter().for_each(|e| {
        state.insert(e.1.clone(), None);
    });

    while state.iter().any(|(_, v)| v.is_none()) {
        for instruction in input.iter() {
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
    input.iter().for_each(|e| {
        state.insert(e.1.clone(), None);
    });

    // Run the algorithm once
    while state.iter().any(|(_, v)| v.is_none()) {
        for instruction in input.iter() {
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
    input.iter().for_each(|e| {
        state.insert(e.1.clone(), None);
    });

    // Run the algorithm once more
    while state.iter().any(|(_, v)| v.is_none()) {
        for instruction in input.iter() {
            instruction.execute(&mut state);
        }
    }

    let res = state.get("a").unwrap().expect("No solution for 'a'");

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
y AND y -> a"#;

    #[test]
    fn test_part1() {
        assert_eq!(456.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
