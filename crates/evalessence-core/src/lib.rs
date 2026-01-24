use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use serde_json::Value;
use reqwest::Client;
use evalessence_api::{Api, SampleError};

#[derive(Default)]
pub struct LocalApi {
    client: Client,
}

#[async_trait]
impl Api for LocalApi {
    async fn run_samples(&self, url: &str, inputs: Vec<Value>) -> Vec<Result<Value, SampleError>> {
        const CONCURRENT_REQUESTS: usize = 10;

        stream::iter(inputs)
            // We pass the reference to self and the url into the stream
            .map(|input| self.run_single_sample(url, input))
            .buffered(CONCURRENT_REQUESTS)
            .collect()
            .await
    }
}

impl LocalApi {
    /// Helper to process a single request
    async fn run_single_sample(&self, url: &str, input: Value) -> Result<Value, SampleError> {
        let res = self.client.post(url)
            .json(&input)
            .send()
            .await
            .map_err(|e| format!("Network error: {e}"))?
            .error_for_status()
            .map_err(|e| format!("Status error: {e}"))?;

        res.json::<Value>()
            .await
            .map_err(|e| format!("Parse error: {e}"))
    }
}