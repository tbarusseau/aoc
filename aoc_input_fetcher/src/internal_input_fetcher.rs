use std::env;

use dotenv::dotenv;

pub struct InternalInputFetcher {
    api_base_url: String,
    session_cookie: String,
}

impl InternalInputFetcher {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            api_base_url: "https://adventofcode.com".to_string(),
            session_cookie: env::var("AOC_SESSION_COOKIE")
                .expect("No AOC_SESSION_COOKIE found in environment variables"),
        }
    }

    #[cfg(test)]
    fn with_api_url(api_base_url: &str) -> InternalInputFetcher {
        dotenv().ok();

        InternalInputFetcher {
            api_base_url: api_base_url.to_string(),
            ..Default::default()
        }
    }

    pub fn get_input(&self, year: i32, day: u32) -> anyhow::Result<String> {
        let request_url = format!("{}/{}/day/{}/input", &self.api_base_url, year, day);

        let resp = attohttpc::get(request_url)
            .header("Cookie", format!("session={}", &self.session_cookie))
            .send()
            .map_err(|e| anyhow::anyhow!("attohttpc error: {:?}", e.kind()))?;

        if resp.is_success() {
            Ok(resp
                .text()
                .map_err(|e| anyhow::anyhow!("attohttpc error: {:?}", e.kind()))?)
        } else {
            Err(anyhow::anyhow!(
                "Bad response from server while fetching input y{}, d{}: {:?}\n{:?}",
                year,
                day,
                resp.status(),
                resp.text().map_err(|e| anyhow::anyhow!(e))?,
            ))
        }
    }
}

impl Default for InternalInputFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use warp::Filter;

    use super::*;

    #[tokio::test]
    async fn test_get_input() {
        // Insert a false session cookie in the environment
        env::set_var("AOC_SESSION_COOKIE", "test_cookie");

        // Create a mock router
        let input_handler = warp::path!(String / "day" / String / "input")
            .and(warp::filters::header::header::<String>("Cookie"))
            .map(|year, day, _session_cookie| Ok(format!("year: {}, day: {}", year, day)));

        let (tx, rx) = tokio::sync::oneshot::channel();

        let (addr, server) =
            warp::serve(input_handler).bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), async {
                rx.await.ok();
            });
        tokio::task::spawn(server);

        // Now, try to fetch inputs from our mock server
        std::thread::sleep(std::time::Duration::from_millis(100));
        let input_fetcher =
            InternalInputFetcher::with_api_url(&format!("http://{}", addr.to_string()));
        let input_y2021_d20 = input_fetcher.get_input(2021, 20).unwrap();

        assert_eq!(input_y2021_d20, "year: 2021, day: 20".to_string());

        let _ = tx.send(());
    }
}
