use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use thiserror::Error;
use anyhow;

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
pub struct App {
    pub id: AppId,
    pub name: String,
    pub envs: Vec<Env>,
    pub datasets: Vec<Dataset>,
    pub pipelines: Vec<Pipeline>,
    pub etag: String,
    pub filename: String,
}

#[async_trait]
pub trait AppServices: Send + Sync {

    /// load all apps in the config directory with the format app-{id}.yaml
    async fn list(&self) -> AppResult<Vec<AppResult<App>>>;
    async fn create(&self, name: String) -> AppResult<App>;
    async fn get(&self, filename: String) -> AppResult<App>;
    async fn delete(&self, filename: String) -> AppResult<()>;
    async fn update(&self, app: App) -> AppResult<App>;
}


#[derive(Error, Debug)]
pub enum AppError {
    #[error("App config file '{filename}' not found")]
    NotFound {filename: String},
    
    #[error("App config file '{filename}' has been modified, please reload it")]
    Conflict {filename: String},

    #[error("Internal service error: {source}")]
    Internal{
        #[source]
        source: anyhow::Error },

    #[error("App config file '{filename}' could not be loaded: invalid format")]
    ValidationError {filename: String},
}

pub type AppResult<T> = Result<T, AppError>;