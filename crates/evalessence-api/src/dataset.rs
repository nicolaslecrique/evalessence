use anyhow;
use arrow::array::StringArray;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use futures::stream::BoxStream;
use thiserror::Error; // Recommended for custom errors

#[derive(Error, Debug)]
pub enum DatasetError {
    #[error("Arrow error: {0}")]
    ArrowError(#[from] arrow::error::ArrowError),

    #[error("Internal service error: {source}")]
    Internal {
        #[source]
        source: anyhow::Error,
    },
    // Add other variants as needed
}

pub type Result<T> = std::result::Result<T, DatasetError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderDirection {
    Asc,
    Desc,
}

pub struct SendableRecordBatchStream {
    pub schema: SchemaRef,
    pub stream: BoxStream<'static, Result<RecordBatch>>,
}

pub enum Delete {
    ByIds(StringArray),
    Where(String),
}

#[async_trait::async_trait]
pub trait DatasetService: Send + Sync {
    async fn update(
        &self,
        dataset_id: String,
        upsert: Option<SendableRecordBatchStream>,
        delete: Option<Delete>,
    ) -> Result<()>;

    async fn select(
        &self,
        dataset_id: String,
        where_clause: Option<String>,
        order_by: Option<Vec<(String, OrderDirection)>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<SendableRecordBatchStream>;
}
