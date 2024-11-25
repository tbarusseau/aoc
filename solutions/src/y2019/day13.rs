pub struct Day13;

crate::impl_day!("13", true);

use crate::y2019::intcode_computer::IntcodeComputer;

struct Game {
    pub computer: IntcodeComputer,
    pub score: i64,
}

impl Game {
    pub fn from(input: &[i64]) -> Self {
        Self {
            computer: IntcodeComputer::from(input, vec![]),
            score: 0,
        }
    }

    pub fn count_blocks(&mut self) -> i64 {
        self.computer.reinitialize_memory();

        let mut count = 0;
        let mut r = Vec::new();

        loop {
            use crate::y2019::intcode_computer::State::{
                GaveOutput, Halted, Runnable, WaitingForInput,
            };

            match self.computer.process() {
                Runnable => {}
                Halted => break,
                GaveOutput(o) => {
                    if r.len() == 2 {
                        // Ignore positions for now
                        r.clear();

                        if o == 2 {
                            count += 1;
                        }
                    } else {
                        r.push(o);
                    }
                }
                WaitingForInput => panic!("Computer should not need any input"),
            }
        }

        count
    }

    pub fn run(&mut self) -> i64 {
        // Track ball and paddle x-position
        let mut ballx = 0;
        let mut padx = 0;

        let mut r = Vec::new();

        self.computer.reinitialize_memory();
        self.computer.patch_memory(0, 2);

        loop {
            use crate::y2019::intcode_computer::State::{
                GaveOutput, Halted, Runnable, WaitingForInput,
            };

            match self.computer.process() {
                Runnable => {}
                Halted => break,
                GaveOutput(o) => {
                    if r.len() == 2 {
                        let y = r.pop().unwrap();
                        let x = r.pop().unwrap();

                        if x == -1 && y == 0 {
                            self.score = o;
                        } else if o == 3 {
                            padx = x;
                        } else if o == 4 {
                            ballx = x;
                        }
                    } else {
                        r.push(o);
                    }
                }
                WaitingForInput => {
                    // Just follow the ball
                    use std::cmp::Ordering::{Equal, Greater, Less};

                    let joystick = match padx.cmp(&ballx) {
                        Equal => 0,
                        Less => 1,
                        Greater => -1,
                    };

                    self.computer.provide_input(vec![joystick]);
                }
            }
        }

        self.score
    }
}

fn process_input(input: &str) -> Vec<i64> {
    input.split(',').flat_map(str::parse).collect()
}

#[allow(unused)]
fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut g = Game::from(&input);
    let res = g.count_blocks();

    Box::new(res)
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut g = Game::from(&input);
    let res = g.run();

    Box::new(res)
}
