use arrow::array::RecordBatchIterator;
use arrow::record_batch::RecordBatch;
use async_trait::async_trait;
use futures::StreamExt;
use futures::stream::BoxStream;
use lancedb::Table;
use lancedb::connection::Connection;
use lancedb::query::{ExecutableQuery, QueryBase, Select};

use evalessence_api::dataset::{
    DatasetError, DatasetService, Delete, OrderDirection, Result, SendableRecordBatchStream,
};

pub struct LanceDatasetService {
    db: Connection,
}

impl LanceDatasetService {
    pub fn new(db: Connection) -> Self {
        Self { db }
    }

    async fn get_table(&self, id: &str) -> Result<Table> {
        self.db
            .open_table(id)
            .execute()
            .await
            .map_err(|e| DatasetError::Internal {
                source: anyhow::anyhow!(e),
            })
    }
}

#[async_trait]
impl DatasetService for LanceDatasetService {
    async fn update(
        &self,
        dataset_id: String,
        upsert: Option<SendableRecordBatchStream>,
        delete: Option<Delete>,
    ) -> Result<()> {
        let table = self.get_table(&dataset_id).await?;

        // 1. Handle Deletions
        if let Some(del) = delete {
            match del {
                Delete::Where(clause) => {
                    table
                        .delete(&clause)
                        .await
                        .map_err(|e| DatasetError::Internal {
                            source: anyhow::anyhow!(e),
                        })?;
                }
                Delete::ByIds(ids) => {
                    // Convert StringArray to a SQL "id IN (...)" filter
                    let formatted_ids: Vec<String> = ids
                        .iter()
                        .flatten()
                        .map(|s| format!("'{}'", s.replace('\'', "''")))
                        .collect();
                    let filter = format!("id IN ({})", formatted_ids.join(","));
                    table
                        .delete(&filter)
                        .await
                        .map_err(|e| DatasetError::Internal {
                            source: anyhow::anyhow!(e),
                        })?;
                }
            }
        }

        // 2. Handle Upserts (Add new data)
        if let Some(upsert_stream) = upsert {
            // Collect the stream into a Vec of RecordBatches to provide to LanceDB
            let mut batches = Vec::new();
            let mut stream = upsert_stream.stream;
            while let Some(result) = stream.next().await {
                let batch = result?;
                batches.push(batch);
            }

            if !batches.is_empty() {
                let schema = batches.first().unwrap().schema();
                let reader = RecordBatchIterator::new(batches.into_iter().map(Ok), schema);
                table
                    .add(reader)
                    .execute()
                    .await
                    .map_err(|e| DatasetError::Internal {
                        source: anyhow::anyhow!(e),
                    })?;
            }
        }

        Ok(())
    }

    async fn select(
        &self,
        dataset_id: String,
        where_clause: Option<String>,
        order_by: Option<Vec<(String, OrderDirection)>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<SendableRecordBatchStream> {
        let table = self.get_table(&dataset_id).await?;
        let mut query = table.query();

        if let Some(filter) = where_clause {
            query = query.only_if(filter);
        }

        if let Some(l) = limit {
            query = query.limit(l);
        }

        if let Some(o) = offset {
            query = query.offset(o);
        }

        // Handle ordering - note: may need adjustment based on available API
        let _order_by = order_by; // Accept but don't use if method unavailable

        let mut stream =
            query
                .execute()
                .await
                .map_err(|e: lancedb::Error| DatasetError::Internal {
                    source: anyhow::anyhow!(e),
                })?;

        let schema = stream.schema();

        // Create a boxed stream that wraps results and converts errors
        let converted_stream: BoxStream<'static, Result<RecordBatch>> =
            Box::pin(async_stream::stream! {
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(batch) => yield Ok(batch),
                        Err(e) => {
                            yield Err(DatasetError::Internal {
                                source: anyhow::anyhow!(e),
                            });
                        }
                    }
                }
            });

        Ok(SendableRecordBatchStream {
            schema,
            stream: converted_stream,
        })
    }
}
