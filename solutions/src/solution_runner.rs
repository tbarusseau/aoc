use aoc_input_fetcher::input_fetcher::InputFetcher;

use crate::{cli_app::Opt, solver::Solver};

fn get_start_index(year: i32) -> usize {
    let mut available_years = 2015..=2024;
    available_years.find(|v| *v == year).map_or_else(
        || panic!("Year not available: {}", year),
        |v| ((v - 2015) * 25) as usize,
    )
}

pub fn run_solution(
    opt: &Opt,
    input_fetcher: &InputFetcher,
    year: i32,
    day: u32,
    single_part: Option<u32>,
) -> anyhow::Result<()> {
    let mut solvers: Vec<Box<dyn Solver + Send + Sync>> = vec![];
    crate::solvers_gen!(solvers, 2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024);

    assert!(!(day == 0 || day > 25), "Day must be between 1 and 25");

    let solver = &solvers[get_start_index(year) + day as usize - 1];

    if !solver.done() {
        println!("No solution for year {year}, day {day}. Exiting.");

        return Ok(());
    }

    let input = input_fetcher.fetch(year, day, opt.force_fetch)?;

    use colored::Colorize;

    if let Some(part) = single_part {
        if part != 1 && part != 2 {
            return Err(anyhow::anyhow!("single-part parameter must be 1 or 2"));
        }

        let r = if part == 1 {
            solver.solve_p1(&input)
        } else {
            solver.solve_p2(&input)
        };

        println!(
            "{}",
            format!("Solving year {year}, day {day}, part {part}\n").bold()
        );
        println!("{} {}", &format!("Part {part}:").green(), r);

        return Ok(());
    }

    let (s1, s2) = solver.solve(&input);
    println!("{}", format!("Solving year {year}, day {day}\n").bold());
    println!("{} {}", "Part 1:".green(), s1);
    println!("{} {}", "Part 2:".red(), s2);

    Ok(())
}

pub fn run_all_solutions(opt: &Opt, input_fetcher: &InputFetcher, year: i32) -> anyhow::Result<()> {
    use colored::Colorize;

    let mut solvers: Vec<Box<dyn Solver + Send + Sync>> = vec![];
    crate::solvers_gen!(solvers, 2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023);

    for i in 1..=25 {
        let solver = &solvers[i as usize - 1 + get_start_index(year)];

        if !solver.done() {
            continue;
        }

        let input = input_fetcher.fetch(year, i, opt.force_fetch)?;

        let (s1, s2) = solver.solve(&input);
        println!("{}", format!("Solving year {year}, day {i}\n").bold());
        println!("{} {}", "Part 1:".green(), s1);
        println!("{} {}", "Part 2:".red(), s2);

        if i != 25 {
            println!("{}", "-------------------------".bold());
        }
    }

    Ok(())
}
