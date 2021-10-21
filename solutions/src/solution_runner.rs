use aoc_input_fetcher::input_fetcher::InputFetcher;

use crate::{cli_app::Opt, solver::Solver};

pub fn run_solution(
    opt: &Opt,
    input_fetcher: &InputFetcher,
    year: i32,
    day: u32,
) -> anyhow::Result<()> {
    let solver = match year {
        2019 => match day {
            1 => crate::y2019::day1::Day1,
            _ => panic!("Not implemented"),
        },
        _ => panic!("Not implemented"),
    };

    let input = input_fetcher.fetch(year, day, opt.force_fetch)?;

    println!("Solving year {}, day {}", year, day);
    println!("Part 1: {}", solver.solve(false, input.clone()));
    println!("Part 2: {}", solver.solve(true, input));

    Ok(())
}
