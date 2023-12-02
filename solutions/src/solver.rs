use std::fmt::Display;

pub struct Results {
    pub result: Box<dyn Display>,
    pub duration: std::time::Duration,
}

impl Display for Results {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;

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

    fn solve_p1(&self, input: &str) -> Results {
        let now = std::time::Instant::now();
        Results {
            result: self.solve_part1(input),
            duration: now.elapsed(),
        }
    }

    fn solve_p2(&self, input: &str) -> Results {
        let now = std::time::Instant::now();
        Results {
            result: self.solve_part2(input),
            duration: now.elapsed(),
        }
    }

    fn solve(&self, input: &str) -> (Results, Results) {
        let r1 = self.solve_p1(input);
        let r2 = self.solve_p2(input);

        (r1, r2)
    }

    fn done(&self) -> bool {
        true
    }
}

// pub struct SolverResultsWithResult {
//     pub result: anyhow::Result<Box<dyn Display>>,
//     pub duration: std::time::Duration,
// }

// impl Display for SolverResultsWithResult {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         use colored::*;

//         let result = if let Err(err) = &self.result {
//             format!("[ERR] {}", err).bold().red()
//         } else {
//             format!("{}", self.result.as_ref().unwrap()).bold().blue()
//         };

//         write!(
//             f,
//             "\nResult: {}
// Duration: {:?}\n",
//             result, self.duration
//         )
//     }
// }

// pub trait SolverWithResult {
//     fn solve_part1(&self, input: &str) -> anyhow::Result<Box<dyn Display>>;
//     fn solve_part2(&self, input: &str) -> anyhow::Result<Box<dyn Display>>;

//     fn solve_p1(&self, input: &str) -> SolverResultsWithResult {
//         let now = std::time::Instant::now();
//         let r1 = SolverResultsWithResult {
//             result: self.solve_part1(&input),
//             duration: now.elapsed(),
//         };

//         r1
//     }

//     fn solve_p2(&self, input: &str) -> SolverResultsWithResult {
//         let now = std::time::Instant::now();
//         let r2 = SolverResultsWithResult {
//             result: self.solve_part2(&input),
//             duration: now.elapsed(),
//         };

//         r2
//     }

//     fn solve(&self, input: &str) -> (SolverResultsWithResult, SolverResultsWithResult) {
//         let r1 = self.solve_p1(input);
//         let r2 = self.solve_p2(input);

//         (r1, r2)
//     }

//     fn done(&self) -> bool {
//         true
//     }
// }
