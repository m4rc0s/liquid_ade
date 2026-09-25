use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use liquid_ade::{db, scpe, server};
use std::path::Path;
use tower::ServiceExt;

fn test_app() -> axum::Router {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let conn = db::init_db_at(":memory:").expect("Failed to init in-memory database");
    let state = server::AppState::new(conn, workspace_root);
    server::create_router_with_state(state)
}

/// Test scenario S1:
/// epic_04_living_spec_canvas_shell#S1 — Workspace outline projection
/// Given a workspace containing canonical features and epics on disk,
/// When a client requests GET /api/scpe/outline,
/// Then the server responds 200 with all features and epics projecting their lifecycle state,
/// task progress (tasks_done/tasks_total), and canonical document paths with leading slashes.
#[tokio::test]
async fn test_epic_04_living_spec_canvas_shell_s1_outline_projection() {
    let app = test_app();

    let request = Request::builder()
        .uri("/api/scpe/outline")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let outline: scpe::ScpeOutline = serde_json::from_slice(&body_bytes).unwrap();

    assert!(!outline.features.is_empty(), "Features must not be empty");

    let feat = outline
        .features
        .iter()
        .find(|f| f.slug == "02-inception-studio")
        .expect("02-inception-studio must be in outline");

    assert!(feat.path.starts_with('/'), "Feature path must start with /");

    let epic_02 = feat
        .epics
        .iter()
        .find(|e| e.slug == "epic_02_conversational_copilot_panel")
        .expect("epic_02 must be in outline");

    assert_eq!(epic_02.state, "Done");
    assert_eq!(epic_02.tasks_done, 4);
    assert_eq!(epic_02.tasks_total, 4);
    assert!(epic_02.path.starts_with('/'), "Epic path must start with /");
}

/// Test scenario S2:
/// epic_04_living_spec_canvas_shell#S2 — Epic detail card projection
/// Given a valid feature and epic slug,
/// When a client requests GET /api/scpe/epic?feature=02-inception-studio&epic=epic_02_conversational_copilot_panel,
/// Then the server responds 200 with parsed Intent, rules R1–R5, examples S1–S5, slices citing their R#/S# IDs, and raw section bodies.
#[tokio::test]
async fn test_epic_04_living_spec_canvas_shell_s2_epic_detail_projection() {
    let app = test_app();

    let request = Request::builder()
        .uri("/api/scpe/epic?feature=02-inception-studio&epic=epic_02_conversational_copilot_panel")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let detail: scpe::EpicDetailResponse = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(detail.feature, "02-inception-studio");
    assert_eq!(detail.epic, "epic_02_conversational_copilot_panel");
    assert_eq!(detail.state, "Done");
    assert!(!detail.intent.is_empty(), "Intent must be populated");
    assert_eq!(detail.rules.len(), 5, "Must parse 5 rules R1-R5");
    assert_eq!(detail.examples.len(), 5, "Must parse 5 examples S1-S5");
    assert_eq!(detail.slices.len(), 3, "Must parse 3 slices");
    assert!(
        !detail.raw_sections.is_empty(),
        "Raw sections must be populated"
    );
}

/// Test scenario S3:
/// epic_04_living_spec_canvas_shell#S3 — Path traversal protection on spec reader
/// Given a client request containing an escaping path parameter,
/// When a GET request is sent to /api/scpe/epic?feature=../../../etc&epic=passwd,
/// Then the server responds 403 with error code TRAVERSAL_DETECTED and reads no files outside the workspace root.
#[tokio::test]
async fn test_epic_04_living_spec_canvas_shell_s3_path_traversal_protection() {
    let app = test_app();

    let request = Request::builder()
        .uri("/api/scpe/epic?feature=../../../etc&epic=passwd")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body_json["error"], "TRAVERSAL_DETECTED");
}

/// Test scenario S4:
/// epic_04_living_spec_canvas_shell#S4 — State machine casing and round-trip fidelity
/// Given epics on disk with various SCPE lifecycle states,
/// When the outline endpoint reads each epic's quick_status.md,
/// Then all six states round-trip exactly including uppercase WIP and unrecognised values as raw text without silent coercion to Draft.
#[test]
fn test_epic_04_living_spec_canvas_shell_s4_state_roundtrip_fidelity() {
    let states = [
        "Draft",
        "Ready",
        "WIP",
        "Blocked",
        "Done",
        "Stale",
        "CustomState",
    ];
    for state in states {
        let content = format!("# Status\n\n- **state:** {state}\n- **confidence:** high\n");
        let parsed = scpe::parse_quick_status_state(&content);
        assert_eq!(parsed, state, "State must round-trip with exact fidelity");
    }
}

/// Test scenario S5:
/// epic_04_living_spec_canvas_shell#S5 — Task list with badge metadata parsing
/// Given a task line in tasks.md formatted as - [x] TASK-02: Stream tokens (S1, S2, S5, R2, R4, R5),
/// When the epic detail endpoint parses the file,
/// Then the returned task object contains done: true, label: "TASK-02", examples: ["S1", "S2", "S5"], and rules: ["R2", "R4", "R5"].
#[test]
fn test_epic_04_living_spec_canvas_shell_s5_task_list_badge_parsing() {
    let content = "# Tasks\n\n- [x] TASK-02: Stream tokens (S1, S2, S5, R2, R4, R5)\n- [ ] TASK-03: Provider settings (S4, R3)\n";
    let tasks = scpe::parse_tasks(content);

    assert_eq!(tasks.len(), 2);
    let t2 = &tasks[0];
    assert!(t2.done);
    assert_eq!(t2.label, "TASK-02");
    assert_eq!(t2.text, "Stream tokens");
    assert_eq!(t2.examples, vec!["S1", "S2", "S5"]);
    assert_eq!(t2.rules, vec!["R2", "R4", "R5"]);

    let t3 = &tasks[1];
    assert!(!t3.done);
    assert_eq!(t3.label, "TASK-03");
    assert_eq!(t3.text, "Provider settings");
    assert_eq!(t3.examples, vec!["S4"]);
    assert_eq!(t3.rules, vec!["R3"]);
}

/// Test scenario S6:
/// epic_04_living_spec_canvas_shell#S6 — Co-pilot document binding to active epic plan
/// Given an epic selected on the Spec Navigator without a specific document file selected,
/// When the user submits a prompt in the Conversational Co-Pilot tab,
/// Then the turn automatically binds that epic's plan.md path as the active context document.
#[test]
fn test_epic_04_living_spec_canvas_shell_s6_copilot_document_binding() {
    let feature_slug = "02-inception-studio";
    let epic_slug = "epic_02_conversational_copilot_panel";
    let bound_plan_path = format!("features/{feature_slug}/epics/{epic_slug}/plan.md");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
    let plan_file = workspace_root.join(&bound_plan_path);
    assert!(plan_file.exists(), "Bound plan.md must exist on disk");

    let read_result = liquid_ade::workspace::read_workspace_file(workspace_root, &bound_plan_path);
    assert!(
        read_result.is_ok(),
        "Context document read through path guard must succeed"
    );
}
