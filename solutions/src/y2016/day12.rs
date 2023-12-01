use std::{collections::HashMap, convert::TryFrom};

use crate::utils::add_to_usize::add_to_usize;

pub struct Day12;

crate::impl_day!("12", true);

#[derive(Debug)]
enum Instruction {
    CopyValue(i32, char),
    CopyRegister(char, char),
    Inc(char),
    Dec(char),
    JnzValue(i32, i32),
    JnzRegister(char, i32),
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Instruction::*;

        if let Ok((a, b)) = sscanf::sscanf!(value, "cpy {String} {char}") {
            if let Ok(v) = i32::from_str_radix(&a, 10) {
                Ok(CopyValue(v, b))
            } else {
                let c = a.chars().next().ok_or_else(|| ())?;

                Ok(CopyRegister(c, b))
            }
        } else if let Ok(a) = sscanf::sscanf!(value, "inc {char}") {
            Ok(Inc(a))
        } else if let Ok(a) = sscanf::sscanf!(value, "dec {char}") {
            Ok(Dec(a))
        } else if let Ok((a, b)) = sscanf::sscanf!(value, "jnz {String} {i32}") {
            if let Ok(v) = i32::from_str_radix(&a, 10) {
                Ok(JnzValue(v, b))
            } else {
                let c = a.chars().next().ok_or_else(|| ())?;

                Ok(JnzRegister(c, b))
            }
        } else {
            Err(())
        }
    }
}

fn process_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .flat_map(Instruction::try_from)
        .collect()
}

fn init_registers(registers: &mut HashMap<char, i32>) {
    registers.insert('a', 0);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);
}

fn process_instructions(instructions: &[Instruction], registers: &mut HashMap<char, i32>) {
    let mut instruction_pointer: usize = 0;
    let mut prevent_increment;

    loop {
        prevent_increment = false;
        let instruction = &instructions[instruction_pointer];

        match instruction {
            Instruction::CopyValue(a, b) => {
                registers.entry(*b).and_modify(|e| *e = *a);
            }
            Instruction::CopyRegister(a, b) => {
                let v = registers.get(a).expect("unknown register").clone();
                registers.entry(*b).and_modify(|e| *e = v);
            }
            Instruction::Inc(a) => {
                registers.entry(*a).and_modify(|e| *e += 1);
            }
            Instruction::Dec(a) => {
                registers.entry(*a).and_modify(|e| *e -= 1);
            }
            Instruction::JnzValue(a, b) => {
                if *a != 0 {
                    instruction_pointer = add_to_usize(instruction_pointer, *b);
                    prevent_increment = true;
                }
            }
            Instruction::JnzRegister(a, b) => {
                let v = registers.get(a).expect("unknown register");

                if *v != 0 {
                    instruction_pointer = add_to_usize(instruction_pointer, *b);
                    prevent_increment = true;
                }
            }
        }

        if !prevent_increment {
            instruction_pointer += 1;
        }

        if instruction_pointer >= instructions.len() {
            break;
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let instructions = process_input(input);
    let mut registers: HashMap<char, i32> = HashMap::new();

    init_registers(&mut registers);
    process_instructions(&instructions, &mut registers);

    Box::new(*registers.get(&'a').expect("unknown register"))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let instructions = process_input(input);
    let mut registers = HashMap::new();

    init_registers(&mut registers);
    registers.entry('c').and_modify(|e| *e = 1);
    process_instructions(&instructions, &mut registers);

    Box::new(*registers.get(&'a').expect("unknown register"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a
"#;

    #[test]
    fn test_part1() {
        assert_eq!(42.to_string(), *solve_part1(INPUT).to_string());
    }
}
