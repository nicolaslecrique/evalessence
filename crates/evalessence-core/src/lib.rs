use async_trait::async_trait;
use serde_json::Value;
use futures::future::join_all;
use reqwest::Client;

// TODO this is generated code to clean

pub type SampleError = String;

pub struct HttpApi {
    client: Client,
}

impl HttpApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Api for HttpApi {
    async fn run_samples(&self, url: &str, inputs: Vec<Value>) -> Vec<Result<Value, SampleError>> {
        // Create a list of futures (one for each input)
        let tasks = inputs.into_iter().map(|input| {
            let client = self.client.clone();
            let url = url.to_string();

            // We move everything into an async block for this specific sample
            async move {
                let response = client
                    .post(&url)
                    .json(&input)
                    .send()
                    .await
                    .map_err(|e| format!("Network error: {}", e))?;

                let status = response.status();
                if status.is_success() {
                    response
                        .json::<Value>()
                        .await
                        .map_err(|e| format!("JSON parse error: {}", e))
                } else {
                    Err(format!("Server returned error status: {}", status))
                }
            }
        });

        // run all tasks in parallel and wait for all to finish
        join_all(tasks).await
    }
}