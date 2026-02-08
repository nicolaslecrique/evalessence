use atomicwrites::{AllowOverwrite, AtomicFile};
use std::{io, io::Write, path::PathBuf};
use tokio::task;

pub async fn atomic_write_async(path: PathBuf, data: Vec<u8>) -> io::Result<()> {
    task::spawn_blocking(move || {
        let af = AtomicFile::new(path, AllowOverwrite);

        af.write(|f| {
            f.write_all(&data)?;
            Ok(())
        })
        .map_err(|e| match e {
            atomicwrites::Error::Internal(err) | atomicwrites::Error::User(err) => err,
        })
    })
    .await
    .map_err(io::Error::other)?
}
