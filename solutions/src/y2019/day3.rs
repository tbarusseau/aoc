use crate::{solver::Solver, utils::manhattan::manhattan_distance};

pub struct Day3;

crate::impl_day!("3", true);

use std::collections::HashMap;

enum Direction {
    U,
    R,
    D,
    L,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    steps_sum: usize,
    wires_count: usize,
}

struct Movement {
    direction: Direction,
    steps: isize,
}

impl Movement {
    pub fn from(input: &str) -> Movement {
        Movement {
            direction: match input.chars().next().unwrap() {
                'U' => Direction::U,
                'R' => Direction::R,
                'D' => Direction::D,
                'L' => Direction::L,
                _ => panic!("Invalid direction"),
            },
            steps: input
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<isize>()
                .unwrap(),
        }
    }
}

struct Wire {
    id: usize,
    movements: Vec<Movement>,
}

impl Wire {
    pub fn from(input: &str, id: usize) -> Wire {
        Wire {
            id,
            movements: input.split(',').map(Movement::from).collect(),
        }
    }
}

struct Board {
    wires: Vec<Wire>,
    intersections: HashMap<(isize, isize), Cell>,
    current_wire_total_steps: usize,
}

impl Board {
    pub fn from(input: &str) -> Board {
        Board {
            wires: input
                .lines()
                .enumerate()
                .map(|(i, l)| Wire::from(l, i))
                .collect(),
            intersections: HashMap::new(),
            current_wire_total_steps: 0,
        }
    }

    pub fn trace_wires(&mut self) {
        for wire in &self.wires {
            self.current_wire_total_steps = 0;
            let mut position = (0_isize, 0_isize);

            for movement in &wire.movements {
                let steps = movement.steps;
                let current_total = self.current_wire_total_steps;

                for s in 1..=steps {
                    match movement.direction {
                        Direction::U => position.1 -= 1,
                        Direction::D => position.1 += 1,
                        Direction::L => position.0 -= 1,
                        Direction::R => position.0 += 1,
                    }

                    self.intersections
                        .entry(position)
                        .and_modify(|e| {
                            e.wires_count |= 1 << wire.id;
                            e.steps_sum += current_total + s as usize;
                        })
                        .or_insert(Cell {
                            wires_count: 1 << wire.id,
                            steps_sum: current_total + s as usize,
                        });
                }

                self.current_wire_total_steps += steps as usize;
            }
        }
    }

    pub fn get_distance_to_closest_intersection(&self) -> usize {
        manhattan_distance(
            (0, 0),
            *self
                .intersections
                .iter()
                .filter(|&(_, c)| c.wires_count > 2)
                .min_by_key(|((x, y), _)| manhattan_distance((0, 0), (*x, *y)))
                .unwrap()
                .0,
        )
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut b = Board::from(input);
    b.trace_wires();

    Box::new(b.get_distance_to_closest_intersection())
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut b = Board::from(input);
    b.trace_wires();

    let res = b
        .intersections
        .iter()
        .filter(|&(_, c)| c.wires_count > 2)
        .min_by_key(|(_, c)| c.steps_sum)
        .unwrap()
        .1
        .steps_sum;

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use crate::utils::manhattan::manhattan_distance;

    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(6, manhattan_distance((0, 0), (3, 3)));
        assert_eq!(6, manhattan_distance((0, 0), (-3, -3)));
    }

    #[test]
    fn test_wires() {
        let mut board = Board::from(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
        );
        board.trace_wires();
        assert_eq!(159, board.get_distance_to_closest_intersection());

        let mut board = Board::from(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        board.trace_wires();
        assert_eq!(135, board.get_distance_to_closest_intersection());
    }
}
