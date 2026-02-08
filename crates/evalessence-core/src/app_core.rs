use async_trait::async_trait;
use evalessence_api::app::{App, AppError, AppId, AppResult, AppServices, Dataset, Env, Pipeline};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_saphyr;
use slug::slugify;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::fs::DirEntry;
use tokio_stream::StreamExt;

use tokio_stream::wrappers::ReadDirStream;
/// The internal format saved to disk (no etag, no filename)
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    pub id: AppId,
    pub name: String,
    pub envs: Vec<Env>,
    pub datasets: Vec<Dataset>,
    pub pipelines: Vec<Pipeline>,
}

pub struct FileAppService {
    config_dir: PathBuf,
}

impl FileAppService {
    pub fn new(config_dir: impl AsRef<Path>) -> Self {
        Self {
            config_dir: config_dir.as_ref().to_path_buf(),
        }
    }

    // Helper to clean the name and add a random suffix
    fn generate_id(&self, name: &str) -> AppId {
        AppId(format!("{}-{}", slugify(name), nanoid!(4)))
    }

    // Helper to calculate ETag from raw bytes
    fn calculate_etag(&self, bytes: &[u8]) -> String {
        return blake3::hash(bytes).to_string();
    }

    fn get_path(&self, filename: &str) -> PathBuf {
        self.config_dir.join(filename)
    }
}

#[async_trait]
impl AppServices for FileAppService {
    async fn list(&self) -> AppResult<Vec<AppResult<App>>> {
        let entries = fs::read_dir(&self.config_dir)
            .await
            .map_err(|e| AppError::Internal { source: e.into() })?;

        let apps = ReadDirStream::new(entries)
            .filter_map(|res: Result<DirEntry, std::io::Error>| {
                // 1. If the OS fails to even give us an entry, we have no filename.
                // Since we can't check if it's an "app-*.yaml" file, we must skip it.
                let entry = res.ok()?;

                let name = entry.file_name().to_string_lossy().into_owned();

                // 2. Only proceed if it matches your pattern
                (name.starts_with("app-") && name.ends_with(".yaml")).then_some(name)
            })
            .then(|filename| async move {
                // 3. Now we only call 'get' for valid filenames.
                // Any error here will be preserved in your Vec<AppResult<App>>.
                self.get(filename).await
            })
            .collect()
            .await;

        Ok(apps)
    }

    async fn create(&self, name: String) -> AppResult<App> {
        let id = self.generate_id(&name);
        let filename = format!("app-{id}.yaml");

        // create and save the AppConfig with empty envs/datasets/pipelines
        let config = AppConfig {
            id: id.clone(),
            name: name.clone(),
            envs: vec![],
            datasets: vec![],
            pipelines: vec![],
        };

        // 1. Serialize to an in-memory string (Sync)
        let yaml_data = serde_saphyr::to_string(&config)
            .map_err(|e| AppError::Internal { source: e.into() })?;

        // 2. Write to the file (Async)
        fs::write(self.get_path(&filename), yaml_data)
            .await
            .map_err(|e| AppError::Internal { source: e.into() })?;

        self.get(filename).await
    }

    async fn get(&self, filename: String) -> AppResult<App> {
        let path = self.get_path(&filename);

        let yaml_bytes = fs::read(&path).await.map_err(|e| AppError::FileIoError {
            filename: filename.clone(),
            source: e.into(),
        })?;

        let config: AppConfig =
            serde_saphyr::from_slice(&yaml_bytes).map_err(|e| AppError::ValidationError {
                filename: filename.clone(),
                source: e.into(),
            })?;

        Ok(App {
            id: config.id,
            name: config.name,
            envs: config.envs,
            datasets: config.datasets,
            pipelines: config.pipelines,
            etag: self.calculate_etag(&yaml_bytes),
            filename,
        })
    }

    async fn delete(&self, filename: String) -> AppResult<()> {
        fs::remove_file(self.get_path(&filename))
            .await
            .map_err(|e| AppError::FileIoError {
                filename,
                source: e.into(),
            })
    }

    async fn update(&self, app: App) -> AppResult<App> {
        let path = self.get_path(&app.filename);

        // Conflict check: hash existing file and compare with incoming etag
        if path.exists() {
            let current_bytes = fs::read(&path)
                .await
                .map_err(|e| AppError::Internal { source: e.into() })?;
            let current_etag = self.calculate_etag(&current_bytes);
            if current_etag != app.etag {
                return Err(AppError::Conflict {
                    filename: app.filename,
                });
            }
        }

        // Map App to AppConfig (stripping metadata)
        let config = AppConfig {
            id: app.id.clone(),
            name: app.name.clone(),
            envs: app.envs.clone(),
            datasets: app.datasets.clone(),
            pipelines: app.pipelines.clone(),
        };

        let yaml_bytes =
            serde_yaml::to_vec(&config).map_err(|e| AppError::Internal { source: e.into() })?;

        fs::write(&path, &yaml_bytes)
            .await
            .map_err(|e| AppError::Internal { source: e.into() })?;

        // Return updated App with new ETag
        Ok(App {
            etag: self.calculate_etag(&yaml_bytes),
            ..app
        })
    }
}
