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

#[tokio::test]
async fn create_get_consistent() {
    let td = tempdir().unwrap();
    let svc = FileAppService::new(td.path());

    let created_app = svc.create("My App".to_string()).await.unwrap();
    let get_app = svc.get(created_app.filename.clone()).await.unwrap();

    assert_eq!(created_app.etag, get_app.etag); // apps are the same
    assert_eq!(created_app.name, "My App");
    assert_eq!(created_app.filename, format!("app-{}.yaml", created_app.id)); // filename is derived from id
    assert!(created_app.id.to_string().starts_with("my-app-")); // id is slugified name + random suffix
    assert!(created_app.pipelines.is_empty()); // pipelines/envs/datasets are empty by default
    assert!(created_app.envs.is_empty());
    assert!(created_app.datasets.is_empty());
}

#[tokio::test]
async fn list_filters_non_app_files_and_preserves_get_errors() {
    let td = tempdir().unwrap();
    let svc = FileAppService::new(td.path());

    // write valid
    svc.create("valid".to_string()).await.unwrap();

    // invalid
    let bad = td.path().join("app-bad.yaml");
    fs::write(&bad, "not: : valid: yaml").await.unwrap();

    // ignored
    let other = td.path().join("ignore.txt");
    fs::write(&other, "hello").await.unwrap();

    let res = svc.list().await.unwrap();
    // should only attempt the two app-*.yaml files
    assert_eq!(res.len(), 2);

    let oks = res
        .iter()
        .filter_map(|r| r.as_ref().ok())
        .filter(|a| a.name == "valid")
        .count();
    let errs = res
        .iter()
        .filter_map(|r| r.as_ref().err())
        .filter(|e| matches!(e, AppError::ValidationError { .. }))
        .count();

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
