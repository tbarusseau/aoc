use std::{env::current_dir, fs::create_dir_all, path::PathBuf};

use crate::internal_input_fetcher::InternalInputFetcher;

pub struct InputFetcher {
    input_fetcher: InternalInputFetcher,
    solutions_path: PathBuf,
}

impl InputFetcher {
    pub fn try_new() -> anyhow::Result<InputFetcher> {
        Ok(InputFetcher {
            input_fetcher: InternalInputFetcher::new(),
            solutions_path: current_dir()
                .map_err(|e| anyhow::anyhow!(e))?
                .join("solutions")
                .join("inputs"),
        })
    }

    fn fetch(&self, year: i32, day: i32, force: bool) -> anyhow::Result<()> {
        let path = self
            .solutions_path
            .join(&format!("{}", year))
            .join(&format!("{}.txt", day));

        // Abort if file already exists
        if path.exists() && !force {
            return Ok(());
        }

        let content = self.input_fetcher.get_input(year, day)?;

        create_dir_all(&path.parent().unwrap()).map_err(|e| anyhow::anyhow!(e))?;
        let mut file = std::fs::File::create(&path).map_err(|e| anyhow::anyhow!(e))?;
        std::io::Write::write_all(&mut file, content.as_bytes()).map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }

    pub fn fetch_date<Tz: chrono::TimeZone>(
        &self,
        date: &chrono::Date<Tz>,
        force: bool,
    ) -> anyhow::Result<()> {
        use chrono::Datelike;

        self.fetch(date.year() - 1, date.day() as i32, force)
    }

    pub fn fetch_today(&self, force: bool) -> anyhow::Result<()> {
        use chrono::prelude::*;

        let now = Utc::now();
        let aoc_timezone = now.with_timezone(&FixedOffset::west(3600 * 5));

        self.fetch_date(&aoc_timezone.date(), force)
    }
}
