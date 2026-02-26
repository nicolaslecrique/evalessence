use arrow::array::Array;
use arrow::record_batch::{RecordBatch, RecordBatchIterator};
use duckdb::Connection;
use duckdb::vtab::arrow::{ArrowVTab, arrow_recordbatch_to_query_params};
use evalessence_api::dataset::{
    DatasetError, DatasetService, Delete, OrderDirection, Result, SendableRecordBatchReader,
};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct DuckDbDatasetService {
    conn: Arc<Mutex<Connection>>,
    base_path: PathBuf,
}

impl DuckDbDatasetService {
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open_in_memory().map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to open connection: {}", e),
        })?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            base_path: base_path.as_ref().to_path_buf(),
        })
    }

    fn dataset_path(&self, dataset_id: &str) -> PathBuf {
        self.base_path.join(format!("{}.parquet", dataset_id))
    }

    fn ensure_table_loaded(&self, dataset_id: &str) -> Result<()> {
        let path = self.dataset_path(dataset_id);
        let conn = self.conn.lock().map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to lock connection: {}", e),
        })?;

        if path.exists() {
            let sql = format!(
                "CREATE OR REPLACE TABLE {} AS SELECT * FROM read_parquet('{}')",
                dataset_id,
                path.display()
            );
            conn.execute(&sql, []).map_err(|e| DatasetError::Internal {
                source: anyhow::anyhow!("Failed to load table: {}", e),
            })?;
        }

        Ok(())
    }

    fn save_table(&self, dataset_id: &str) -> Result<()> {
        let path = self.dataset_path(dataset_id);
        let conn = self.conn.lock().map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to lock connection: {}", e),
        })?;

        let sql = format!(
            "COPY {} TO '{}' (FORMAT PARQUET)",
            dataset_id,
            path.display()
        );
        conn.execute(&sql, []).map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to save table: {}", e),
        })?;

        Ok(())
    }
}

impl DatasetService for DuckDbDatasetService {
    fn update(
        &self,
        dataset_id: String,
        upsert: Option<SendableRecordBatchReader>,
        delete: Option<Delete>,
    ) -> Result<()> {
        self.ensure_table_loaded(&dataset_id)?;

        let conn = self.conn.lock().map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to lock connection: {}", e),
        })?;

        // Handle upsert
        if let Some(mut reader) = upsert {
            let mut batches: Vec<RecordBatch> = Vec::new();
            while let Some(batch) = reader.next() {
                batches.push(batch?);
            }

            if !batches.is_empty() {
                // Register ArrowVTab once so arrow_scan is available
                conn.register_table_function::<ArrowVTab>("arrow_scan")
                    .map_err(|e| DatasetError::Internal {
                        source: anyhow::anyhow!("Failed to register ArrowVTab: {}", e),
                    })?;

                for batch in batches {
                    let params = arrow_recordbatch_to_query_params(batch);
                    conn.execute(
                        &format!("INSERT OR REPLACE INTO {dataset_id} SELECT * FROM arrow(?, ?)"),
                        params,
                    )
                    .map_err(|e| DatasetError::Internal {
                        source: anyhow::anyhow!("Failed to upsert data: {}", e),
                    })?;
                }
            }
        }

        // Handle delete
        if let Some(delete_spec) = delete {
            match delete_spec {
                Delete::ByIds(ids) => {
                    let id_values: Vec<String> = (0..Array::len(&ids))
                        .map(|i| ids.value(i).to_string())
                        .collect();

                    if !id_values.is_empty() {
                        let placeholders = id_values
                            .iter()
                            .map(|v| format!("'{}'", v.replace("'", "''")))
                            .collect::<Vec<_>>()
                            .join(", ");

                        let sql =
                            format!("DELETE FROM {} WHERE id IN ({})", dataset_id, placeholders);
                        conn.execute(&sql, []).map_err(|e| DatasetError::Internal {
                            source: anyhow::anyhow!("Failed to delete by IDs: {}", e),
                        })?;
                    }
                }
                Delete::Where(where_clause) => {
                    let sql = format!("DELETE FROM {} WHERE {}", dataset_id, where_clause);
                    conn.execute(&sql, []).map_err(|e| DatasetError::Internal {
                        source: anyhow::anyhow!("Failed to delete with WHERE: {}", e),
                    })?;
                }
            }
        }

        drop(conn);
        self.save_table(&dataset_id)?;

        Ok(())
    }

    fn select(
        &self,
        dataset_id: String,
        where_clause: Option<String>,
        order_by: Option<Vec<(String, OrderDirection)>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<SendableRecordBatchReader> {
        self.ensure_table_loaded(&dataset_id)?;

        let mut sql = format!("SELECT * FROM {}", dataset_id);

        if let Some(where_str) = where_clause {
            sql.push_str(&format!(" WHERE {}", where_str));
        }

        if let Some(order_vec) = order_by {
            if !order_vec.is_empty() {
                let order_clause = order_vec
                    .iter()
                    .map(|(col, dir)| {
                        let dir_str = match dir {
                            OrderDirection::Asc => "ASC",
                            OrderDirection::Desc => "DESC",
                        };
                        format!("{} {}", col, dir_str)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                sql.push_str(&format!(" ORDER BY {}", order_clause));
            }
        }

        if let Some(lim) = limit {
            sql.push_str(&format!(" LIMIT {}", lim));
        }

        if let Some(off) = offset {
            sql.push_str(&format!(" OFFSET {}", off));
        }

        let conn = self.conn.lock().map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to lock connection: {}", e),
        })?;

        let mut stmt = conn.prepare(&sql).map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to prepare statement: {}", e),
        })?;
        let arrow = stmt.query_arrow([]).map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to execute query: {}", e),
        })?;

        // Collect all batches eagerly so we can release the connection lock
        let schema = arrow.get_schema();
        let batches: Vec<RecordBatch> = arrow.collect();

        Ok(Box::new(RecordBatchIterator::new(
            batches.into_iter().map(Ok),
            schema,
        )))
    }
}
