mod cli_app;

fn main() -> anyhow::Result<()> {
    let opt = <cli_app::Opt as structopt::StructOpt>::from_args();

    let input_fetcher = aoc_input_fetcher::input_fetcher::InputFetcher::try_new()?;

    input_fetcher
        .fetch_today(true)
        .expect("Couldn't get today's input");

    Ok(())
}
