use anyhow;
use arrow::array::StringArray;
use arrow::record_batch::RecordBatchReader;
use thiserror::Error;

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

pub enum Delete {
    ByIds(StringArray),
    Where(String),
}

pub type SendableRecordBatchReader = Box<dyn RecordBatchReader + Send + Sync>;

pub trait DatasetService: Send + Sync {
    /// Update a dataset with upsert and/or delete operations
    ///
    /// # Arguments
    /// * `dataset_id` - The identifier of the dataset to update
    /// * `upsert` - Optional record batch reader containing rows to insert or update
    /// * `delete` - Optional delete specification (by IDs or WHERE clause)
    ///
    /// # Errors
    /// Returns a [`DatasetError::ArrowError`] if an Arrow operation fails, or
    /// [`DatasetError::Internal`] if an internal service error occurs.
    fn update(
        &self,
        dataset_id: String,
        upsert: Option<SendableRecordBatchReader>,
        delete: Option<Delete>,
    ) -> Result<()>;

    /// Select data from a dataset
    ///
    /// # Arguments
    /// * `dataset_id` - The identifier of the dataset to query
    /// * `where_clause` - Optional SQL WHERE clause filter
    /// * `order_by` - Optional list of (column, direction) pairs for sorting
    /// * `limit` - Optional maximum number of rows to return
    /// * `offset` - Optional number of rows to skip
    ///
    /// # Returns
    /// A `RecordBatchReader` that can be used to iterate over the results
    ///
    /// # Errors
    /// Returns a [`DatasetError::ArrowError`] if an Arrow operation fails, or
    /// [`DatasetError::Internal`] if an internal service error occurs.
    fn select(
        &self,
        dataset_id: String,
        where_clause: Option<String>,
        order_by: Option<Vec<(String, OrderDirection)>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<SendableRecordBatchReader>;
}
