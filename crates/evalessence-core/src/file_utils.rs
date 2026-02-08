use atomicwrites::{AllowOverwrite, AtomicFile};
use std::{io, io::Write, path::PathBuf};
use tokio::task;

// Utility for atomic file writes in an async context
// we need this because several calls might try to write the same file at the same time
pub async fn atomic_write_async(
    path: impl Into<PathBuf>,
    data: impl Into<Vec<u8>>, // Or use bytes::Bytes for zero-copy clones
) -> io::Result<()> {
    let path = path.into();
    let data = data.into();

    task::spawn_blocking(move || {
        let af = AtomicFile::new(path, AllowOverwrite);

        af.write(|f| {
            f.write_all(&data)?;
            // Optional: f.sync_all()?; // Ensures data hits the disk
            Ok(())
        })
        .map_err(|e| match e {
            // Flatten the nested error types
            atomicwrites::Error::Internal(err) | atomicwrites::Error::User(err) => err,
        })
    })
    .await
    // Convert JoinError to io::Error
    .map_err(io::Error::other)?
}
