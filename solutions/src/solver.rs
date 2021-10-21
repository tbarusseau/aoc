use std::fmt::Display;

use crate::cli_app::Opt;

pub struct SolverResults {
    pub result: Box<dyn Display>,
    pub duration: std::time::Duration,
}

impl Display for SolverResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::*;

        let result = format!("{}", self.result).bold().blue();

        write!(
            f,
            "\nResult: {}
Duration: {}μs\n",
            result,
            self.duration.as_micros()
        )
    }
}

pub trait Solver {
    fn solve_part1(&self, input: String) -> Box<dyn Display>;
    fn solve_part2(&self, input: String) -> Box<dyn Display>;

    fn solve(&self, _opt: &Opt, input: String) -> (SolverResults, SolverResults) {
        let now = std::time::Instant::now();
        let r1 = SolverResults {
            result: self.solve_part1(input.clone()),
            duration: now.elapsed(),
        };

        let now = std::time::Instant::now();
        let r2 = SolverResults {
            result: self.solve_part2(input),
            duration: now.elapsed(),
        };

        (r1, r2)
    }

    fn done(&self) -> bool {
        true
    }
}
