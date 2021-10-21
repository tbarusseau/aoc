use crate::solver::Solver;

pub struct Day1;

fn get_total(i: i64) -> i64 {
    if i > 0 {
        i + get_total(i / 3 - 2)
    } else {
        0
    }
}

impl Solver for Day1 {
    type ProcessedInput = Vec<i64>;

    fn solve_part1(&self, input: Self::ProcessedInput) -> Box<dyn std::fmt::Display> {
        let r = input.iter().fold(0, |acc, n| acc + n / 3 - 2);

        Box::new(r)
    }

    fn solve_part2(&self, input: Self::ProcessedInput) -> Box<dyn std::fmt::Display> {
        let r = input.iter().fold(0, |acc, n| acc + get_total(n / 3 - 2));

        Box::new(r)
    }

    fn process_input(&self, input: String) -> Self::ProcessedInput {
        input.lines().flat_map(|l| str::parse(l)).collect()
    }
}
