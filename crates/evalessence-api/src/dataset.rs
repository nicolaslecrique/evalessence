use arrow::record_batch::RecordBatchReader;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

/// Represents the sort order
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[async_trait::async_trait]
pub trait DatasetServices: Send + Sync {
    /// Create or update the table
    async fn update(
        &self,
        dataset_id: String,
        upsert_by_id: impl RecordBatchReader,
        delete_by_id: impl arrow::array::Array,
    ) -> ();

    /// Select records from a dataset
    async fn select(
        &self,
        dataset_id: String,
        where_clause: Option<String>,
        order_by: Vec<(String, OrderDirection)>,
        limit: Option<usize>,
    ) -> impl RecordBatchReader;
}
// todo improve with https://gemini.google.com/app/3d61f0575b50eca8
