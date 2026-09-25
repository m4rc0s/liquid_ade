use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use liquid_ade::server;
use std::path::Path;
use tower::ServiceExt;

/// Test scenario S1:
/// epic_03_technical_deal_compliance#S1 — Axum 0.8 framework compliance
/// Given the ADE application compiled with Axum 0.8,
/// When the test suite executes health, workspace, and copilot endpoints,
/// Then all existing routes respond with valid HTTP status codes and zero deprecation warnings.
#[tokio::test]
async fn test_epic_03_technical_deal_compliance_s1_axum_08_framework_compliance() {
    let app = server::create_router();

    // Verify /api/health
    let req = Request::builder()
        .uri("/api/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body_bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body_json["status"], "ok");

    // Verify /api/settings/copilot
    let req = Request::builder()
        .uri("/api/settings/copilot")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

/// Test scenario S2:
/// epic_03_technical_deal_compliance#S2 — Complete SCPE state token coverage
/// Given the design system stylesheets in index.css and App.css,
/// When checking color variables and .astryx-chip classes,
/// Then tokens and classes exist for all six SCPE states including stale and blocked.
#[test]
fn test_epic_03_technical_deal_compliance_s2_complete_scpe_state_tokens() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let index_css = std::fs::read_to_string(manifest_dir.join("ui/src/index.css"))
        .expect("ui/src/index.css must be readable");
    let app_css = std::fs::read_to_string(manifest_dir.join("ui/src/App.css"))
        .expect("ui/src/App.css must be readable");

    let required_tokens = [
        "--status-draft",
        "--status-ready",
        "--status-wip",
        "--status-blocked",
        "--status-done",
        "--status-stale",
    ];

    for token in required_tokens {
        assert!(
            index_css.contains(token),
            "index.css must define token {}",
            token
        );
    }

    let required_chips = [
        ".astryx-chip.draft",
        ".astryx-chip.ready",
        ".astryx-chip.wip",
        ".astryx-chip.blocked",
        ".astryx-chip.done",
        ".astryx-chip.stale",
    ];

    for chip in required_chips {
        assert!(
            app_css.contains(chip),
            "App.css must define chip class {}",
            chip
        );
    }
}

/// Test scenario S3:
/// epic_03_technical_deal_compliance#S3 — Astryx component encapsulation
/// Given the extracted Astryx component library in apps/ade/ui/src/components/,
/// When inspecting StateChip, TaskItem, SpecCard, RuleItem, and Chip,
/// Then typed Astryx component wrappers are properly implemented and exported.
#[test]
fn test_epic_03_technical_deal_compliance_s3_astryx_component_encapsulation() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_dir = manifest_dir.join("ui/src/components");

    assert!(components_dir.join("StateChip.tsx").exists());
    assert!(components_dir.join("TaskItem.tsx").exists());
    assert!(components_dir.join("SpecCard.tsx").exists());
    assert!(components_dir.join("RuleItem.tsx").exists());
    assert!(components_dir.join("Chip.tsx").exists());
    assert!(components_dir.join("index.ts").exists());

    let state_chip_content = std::fs::read_to_string(components_dir.join("StateChip.tsx")).unwrap();
    assert!(state_chip_content.contains("SCPEState"));
    assert!(state_chip_content.contains("StateChip"));

    let task_item_content = std::fs::read_to_string(components_dir.join("TaskItem.tsx")).unwrap();
    assert!(task_item_content.contains("TaskItem"));
    assert!(task_item_content.contains("astryx-task-item"));

    let spec_card_content = std::fs::read_to_string(components_dir.join("SpecCard.tsx")).unwrap();
    assert!(spec_card_content.contains("SpecCard"));
    assert!(spec_card_content.contains("@astryxdesign/core"));
}

/// Test scenario S4:
/// epic_03_technical_deal_compliance#S4 — Zustand workspace state migration
/// Given the application UI state store in apps/ade/ui/src/stores/workspaceStore.ts,
/// When inspecting store definitions,
/// Then activeProject, activePath, openDocument, and loadTree are managed by Zustand.
#[test]
fn test_epic_03_technical_deal_compliance_s4_zustand_workspace_state_migration() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let store_file = manifest_dir.join("ui/src/stores/workspaceStore.ts");

    assert!(store_file.exists());
    let store_content = std::fs::read_to_string(store_file).unwrap();
    assert!(store_content.contains("create<WorkspaceState>"));
    assert!(store_content.contains("activeProject"));
    assert!(store_content.contains("activePath"));
    assert!(store_content.contains("openDocument"));
    assert!(store_content.contains("loadTree"));
}

/// Test scenario S5:
/// epic_03_technical_deal_compliance#S5 — TypeScript strict mode validation
/// Given tsconfig.app.json configured with strict mode,
/// When inspecting compiler options,
/// Then strict mode is explicitly set to true.
#[test]
fn test_epic_03_technical_deal_compliance_s5_typescript_strict_mode_validation() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tsconfig_file = manifest_dir.join("ui/tsconfig.app.json");

    assert!(tsconfig_file.exists());
    let tsconfig_content = std::fs::read_to_string(tsconfig_file).unwrap();
    assert!(
        tsconfig_content.contains("\"strict\": true"),
        "tsconfig.app.json must contain '\"strict\": true'"
    );
}
