use std::fmt::Display;

pub trait Solver {
    fn solve_part1(&self, input: String) -> Box<dyn Display>;
    fn solve_part2(&self, input: String) -> Box<dyn Display>;

    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        (self.solve_part1(input.clone()), self.solve_part2(input))
    }

    fn done(&self) -> bool {
        true
    }
}
