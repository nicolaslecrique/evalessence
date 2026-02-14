use arrow::array::StringArray;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};
use thiserror::Error; // Recommended for custom errors

#[derive(Error, Debug)]
pub enum DatasetError {
    #[error("Arrow error: {0}")]
    ArrowError(#[from] arrow::error::ArrowError),
    // Add other variants as needed
}

pub type Result<T> = std::result::Result<T, DatasetError>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// Combines the stream and its metadata
pub struct DatasetStream {
    pub schema: SchemaRef,
    pub stream: BoxStream<'static, Result<RecordBatch>>,
}

#[async_trait::async_trait]
pub trait DatasetServices: Send + Sync {
    /// Use &str for IDs to avoid ownership overhead.
    /// Accept a stream for upserts to support large data migrations.
    async fn update(
        &self,
        dataset_id: String,
        upsert_stream: DatasetStream,
        delete_ids: StringArray,
    ) -> Result<()>;

    /// Return the custom stream struct so the caller knows the schema
    /// without having to poll the first batch.
    async fn select(
        &self,
        dataset_id: String,
        where_clause: Option<String>,
        order_by: Vec<(String, OrderDirection)>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<DatasetStream>;
}
