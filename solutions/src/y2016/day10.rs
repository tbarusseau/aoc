// Currently on hold because I didn't understand the problem
// and spent too long on this solution that is wrong.

use std::collections::HashMap;

use itertools::Itertools;

use crate::solver::Solver;

pub struct Day10;

crate::impl_day!("10", true);

enum Instruction {
    Value(i32, i32),
    Give(i32, String, i32, String, i32),
}

fn process_input(input: &str) -> Vec<Instruction> {
    input
        .trim_end()
        .lines()
        .map(|l| {
            let parsed = sscanf::sscanf!(l, "value {i32} goes to bot {i32}");
            if let Ok((value, bot)) = parsed {
                return Instruction::Value(value, bot);
            }

            let parsed = sscanf::sscanf!(
                l,
                "bot {i32} gives low to {String} {i32} and high to {String} {i32}"
            );
            if let Ok((bot, low_dest, low, high_dest, high)) = parsed {
                return Instruction::Give(bot, low_dest, low, high_dest, high);
            }

            panic!("invalid input: {}", l);
        })
        .collect_vec()
}

#[derive(Debug, PartialEq)]
struct Microship(Option<i32>, Option<i32>);

impl Microship {
    #[allow(unused)]
    pub fn is_full(&self) -> bool {
        self.0.is_some() && self.1.is_some()
    }

    pub fn give_low(&mut self) -> i32 {
        assert!(self.0.is_some() || self.1.is_some());

        let a = self.0;
        let b = self.1;

        match (a, b) {
            (Some(a), Some(b)) => {
                if a < b {
                    self.0 = None;

                    a
                } else {
                    self.1 = None;

                    b
                }
            }
            // (Some(v), None) | (None, Some(v)) => {
            //     if self.0.is_some() {
            //         self.0 = None;
            //     } else {
            //         self.1 = None;
            //     }

            //     v
            // }
            _ => unreachable!(),
        }
    }

    pub fn give_high(&mut self) -> i32 {
        assert!(self.0.is_some() || self.1.is_some());

        let a = self.0;
        let b = self.1;

        match (a, b) {
            (Some(a), Some(b)) => {
                if a > b {
                    self.0 = None;

                    a
                } else {
                    self.1 = None;

                    b
                }
            }
            // (Some(v), None) | (None, Some(v)) => {
            //     if self.0.is_some() {
            //         self.0 = None;
            //     } else {
            //         self.1 = None;
            //     }

            //     v
            // }
            _ => unreachable!(),
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let instructions = process_input(input);
    let mut microchips: HashMap<i32, Microship> = HashMap::new();

    let values = instructions
        .iter()
        .filter_map(|v| match v {
            v @ Instruction::Value(_, _) => Some(v),
            _ => None,
        })
        .collect_vec();

    let gives = instructions
        .iter()
        .filter_map(|v| match v {
            Instruction::Give(_, _, _, _, _) => Some(v),
            _ => None,
        })
        .collect_vec();

    for val in values {
        if let Instruction::Value(value, bot_index) = val {
            println!("Giving {} to {}", value, bot_index);

            microchips
                .entry(*bot_index)
                .and_modify(|v| {
                    if v.0.is_none() {
                        v.0 = Some(*value);
                    } else if v.1.is_none() {
                        v.1 = Some(*value);
                    } else {
                        unreachable!()
                    }
                })
                .or_insert(Microship(Some(*value), None));
        } else {
            unreachable!()
        }
    }

    let mut res = None;
    for give in gives {
        if let Instruction::Give(bot_index, _, _, _, _) = give {
            println!("Getting bot index {}", bot_index);
            let microchip = microchips.get_mut(&bot_index).unwrap();
            let low = microchip.give_low();
            let high = microchip.give_high();

            if low == 17 && high == 61 {
                res = Some(*bot_index);
            }
        } else {
            unreachable!()
        }
    }

    assert!(res.is_some());

    Box::new(res.unwrap())
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = "Part 2 not done";
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2"#;

    #[test]
    fn test_part1() {
        assert_eq!(0.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
