use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use liquid_ade::server::{create_router_with_state, AppState};
use std::fs;
use tempfile::tempdir;
use tower::ServiceExt;

/// Test scenario S1:
/// epic_01_project_scaffold_generator#S1 — Scaffold new canonical SCPE project
/// Given an empty target directory on disk,
/// When a POST request is sent to /api/workspace/new with {"name":"new-product","path":"/tmp/new-product"},
/// Then the response status is 201 Created and all canonical SCPE files exist in the destination.
#[tokio::test]
async fn test_epic_01_project_scaffold_generator_s1_scaffold_new_project() {
    let tmp_root = tempdir().expect("Failed to create root tempdir");
    let state = AppState::in_memory(tmp_root.path().to_path_buf());
    let app = create_router_with_state(state.clone());

    let target_dir = tempdir().expect("Failed to create target tempdir");
    let target_path = target_dir.path().join("new-product");

    let payload = serde_json::json!({
        "name": "new-product",
        "path": target_path.to_string_lossy().to_string()
    });

    let req = Request::builder()
        .uri("/api/workspace/new")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["name"], "new-product");
    assert!(json["id"].is_string());

    // Verify all canonical SCPE files exist in destination (R2)
    let canonical_root_files = [
        "index.md",
        "product_vision.md",
        "roadmap.md",
        "glossary.md",
        "architecture.md",
        "technical_deal.md",
        "team_playbook.md",
        "quick_status.md",
        "CLAUDE.md",
    ];

    for file_name in canonical_root_files {
        let p = target_path.join(file_name);
        assert!(
            p.is_file(),
            "Canonical file {} must exist in scaffolded project",
            file_name
        );
    }

    // Verify canonical directories exist
    assert!(target_path.join("apps").is_dir());
    assert!(target_path.join("features").is_dir());
    assert!(target_path.join("assets").is_dir());

    // Verify workspace root was updated to the new project
    let current_root = state.workspace_root.read().unwrap().clone();
    assert_eq!(current_root, target_path.canonicalize().unwrap());
}

/// Test scenario S2:
/// epic_01_project_scaffold_generator#S2 — Reject invalid project slug
/// Given an active server,
/// When a POST request is sent to /api/workspace/new with {"name":"Invalid Name!","path":"/tmp/invalid"},
/// Then the response status is 400 Bad Request with validation error message.
#[tokio::test]
async fn test_epic_01_project_scaffold_generator_s2_reject_invalid_slug() {
    let tmp_root = tempdir().expect("Failed to create root tempdir");
    let state = AppState::in_memory(tmp_root.path().to_path_buf());
    let app = create_router_with_state(state);

    let target_dir = tempdir().expect("Failed to create target tempdir");
    let target_path = target_dir.path().join("invalid-product");

    let payload = serde_json::json!({
        "name": "Invalid Name!",
        "path": target_path.to_string_lossy().to_string()
    });

    let req = Request::builder()
        .uri("/api/workspace/new")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["error"], "INVALID_PROJECT_NAME");
}

#[tokio::test]
async fn test_epic_01_project_scaffold_generator_reject_non_empty_dir() {
    let tmp_root = tempdir().expect("Failed to create root tempdir");
    let state = AppState::in_memory(tmp_root.path().to_path_buf());
    let app = create_router_with_state(state);

    let target_dir = tempdir().expect("Failed to create target tempdir");
    let target_path = target_dir.path().join("existing-product");
    fs::create_dir_all(&target_path).unwrap();
    fs::write(target_path.join("existing_file.txt"), "hello").unwrap();

    let payload = serde_json::json!({
        "name": "existing-product",
        "path": target_path.to_string_lossy().to_string()
    });

    let req = Request::builder()
        .uri("/api/workspace/new")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["error"], "DIRECTORY_NOT_EMPTY");
}
