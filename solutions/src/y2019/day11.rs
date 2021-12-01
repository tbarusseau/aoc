use crate::solver::Solver;

pub struct Day11;

crate::impl_day!("11", true);

use crate::y2019::intcode_computer::IntcodeComputer;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, std::hash::Hash)]
enum Color {
    Black,
    White,
}
#[derive(Debug, std::hash::Hash)]
struct Panel {
    color: Color,
    paint_layers: u32,
}
enum Direction {
    U,
    R,
    D,
    L,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, std::hash::Hash)]
struct Pos {
    x: isize,
    y: isize,
}

struct Robot {
    brain: IntcodeComputer,
    pos: Pos,
    dir: Direction,
    grid: HashMap<Pos, Panel>,
}

impl Robot {
    pub fn from(input: Vec<i64>, brain_input: i64) -> Robot {
        Robot {
            brain: IntcodeComputer::from(&input, vec![brain_input]),
            pos: Pos { x: 0, y: 0 },
            dir: Direction::U,
            grid: HashMap::new(),
        }
    }

    pub fn paint_hull(&mut self) {
        let mut r = Vec::new();

        loop {
            use crate::y2019::intcode_computer::State::*;

            match self.brain.process() {
                Runnable => panic!("Unexpected process result"),
                Halted => break,
                GaveOutput(o) => {
                    if r.is_empty() {
                        r.push(o);
                    } else {
                        // Both needed values are recovered
                        let dir = o;
                        let col = r.pop().unwrap();

                        // Match color
                        let col = match col {
                            0 => Color::Black,
                            1 => Color::White,
                            _ => panic!("Impossible color output: {}", col),
                        };

                        // Paint the hull
                        self.grid
                            .entry(self.pos)
                            .and_modify(|g| {
                                g.color = col;
                                g.paint_layers += 1;
                            })
                            .or_insert(Panel {
                                color: col,
                                paint_layers: 1,
                            });

                        // Update direction
                        self.dir = match dir {
                            0 => match self.dir {
                                Direction::U => Direction::L,
                                Direction::R => Direction::U,
                                Direction::D => Direction::R,
                                Direction::L => Direction::D,
                            },
                            1 => match self.dir {
                                Direction::U => Direction::R,
                                Direction::R => Direction::D,
                                Direction::D => Direction::L,
                                Direction::L => Direction::U,
                            },
                            _ => panic!("Impossible direction output: {}", dir),
                        };

                        // Update position
                        match self.dir {
                            Direction::U => self.pos.y += 1,
                            Direction::R => self.pos.x += 1,
                            Direction::D => self.pos.y -= 1,
                            Direction::L => self.pos.x -= 1,
                        }
                    }
                }
                WaitingForInput => {
                    // Check color of current cell to provide input
                    let input = match self.grid.get(&self.pos) {
                        None => 0, // Black
                        Some(e) => match e.color {
                            Color::Black => 0,
                            Color::White => 1,
                        },
                    };

                    self.brain.provide_input(vec![input]);
                }
            }
        }
    }

    pub fn display_hull(&self) -> String {
        let min_x = self
            .grid
            .iter()
            .min_by_key(|&(Pos { x, y: _ }, _)| x)
            .unwrap()
            .0
            .x;
        let max_x = self
            .grid
            .iter()
            .max_by_key(|&(Pos { x, y: _ }, _)| x)
            .unwrap()
            .0
            .x;
        let min_y = self
            .grid
            .iter()
            .min_by_key(|&(Pos { x: _, y }, _)| y)
            .unwrap()
            .0
            .y;
        let max_y = self
            .grid
            .iter()
            .max_by_key(|&(Pos { x: _, y }, _)| y)
            .unwrap()
            .0
            .y;

        let mut out = String::new();
        out.push('\n');

        // Reverse y axis
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                match self.grid.get(&Pos { x, y }) {
                    None => out.push(' '),
                    Some(p) => match p.color {
                        Color::White => out.push('█'),
                        _ => out.push(' '),
                    },
                }
            }

            out.push('\n');
        }

        out
    }
}

fn process_input(input: &str) -> Vec<i64> {
    input.split(',').flat_map(str::parse).collect()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut r = Robot::from(input, 0);
    r.paint_hull();
    let res = r.grid.iter().filter(|&(_, v)| v.paint_layers > 0).count();

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut r = Robot::from(input.to_vec(), 1);
    r.paint_hull();
    Box::new(r.display_hull())
}
