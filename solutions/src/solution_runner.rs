use aoc_input_fetcher::input_fetcher::InputFetcher;
use chrono::Datelike;

use crate::cli_app::Opt;
use crate::get_aoc_date;
use crate::solver::Solver;

pub fn run_solution(
    opt: &Opt,
    input_fetcher: &InputFetcher,
    year: i32,
    day: u32,
) -> anyhow::Result<()> {
    let mut solvers: Vec<Box<dyn Solver + Send + Sync>> = vec![];
    crate::solvers_gen!(solvers, 2015, 2019, 2020, 2021, 2022);

    let start_index = match year {
        2015 => 0,
        2019 => 25,
        2020 => 50,
        2021 => 75,
        2022 => 100,
        y => panic!("Year not available: {}", y),
    };

    if day == 0 || day > 25 {
        panic!("Day must be between 1 and 25");
    }

    let solver = &solvers[start_index + day as usize - 1];

    if !solver.done() {
        println!("No solution for year {}, day {}. Exiting.", year, day);

        return Ok(());
    }

    let input = input_fetcher.fetch(year, day, opt.force_fetch)?;

    use colored::*;

    let (s1, s2) = solver.solve(opt, input);
    println!("{}", format!("Solving year {}, day {}\n", year, day).bold());
    println!("{} {}", "Part 1:".green(), s1);
    println!("{} {}", "Part 2:".red(), s2);

    Ok(())
}

pub fn run_all_solutions(opt: &Opt, input_fetcher: &InputFetcher, year: i32) -> anyhow::Result<()> {
    let mut solvers: Vec<Box<dyn Solver + Send + Sync>> = vec![];
    crate::solvers_gen!(solvers, 2019, 2020, 2021, 2022);

    let start_index = match year {
        2019 => 0,
        2020 => 25,
        2021 => 50,
        2022 => 75,
        y => panic!("Year not available: {}", y),
    };

    let date = get_aoc_date();

    for i in 1..=25 {
        let solver = &solvers[i as usize - 1 + start_index];

        if !solver.done() {
            continue;
        }

        let input = input_fetcher.fetch(date.year(), i, opt.force_fetch)?;

        use colored::*;

        let (s1, s2) = solver.solve(opt, input);
        println!("{}", format!("Solving year {}, day {}\n", year, i).bold());
        println!("{} {}", "Part 1:".green(), s1);
        println!("{} {}", "Part 2:".red(), s2);
    }

    Ok(())
}
