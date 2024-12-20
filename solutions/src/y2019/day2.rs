use crate::{solver::Solver, y2019::intcode_computer::IntcodeComputer};

pub struct Day2;

fn process_input(input: &str) -> Vec<i64> {
    input.split(',').flat_map(str::parse).collect()
}

impl Solver for Day2 {
    fn solve_part1(&self, input: &str) -> Box<dyn std::fmt::Display> {
        let input = process_input(input);

        let mut c = IntcodeComputer::from(&input, vec![]);

        c.patch_memory(1, 12);
        c.patch_memory(2, 2);

        c.process();
        Box::new(c.index(0))
    }

    fn solve_part2(&self, input: &str) -> Box<dyn std::fmt::Display> {
        let input = process_input(input);

        let mut c = IntcodeComputer::from(&input, vec![]);
        let target = 19_690_720;
        let mut result = None;

        'outer: for noun in 0..=99 {
            for verb in 0..=99 {
                c.reinitialize_memory();

                c.patch_memory(1, noun);
                c.patch_memory(2, verb);

                c.process();

                if c.index(0) == target {
                    result = Some(100 * noun + verb);
                    break 'outer;
                }
            }
        }

        let result = result.unwrap();
        Box::new(result)
    }
}
