mod cli_app;
mod input_fetch_wrapper;

fn main() {
    let opt = <cli_app::Opt as structopt::StructOpt>::from_args();

    let input_fetcher = input_fetch_wrapper::InputFetchWrapper::new();

    input_fetcher
        .fetch_today(false)
        .expect("Couldn't get today's input");
}
