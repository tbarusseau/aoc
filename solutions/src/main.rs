#![allow(clippy::single_call_fn)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::fallible_impl_from)]

use crate::solution_runner::run_solution;
use chrono::Datelike;
use solution_runner::run_all_solutions;

mod cli_app;

mod day_template;
mod solution_runner;
mod solver;
mod utils;

mod y2015;
mod y2016;
mod y2017;
mod y2018;
mod y2019;
mod y2020;
mod y2021;
mod y2022;
mod y2023;

mod macros;

#[allow(deprecated)]
fn get_aoc_date() -> chrono::Date<chrono::FixedOffset> {
    use chrono::prelude::*;

    let now = Utc::now();
    let aoc_timezone = now.with_timezone(&FixedOffset::west(3600 * 5));

    aoc_timezone.date()
}

fn main() -> anyhow::Result<()> {
    let opt = <cli_app::Opt as structopt::StructOpt>::from_args();
    let input_fetcher = aoc_input_fetcher::input_fetcher::InputFetcher::try_new()?;
    let date = get_aoc_date();

    match opt.cmd {
        cli_app::Command::Single {
            year,
            day,
            single_part,
        } => {
            run_solution(
                &opt,
                &input_fetcher,
                year.unwrap_or_else(|| date.year()),
                day.unwrap_or_else(|| date.day()),
                single_part,
            )?;
        }
        cli_app::Command::All { year } => {
            run_all_solutions(&opt, &input_fetcher, year.unwrap_or_else(|| date.year()))?;
        }
    }

    Ok(())
}
