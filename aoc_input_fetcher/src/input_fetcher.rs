use std::{env::current_dir, fs::create_dir_all, io::Read, path::PathBuf};

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
                .join("inputs"),
        })
    }

    pub fn fetch(&self, year: i32, day: u32, force: bool) -> anyhow::Result<String> {
        let path = self
            .solutions_path
            .join(&format!("{}", year))
            .join(&format!("{}.txt", day));

        // Abort if file already exists
        if path.exists() && !force {
            let mut file = std::fs::File::open(&path).map_err(|e| anyhow::anyhow!(e))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| anyhow::anyhow!(e))?;

            return Ok(content);
        }

        let content = self.input_fetcher.get_input(year, day)?;

        create_dir_all(&path.parent().unwrap()).map_err(|e| anyhow::anyhow!(e))?;
        let mut file = std::fs::File::create(&path).map_err(|e| anyhow::anyhow!(e))?;
        std::io::Write::write_all(&mut file, content.as_bytes()).map_err(|e| anyhow::anyhow!(e))?;

        Ok(content)
    }

    pub fn fetch_date<Tz: chrono::TimeZone>(
        &self,
        date: &chrono::Date<Tz>,
        force: bool,
    ) -> anyhow::Result<String> {
        use chrono::Datelike;

        self.fetch(date.year(), date.day(), force)
    }

    pub fn fetch_today(&self, force: bool) -> anyhow::Result<String> {
        use chrono::prelude::*;

        let now = Utc::now();
        let aoc_timezone = now.with_timezone(&FixedOffset::west(3600 * 5));

        self.fetch_date(&aoc_timezone.date(), force)
    }
}
