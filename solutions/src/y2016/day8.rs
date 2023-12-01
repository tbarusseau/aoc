use std::{convert::TryFrom, fmt::Display};

use itertools::Itertools;

pub struct Day8;

crate::impl_day!("8", true);

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = sscanf::sscanf!(value, "rect {usize}x{usize}");
        if let Ok((x, y)) = parsed {
            return Ok(Instruction::Rect(x, y));
        }

        let parsed = sscanf::sscanf!(value, "rotate row y={usize} by {usize}");
        if let Ok((x, y)) = parsed {
            return Ok(Instruction::RotateRow(x, y));
        }

        let parsed = sscanf::sscanf!(value, "rotate column x={usize} by {usize}");
        if let Ok((x, y)) = parsed {
            return Ok(Instruction::RotateColumn(x, y));
        }

        Err(())
    }
}

fn process_input(input: &str) -> Vec<Instruction> {
    input.lines().flat_map(Instruction::try_from).collect_vec()
}

struct Screen(Vec<Vec<bool>>);

impl Screen {
    pub fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Rect(x, y) => {
                for j in 0..*y {
                    for i in 0..*x {
                        self.0[j][i] = true;
                    }
                }
            }
            Instruction::RotateRow(x, y) => {
                self.0[*x].rotate_right(*y);
            }
            Instruction::RotateColumn(x, y) => {
                let mut v = vec![];

                for j in 0..6 {
                    v.push(self.0[j][*x]);
                }

                v.rotate_right(*y);

                for j in 0..6 {
                    self.0[j][*x] = v[j];
                }
            }
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;

        for line in self.0.iter() {
            write!(
                f,
                "{}\n",
                line.iter()
                    .map(|&v| if v { '█' } else { ' ' })
                    .collect::<String>()
            )?;
        }

        Ok(())
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let instructions = process_input(input);
    let mut screen = Screen(vec![vec![false; 50]; 6]);

    instructions
        .iter()
        .for_each(|inst| screen.process_instruction(inst));

    let res = screen.0.iter().flatten().filter(|v| **v).count();
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let instructions = process_input(input);
    let mut screen = Screen(vec![vec![false; 50]; 6]);

    instructions
        .iter()
        .for_each(|inst| screen.process_instruction(inst));

    Box::new(screen)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test_part1() {
        assert_eq!(0.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
