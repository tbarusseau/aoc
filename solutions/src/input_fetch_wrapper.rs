use std::{env::current_dir, fs::create_dir_all};

pub struct InputFetchWrapper {
    input_fetcher: aoc_input_fetcher::InputFetcher,
}

impl InputFetchWrapper {
    pub fn new() -> InputFetchWrapper {
        InputFetchWrapper {
            input_fetcher: aoc_input_fetcher::InputFetcher::new(),
        }
    }

    fn fetch(&self, year: i32, day: i32, force: bool) -> anyhow::Result<()> {
        let path = current_dir()
            .map_err(|e| anyhow::anyhow!(e))?
            .join("solutions")
            .join("inputs");

        // Abort if file already exists
        if path.exists() && !force {
            return Ok(());
        }

        let content = self.input_fetcher.get_input(year, day)?;

        create_dir_all(&path).map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }

    pub fn fetch_date<Tz: chrono::TimeZone>(
        &self,
        date: &chrono::Date<Tz>,
        force: bool,
    ) -> anyhow::Result<()> {
        use chrono::Datelike;

        self.fetch(date.year(), date.day() as i32, force)
    }

    pub fn fetch_today(&self, force: bool) -> anyhow::Result<()> {
        use chrono::prelude::*;

        let now = Utc::now();
        let aoc_timezone = now.with_timezone(&FixedOffset::west(3600 * 5));

        let year = aoc_timezone.day() as i32;
        let day = aoc_timezone.year();

        self.fetch(year, day, force)
    }
}
