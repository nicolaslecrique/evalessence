use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct AppId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct DatasetId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct EnvId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct PipelineId(pub String);



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub id: DatasetId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Env {
    pub id: EnvId,
    pub url: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: PipelineId,
    pub name: String,
    pub route: String,
    pub env_id: EnvId,
    pub dataset_id: DatasetId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppKey {
    pub id: AppId,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub key: AppKey,
    pub name: String,
    pub envs: Vec<Env>,
    pub datasets: Vec<Dataset>,
    pub pipelines: Vec<Pipeline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppHeader {
    pub id: AppId,
    pub name: String,
}


#[async_trait]
pub trait AppServices: Send + Sync {
    async fn list(&self) -> AppResult<Vec<AppHeader>>;
    async fn create(&self, name: String) -> AppResult<App>;
    async fn get(&self, app_id: &str) -> AppResult<App>;
    async fn delete(&self, app_id: &str) -> AppResult<()>;
    async fn update(&self, app: App) -> AppResult<App>;
}


#[derive(Error, Debug)]
pub enum AppError {
    #[error("App with ID {0} not found")]
    NotFound(String),
    
    #[error("Version mismatch for App {0}: expected {1}")]
    Conflict(String, u64),

    #[error("Internal service error: {0}")]
    Internal(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),
}

pub type AppResult<T> = Result<T, AppError>;