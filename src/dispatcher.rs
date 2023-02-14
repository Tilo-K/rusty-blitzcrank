pub mod dispatcher {
    use crate::types::*;
    use reqwest::Client;
    use tokio::time::{sleep, Duration};

    #[tokio::main]
    pub async fn get(url: String, api_key: &str) -> Result<String, BlitzError> {
        let client = Client::new();
        let response = client
            .get(url.clone())
            .header("X-Riot-Token", api_key)
            .send()
            .await;

        let mut resp = match response {
            Ok(d) => d,
            Err(e) => return Err(BlitzError::RequestError(Some(e.to_string()))),
        };

        while resp.status().as_u16() == 429 {
            let wait_time = resp
                .headers()
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok());

            match wait_time {
                Some(t) => {
                    sleep(Duration::from_secs(t)).await;
                    resp = match client.get(url.clone()).send().await {
                        Ok(data) => data,
                        Err(e) => return Err(BlitzError::RequestError(Some(e.to_string()))),
                    };
                }
                None => break,
            }
        }

        if resp.status().is_success() {
            let data = resp.text().await.map(|b| b.to_owned());
            match data {
                Ok(d) => return Ok(d),
                Err(e) => return Err(BlitzError::RequestError(Some(e.to_string()))),
            }
        } else if resp.status().as_u16() == 403 {
            return Err(BlitzError::Forbidden);
        } else if resp.status().as_u16() == 404 {
            return Err(BlitzError::NotFound);
        }

        return Err(BlitzError::BadStatuscode(resp.status().as_u16()));
    }
}
