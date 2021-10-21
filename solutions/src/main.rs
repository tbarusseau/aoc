use crate::solution_runner::run_solution;

mod cli_app;

mod solution_runner;
mod solver;
mod y2019;
mod y2020;
mod y2021;

fn get_aoc_date() -> chrono::Date<chrono::FixedOffset> {
    use chrono::prelude::*;

    let now = Utc::now();
    let aoc_timezone = now.with_timezone(&FixedOffset::west(3600 * 5));

    aoc_timezone.date()
}

fn main() -> anyhow::Result<()> {
    let opt = <cli_app::Opt as structopt::StructOpt>::from_args();

    let input_fetcher = aoc_input_fetcher::input_fetcher::InputFetcher::try_new()?;

    match opt.cmd {
        cli_app::Command::Single { year, day } => {
            use chrono::Datelike;

            let date = get_aoc_date();

            run_solution(
                &opt,
                &input_fetcher,
                year.unwrap_or_else(|| date.year()),
                day.unwrap_or_else(|| date.day()),
            )?;
        }
        cli_app::Command::All {} => todo!(),
    }

    Ok(())
}
