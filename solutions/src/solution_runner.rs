use aoc_input_fetcher::input_fetcher::InputFetcher;

use crate::{
    cli_app::Opt,
    day_template::DayNotDone,
    solver::Solver,
    y2019::{self, day2::Day2},
};

lazy_static::lazy_static! {
    static ref SOLVERS: [Box<dyn Solver + Send + Sync>; 75] = [
        // 2019
        Box::new(y2019::day1::Day1),
        Box::new(Day2),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        // 2020
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        // 2021
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
        Box::new(DayNotDone),
    ];
}

pub fn run_solution(
    opt: &Opt,
    input_fetcher: &InputFetcher,
    year: i32,
    day: u32,
) -> anyhow::Result<()> {
    let start_index = match year {
        2019 => 0,
        2020 => 25,
        2021 => 50,
        y => panic!("Year not available: {}", y),
    };

    if day > 25 {
        panic!("Day must be between 0 and 25");
    }

    let solver = &SOLVERS[start_index + day as usize];

    if !solver.done() {
        println!("No solution for year {}, day {}. Exiting.", year, day);

        return Ok(());
    }

    let input = input_fetcher.fetch(year, day, opt.force_fetch)?;

    let (s1, s2) = solver.solve(input);
    println!("Solving year {}, day {}", year, day);
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);

    Ok(())
}

pub fn run_all_solutions(opt: &Opt, input_fetcher: &InputFetcher, year: i32) -> anyhow::Result<()> {
    let start_index = match year {
        2019 => 0,
        2020 => 25,
        2021 => 50,
        y => panic!("Year not available: {}", y),
    };

    for i in 1..=25 {
        let solver = &SOLVERS[i as usize - 1 + start_index];

        if !solver.done() {
            continue;
        }

        let input = input_fetcher.fetch(2019, i, opt.force_fetch)?;

        let (s1, s2) = solver.solve(input);
        println!("Solving year {}, day {}", year, i);
        println!("Part 1: {}", s1);
        println!("Part 2: {}", s2);
    }

    Ok(())
}
