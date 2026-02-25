use arrow::record_batch::RecordBatchReader;
use duckdb::{DuckdbConnectionManager, params};
use evalessence_api::dataset::{
    DatasetError, DatasetService, Delete, OrderDirection, Result, SendableRecordBatchReader,
};
use r2d2::Pool;
use std::path::{Path, PathBuf};

pub struct DuckDbDatasetService {
    pool: Pool<DuckdbConnectionManager>,
    base_path: PathBuf,
}

impl DuckDbDatasetService {
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let manager = DuckdbConnectionManager::file(base_path.as_ref()).map_err(|e| {
            DatasetError::Internal {
                source: anyhow::anyhow!("Failed to create connection manager: {}", e),
            }
        })?;
        let pool = Pool::new(manager).map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to create connection pool: {}", e),
        })?;

        Ok(Self {
            pool,
            base_path: base_path.as_ref().to_path_buf(),
        })
    }

    fn dataset_path(&self, dataset_id: &str) -> PathBuf {
        self.base_path.join(format!("{}.parquet", dataset_id))
    }

    fn ensure_table_loaded(&self, dataset_id: &str) -> Result<()> {
        let path = self.dataset_path(dataset_id);
        let conn = self.pool.get().map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to get connection from pool: {}", e),
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
        let conn = self.pool.get().map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to get connection from pool: {}", e),
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

        let conn = self.pool.get().map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to get connection from pool: {}", e),
        })?;

        // Handle upsert
        if let Some(mut reader) = upsert {
            // Create temporary table from record batches
            let schema = reader.schema();
            let mut batches = Vec::new();
            while let Some(batch) = reader.next() {
                batches.push(batch?);
            }

            if !batches.is_empty() {
                // Convert batches to Arrow IPC and register
                let temp_table = format!("{}_temp", dataset_id);

                // Register Arrow batches as a view in DuckDB
                conn.execute(&format!("DROP TABLE IF EXISTS {}", temp_table), [])
                    .map_err(|e| DatasetError::Internal {
                        source: anyhow::anyhow!("Failed to drop temp table: {}", e),
                    })?;

                // Insert/merge logic - assuming there's an 'id' column for upsert
                conn.execute(
                    &format!(
                        "INSERT OR REPLACE INTO {} SELECT * FROM arrow_scan(?)",
                        dataset_id
                    ),
                    params![batches],
                )
                .map_err(|e| DatasetError::Internal {
                    source: anyhow::anyhow!("Failed to upsert data: {}", e),
                })?;
            }
        }

        // Handle delete
        if let Some(delete_spec) = delete {
            match delete_spec {
                Delete::ByIds(ids) => {
                    let id_values: Vec<String> = (0..ids.len())
                        .filter_map(|i| ids.value(i).to_string().into())
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

        let conn = self.pool.get().map_err(|e| DatasetError::Internal {
            source: anyhow::anyhow!("Failed to get connection from pool: {}", e),
        })?;

        // Execute query and get Arrow record batches
        let arrow =
            duckdb::arrow::query_arrow(&*conn, &sql, []).map_err(|e| DatasetError::Internal {
                source: anyhow::anyhow!("Failed to execute query: {}", e),
            })?;

        Ok(Box::new(arrow))
    }
}
