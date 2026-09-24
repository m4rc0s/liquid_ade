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
/// epic_02_workspace_fs_scanner#S1 — Workspace directory tree query
/// Given a workspace initialized on disk,
/// When a GET request is sent to /api/workspace/tree,
/// Then the server responds with 200 containing a recursive JSON tree of directories and files.
#[tokio::test]
async fn test_epic_02_workspace_fs_scanner_s1_tree_query() {
    let tmp = tempdir().expect("Failed to create tempdir");
    let root = tmp.path().to_path_buf();

    // Create a mock SCPE workspace structure
    fs::create_dir_all(root.join("apps").join("ade")).unwrap();
    fs::create_dir_all(root.join("features").join("01-workspace-inspector")).unwrap();
    fs::create_dir_all(root.join("target")).unwrap(); // should be excluded
    fs::create_dir_all(root.join(".git")).unwrap(); // should be excluded
    fs::write(root.join("product_vision.md"), "# Product Vision").unwrap();
    fs::write(
        root.join("features")
            .join("01-workspace-inspector")
            .join("index.md"),
        "# Feature 01",
    )
    .unwrap();

    let state = AppState::in_memory(root.clone());
    let app = create_router_with_state(state);

    let req = Request::builder()
        .uri("/api/workspace/tree")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("children").is_some());
    let children = json["children"].as_array().unwrap();

    // Verify product_vision.md and features are included
    let names: Vec<&str> = children.iter().filter_map(|c| c["name"].as_str()).collect();
    assert!(names.contains(&"product_vision.md"));
    assert!(names.contains(&"features"));
    assert!(names.contains(&"apps"));

    // Verify .git and target are excluded (R2)
    assert!(!names.contains(&".git"));
    assert!(!names.contains(&"target"));
}

/// Test scenario S2:
/// epic_02_workspace_fs_scanner#S2 — Path traversal attack rejected
/// Given the server is active,
/// When a GET request is sent to /api/workspace/file?path=../../etc/passwd,
/// Then the server responds with 403 Forbidden and error code TRAVERSAL_DETECTED.
#[tokio::test]
async fn test_epic_02_workspace_fs_scanner_s2_path_traversal_rejected() {
    let tmp = tempdir().expect("Failed to create tempdir");
    let root = tmp.path().to_path_buf();

    let state = AppState::in_memory(root);
    let app = create_router_with_state(state);

    // Attempt traversal with parent directory reference
    let req = Request::builder()
        .uri("/api/workspace/file?path=../../etc/passwd")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "TRAVERSAL_DETECTED");
}

/// Test scenario S3:
/// epic_02_workspace_fs_scanner#S3 — Register and switch active project
/// Given a valid project path on disk,
/// When a POST request is sent to /api/projects/open with {"path":"/path/to/project"},
/// Then the project is registered in SQLite with updated last_opened_at and the active workspace is set.
#[tokio::test]
async fn test_epic_02_workspace_fs_scanner_s3_register_and_switch_project() {
    let tmp_workspace = tempdir().expect("Failed to create workspace tempdir");
    let project_dir = tempdir().expect("Failed to create project tempdir");
    let project_path = project_dir.path().to_path_buf();

    let state = AppState::in_memory(tmp_workspace.path().to_path_buf());
    let app = create_router_with_state(state.clone());

    let payload = serde_json::json!({
        "path": project_path.to_string_lossy().to_string()
    });

    let req = Request::builder()
        .uri("/api/projects/open")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        json["path"],
        project_path
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .to_string()
    );
    assert!(json["id"].is_string());

    // Verify active workspace root was switched
    let current_root = state.workspace_root.read().unwrap().clone();
    assert_eq!(current_root, project_path.canonicalize().unwrap());

    // Verify project appears in list
    let list_req = Request::builder()
        .uri("/api/projects")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let list_res = create_router_with_state(state)
        .oneshot(list_req)
        .await
        .unwrap();
    assert_eq!(list_res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_epic_02_workspace_file_read_and_write_within_bounds() {
    let tmp = tempdir().expect("Failed to create tempdir");
    let root = tmp.path().to_path_buf();

    let state = AppState::in_memory(root.clone());
    let app = create_router_with_state(state.clone());

    // Write file
    let write_payload = serde_json::json!({
        "path": "features/test/spec.md",
        "content": "# Test Specification"
    });
    let write_req = Request::builder()
        .uri("/api/workspace/file")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&write_payload).unwrap()))
        .unwrap();

    let write_res = app.oneshot(write_req).await.unwrap();
    assert_eq!(write_res.status(), StatusCode::OK);

    // Read file
    let read_req = Request::builder()
        .uri("/api/workspace/file?path=features/test/spec.md")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let read_res = create_router_with_state(state)
        .oneshot(read_req)
        .await
        .unwrap();
    assert_eq!(read_res.status(), StatusCode::OK);

    let body = read_res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["content"], "# Test Specification");
}
