#[derive(Debug, structopt::StructOpt)]
pub struct Opt {
    #[structopt(long)]
    pub force_fetch: bool,
    #[structopt(long)]
    pub no_benchmark: bool,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, structopt::StructOpt)]
pub enum Command {
    /// Run a single day.
    // By default, run the current day if applicable.
    Single {
        #[structopt(short, long)]
        year: Option<i32>,
        #[structopt(short, long)]
        day: Option<u32>,
    },
    /// Run all available days.
    All {
        #[structopt(short, long)]
        year: Option<i32>,
    },
}
