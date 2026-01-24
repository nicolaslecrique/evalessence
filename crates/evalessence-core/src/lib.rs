use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use serde_json::Value;
use reqwest::Client;
use evalessence_api::{Api, SampleError};

pub struct LocalApi {
    client: Client,
}

impl Default for LocalApi {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Api for LocalApi {
    async fn run_samples(&self, url: &str, inputs: Vec<Value>) -> Vec<Result<Value, SampleError>> {
        // Limit concurrent requests to 10 so we don't overwhelm the OS or Server
        let concurrent_requests = 10;

        stream::iter(inputs)
            .map(|input| {
                let client = self.client.clone();
                async move {
                    let res = client.post(url)
                        .json(&input)
                        .send()
                        .await
                        .map_err(|e| format!("Network error: {e}"))?;

                    if res.status().is_success() {
                        res.json::<Value>()
                           .await
                           .map_err(|e| format!("Parse error: {e}"))
                    } else {
                        Err(format!("Status error: {}", res.status()))
                    }
                }
            })
            .buffered(concurrent_requests) // Runs up to 10 at a time, maintains order
            .collect::<Vec<_>>()
            .await
    }
}