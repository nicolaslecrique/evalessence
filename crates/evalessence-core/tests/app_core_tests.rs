// remove lints that do not make sense in tests
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::too_many_lines,
    clippy::case_sensitive_file_extension_comparisons
)]

use evalessence_api::app::{AppError, AppServices};
use evalessence_core::app_core::FileAppService;
use tempfile::tempdir;
use tokio::fs;

// use pretty_assertions for better test failure diffs
use pretty_assertions::assert_eq;

// Helper: read file bytes
async fn read_bytes(path: &std::path::Path) -> Vec<u8> {
    fs::read(path).await.unwrap()
}

#[tokio::test]
async fn create_writes_file_and_returns_app() {
    let td = tempdir().unwrap();
    let svc = FileAppService::new(td.path());

    let app = svc.create("My App".to_string()).await.unwrap();

    assert_eq!(app.name, "My App");
    assert!(app.filename.starts_with("app-"));
    assert!(app.filename.ends_with(".yaml"));

    let p = td.path().join(&app.filename);
    assert!(p.exists());

    let bytes = read_bytes(&p).await;
    let etag = blake3::hash(&bytes).to_string();
    assert_eq!(app.etag, etag);

    let s = String::from_utf8(bytes).unwrap();
    assert!(s.contains("name: My App"));
}

#[tokio::test]
async fn get_reads_existing_file_and_computes_etag() {
    let td = tempdir().unwrap();
    let svc = FileAppService::new(td.path());

    let filename = "app-test.yaml";
    let path = td.path().join(filename);

    let yaml = "id: test-id
name: Test App
envs: []
datasets: []
pipelines: []
";

    fs::write(&path, yaml).await.unwrap();

    let app = svc.get(filename.to_string()).await.unwrap();
    assert_eq!(app.name, "Test App");
    assert_eq!(app.filename, filename.to_string());

    let bytes = read_bytes(&path).await;
    let etag = blake3::hash(&bytes).to_string();
    assert_eq!(app.etag, etag);
}

#[tokio::test]
async fn list_filters_non_app_files_and_preserves_get_errors() {
    let td = tempdir().unwrap();
    let svc = FileAppService::new(td.path());

    // valid
    let good = td.path().join("app-good.yaml");
    let good_yaml = "id: gid\nname: Good\nenvs: []\ndatasets: []\npipelines: []\n";
    fs::write(&good, good_yaml).await.unwrap();

    // invalid
    let bad = td.path().join("app-bad.yaml");
    fs::write(&bad, "not: : valid: yaml").await.unwrap();

    // ignored
    let other = td.path().join("ignore.txt");
    fs::write(&other, "hello").await.unwrap();

    let res = svc.list().await.unwrap();
    // should only attempt the two app-*.yaml files
    assert_eq!(res.len(), 2);

    let mut oks = 0usize;
    let mut errs = 0usize;
    for r in res {
        match r {
            Ok(a) => {
                assert_eq!(a.name, "Good");
                oks += 1;
            }
            Err(e) => match e {
                AppError::ValidationError {
                    filename: _,
                    source: _,
                } => errs += 1,
                other => panic!("unexpected error: {other:?}"),
            },
        }
    }

    assert_eq!(oks, 1);
    assert_eq!(errs, 1);
}

#[tokio::test]
async fn update_succeeds_with_matching_etag_and_conflicts_on_stale_etag() {
    let td = tempdir().unwrap();
    let svc = FileAppService::new(td.path());

    let app = svc.create("Updatable".to_string()).await.unwrap();
    let original_etag = app.etag.clone();
    let filename = app.filename.clone();

    // update name with current etag -> should succeed
    let mut updated = app.clone();
    updated.name = "Updated Name".to_string();

    let res = svc.update(updated.clone()).await.unwrap();
    assert_eq!(res.name, "Updated Name");
    assert_ne!(res.etag, original_etag);

    // attempt to update using old etag (simulate stale client)
    let mut stale = updated;
    stale.name = "Another Name".to_string();
    // set stale etag
    stale.etag = original_etag;

    let err = svc.update(stale).await.unwrap_err();
    match err {
        AppError::Conflict { filename: f } => assert_eq!(f, filename),
        other => panic!("expected conflict, got {other:?}"),
    }
}

#[tokio::test]
async fn delete_removes_file_and_get_fails_afterwards() {
    let td = tempdir().unwrap();
    let svc = FileAppService::new(td.path());

    let app = svc.create("ToDelete".to_string()).await.unwrap();
    let filename = app.filename.clone();
    let path = td.path().join(&filename);
    assert!(path.exists());

    svc.delete(filename.clone()).await.unwrap();
    assert!(!path.exists());

    let err = svc.get(filename).await.unwrap_err();
    match err {
        AppError::FileIoError {
            filename: _,
            source: _,
        } => {}
        other => panic!("expected file io error, got {other:?}"),
    }
}

#[tokio::test]
async fn get_non_existent_file_returns_fileioerror() {
    let td = tempdir().unwrap();
    let svc = FileAppService::new(td.path());

    let err = svc
        .get("does-not-exist.yaml".to_string())
        .await
        .unwrap_err();
    match err {
        AppError::FileIoError {
            filename: _,
            source: _,
        } => {}
        other => panic!("expected file io error, got {other:?}"),
    }
}
