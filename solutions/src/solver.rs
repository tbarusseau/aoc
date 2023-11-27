use std::fmt::Display;

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
Duration: {:?}\n",
            result, self.duration
        )
    }
}

pub trait Solver {
    fn solve_part1(&self, input: &str) -> Box<dyn Display>;
    fn solve_part2(&self, input: &str) -> Box<dyn Display>;

    fn solve_p1(&self, input: &str) -> SolverResults {
        let now = std::time::Instant::now();
        let r1 = SolverResults {
            result: self.solve_part1(&input),
            duration: now.elapsed(),
        };

        r1
    }

    fn solve_p2(&self, input: &str) -> SolverResults {
        let now = std::time::Instant::now();
        let r2 = SolverResults {
            result: self.solve_part2(&input),
            duration: now.elapsed(),
        };

        r2
    }

    fn solve(&self, input: &str) -> (SolverResults, SolverResults) {
        let r1 = self.solve_p1(input);
        let r2 = self.solve_p2(input);

        (r1, r2)
    }

    fn done(&self) -> bool {
        true
    }
}
