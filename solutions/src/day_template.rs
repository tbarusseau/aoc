use crate::solver::Solver;

pub struct DayNotDone;

fn process_input(input: String) -> String {
    input
}

impl Solver for DayNotDone {
    fn solve_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        let _input = process_input(input);

        Box::new("Part 1 not done yet".to_string())
    }

    fn solve_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        let _input = process_input(input);

        Box::new("Part 2 not done yet".to_string())
    }

    fn done(&self) -> bool {
        false
    }
}