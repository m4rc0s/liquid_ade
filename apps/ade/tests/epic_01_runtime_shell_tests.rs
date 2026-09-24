use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use liquid_ade::{db, server};
use tower::ServiceExt;

/// Test scenario S1:
/// epic_01_runtime_shell#S1 — Health check returns status ok
/// Given the Axum server is running on port 3000,
/// When a GET request is sent to /api/health,
/// Then the response status is 200 and the body is {"status":"ok","app":"liquid-ade"}.
#[tokio::test]
async fn test_epic_01_runtime_shell_s1_health_check() {
    let app = server::create_router();

    let request = Request::builder()
        .uri("/api/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(
        body_json,
        serde_json::json!({
            "status": "ok",
            "app": "liquid-ade"
        })
    );
}

/// Test scenario S2:
/// epic_01_runtime_shell#S2 — Single page app fallback
/// Given the Axum server is running,
/// When a GET request is sent to /unknown-path,
/// Then the server responds with 200 serving index.html as the SPA fallback.
#[tokio::test]
async fn test_epic_01_runtime_shell_s2_spa_fallback() {
    let app = server::create_router();

    let request = Request::builder()
        .uri("/unknown-path")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("content-type")
        .expect("Missing content-type header")
        .to_str()
        .unwrap();
    assert!(
        content_type.contains("text/html"),
        "Expected text/html content type, got: {}",
        content_type
    );

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8_lossy(&body_bytes);
    assert!(
        body_str.contains("<html")
            || body_str.contains("<!DOCTYPE")
            || body_str.contains("id=\"root\""),
        "Response should contain index.html content"
    );
}

/// Test scenario S3:
/// epic_01_runtime_shell#S3 — Embedded SQLite engine initializes on boot
/// Given the Axum server bootstraps,
/// When the database engine initializes,
/// Then the SQLite file at ~/.liquid/liquid.db is opened and tables projects, checkpoints, and settings exist.
#[test]
fn test_epic_01_runtime_shell_s3_database_initialization() {
    let conn = db::init_db().expect("init_db should succeed");

    let db_path = db::default_db_path().expect("default_db_path should succeed");
    assert!(
        db_path.exists(),
        "Database file at {:?} must exist",
        db_path
    );

    let tables_exist = db::verify_tables_exist(&conn).expect("verify_tables_exist should succeed");
    assert!(
        tables_exist,
        "All tables (projects, checkpoints, settings) must exist"
    );

    // Verify individual table presence
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .unwrap();
    let table_names: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert!(table_names.contains(&"projects".to_string()));
    assert!(table_names.contains(&"checkpoints".to_string()));
    assert!(table_names.contains(&"settings".to_string()));
}
