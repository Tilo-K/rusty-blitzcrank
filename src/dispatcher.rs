pub mod dispatcher {
    use reqwest::Client;
    use tokio::time::{sleep, Duration};

    #[tokio::main]
    pub async fn get(url: String, api_key: &str) -> Result<String, reqwest::Error> {
        let client = Client::new();
        let mut resp = client
            .get(url.clone())
            .header("X-Riot-Token", api_key)
            .send()
            .await?;

        while resp.status() == 429 {
            let wait_time = resp
                .headers()
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok());

            match wait_time {
                Some(t) => {
                    sleep(Duration::from_secs(t)).await;
                    resp = client.get(url.clone()).send().await?;
                }
                None => break,
            }
        }

        if resp.status().is_success() {
            resp.text().await.map(|b| b.to_owned())
        } else {
            Ok("Not found".to_owned())
        }
    }
}
