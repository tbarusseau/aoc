mod cli_app;

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

            input_fetcher.fetch(
                year.unwrap_or_else(|| date.year()),
                day.unwrap_or_else(|| date.day()),
                opt.force_fetch,
            )?;
        }
        cli_app::Command::All {} => todo!(),
    }

    Ok(())
}
