use std::fmt::Display;

pub trait Solver {
    type ProcessedInput;

    fn process_input(&self, input: String) -> Self::ProcessedInput;

    fn solve_part1(&self, input: Self::ProcessedInput) -> Box<dyn Display>;
    fn solve_part2(&self, input: Self::ProcessedInput) -> Box<dyn Display>;

    fn solve(&self, part2: bool, input: String) -> Box<dyn Display> {
        let processed_input = self.process_input(input);

        if part2 {
            self.solve_part2(processed_input)
        } else {
            self.solve_part1(processed_input)
        }
    }
}
