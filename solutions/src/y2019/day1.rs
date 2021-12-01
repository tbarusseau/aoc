use crate::solver::Solver;

pub struct Day1;

fn process_input(input: String) -> Vec<i64> {
    input.lines().flat_map(str::parse).collect()
}

fn get_total(i: i64) -> i64 {
    if i > 0 {
        i + get_total(i / 3 - 2)
    } else {
        0
    }
}

impl Solver for Day1 {
    fn solve_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        let input = process_input(input);
        let r = input.iter().fold(0, |acc, n| acc + n / 3 - 2);

        Box::new(r)
    }

    fn solve_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        let input = process_input(input);
        let r = input.iter().fold(0, |acc, n| acc + get_total(n / 3 - 2));

        Box::new(r)
    }
}
