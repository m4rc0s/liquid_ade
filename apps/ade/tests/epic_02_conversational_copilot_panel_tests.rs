//! Acceptance tests for `02-inception-studio/epic_02_conversational_copilot_panel`.
//!
//! Every scenario of `plan.md` is verified against a stub provider served over plain HTTP, so the
//! suite never depends on network access nor on a real Google Gemini credential.

use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
    Router,
};
use http_body_util::BodyExt;
use liquid_ade::copilot::CopilotConfig;
use liquid_ade::server::{create_router_with_state, AppState};
use std::sync::{Arc, Mutex, OnceLock};
use tempfile::tempdir;
use tower::ServiceExt;

/// Serializes tests that mutate process-wide environment variables.
fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[derive(Clone)]
struct StubProvider {
    sse_body: Arc<String>,
    status: StatusCode,
    captured_request: Arc<Mutex<Option<serde_json::Value>>>,
}

async fn stub_handler(State(stub): State<StubProvider>, body: String) -> impl IntoResponse {
    *stub.captured_request.lock().unwrap() = serde_json::from_str(&body).ok();

    if stub.status.is_success() {
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/event-stream")
            .body(Body::from(stub.sse_body.as_str().to_owned()))
            .unwrap()
    } else {
        Response::builder()
            .status(stub.status)
            .body(Body::from("inference backend offline"))
            .unwrap()
    }
}

/// Boots a stub Gemini-compatible provider on an ephemeral port and returns its base URL.
async fn spawn_stub_provider(
    sse_body: &str,
    status: StatusCode,
) -> (String, Arc<Mutex<Option<serde_json::Value>>>) {
    let captured_request = Arc::new(Mutex::new(None));
    let stub = StubProvider {
        sse_body: Arc::new(sse_body.to_string()),
        status,
        captured_request: Arc::clone(&captured_request),
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = Router::new().fallback(stub_handler).with_state(stub);

    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    (format!("http://{addr}"), captured_request)
}

fn state_with_provider(root: std::path::PathBuf, base_url: &str) -> AppState {
    let state = AppState::in_memory(root).with_copilot_config(CopilotConfig {
        base_url: base_url.to_string(),
        default_model: "gemini-2.5-flash".to_string(),
    });
    {
        let conn = state.db.lock().unwrap();
        liquid_ade::db::set_setting(&conn, liquid_ade::copilot::SETTING_API_KEY, "test-key-123")
            .unwrap();
    }
    state
}

fn chat_request(payload: serde_json::Value) -> Request<Body> {
    Request::builder()
        .uri("/api/copilot/chat")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap()
}

async fn body_to_string(res: Response<Body>) -> String {
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

const GEMINI_SSE_BODY: &str = concat!(
    "data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"Our target audience\"}]}}]}\n\n",
    "data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\" is senior architects.\"}]}}]}\n\n",
);

/// Test scenario S1:
/// epic_02_conversational_copilot_panel#S1 — Streaming co-pilot message
/// Given a user opens the co-pilot panel with product_vision.md active on the canvas and a
/// provider credential configured,
/// When the user sends the prompt "Clarify our target audience",
/// Then the server responds 200 text/event-stream and the answer tokens arrive as incremental
/// token events terminated by a done event.
#[tokio::test]
async fn test_epic_02_conversational_copilot_panel_s1_streaming_message() {
    let tmp = tempdir().unwrap();
    let root = tmp.path().to_path_buf();
    std::fs::write(
        root.join("product_vision.md"),
        "# Product Vision\n\nLiquid ADE is an agentic development environment.",
    )
    .unwrap();

    let (base_url, _) = spawn_stub_provider(GEMINI_SSE_BODY, StatusCode::OK).await;
    let app = create_router_with_state(state_with_provider(root, &base_url));

    let res = app
        .oneshot(chat_request(serde_json::json!({
            "messages": [{ "role": "user", "text": "Clarify our target audience" }],
            "context_path": "product_vision.md"
        })))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let content_type = res
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string();
    assert!(
        content_type.starts_with("text/event-stream"),
        "expected an SSE response, got {content_type}"
    );

    let body = body_to_string(res).await;
    assert!(
        body.contains("event: token"),
        "missing token events: {body}"
    );
    assert!(body.contains("Our target audience"));
    assert!(body.contains("is senior architects."));
    assert!(
        body.contains("event: done"),
        "missing terminating done event"
    );

    // R2: the answer is delivered incrementally, one event per provider chunk.
    assert_eq!(body.matches("event: token").count(), 2);
}

/// Test scenario S2:
/// epic_02_conversational_copilot_panel#S2 — Resilient error handling on model failure
/// Given a configured provider whose inference backend is offline,
/// When the user submits a message,
/// Then the server responds 502 with error code UPSTREAM_UNAVAILABLE so the chat panel can show
/// an inline error banner with a retry action without clearing the input.
#[tokio::test]
async fn test_epic_02_conversational_copilot_panel_s2_upstream_failure() {
    let tmp = tempdir().unwrap();
    let (base_url, _) = spawn_stub_provider("", StatusCode::SERVICE_UNAVAILABLE).await;
    let app = create_router_with_state(state_with_provider(tmp.path().to_path_buf(), &base_url));

    let res = app
        .oneshot(chat_request(serde_json::json!({
            "messages": [{ "role": "user", "text": "Clarify our target audience" }]
        })))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_GATEWAY);

    let body = body_to_string(res).await;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["error"], "UPSTREAM_UNAVAILABLE");
    assert!(json["message"].as_str().unwrap().contains("503"));
}

/// Test scenario S3:
/// epic_02_conversational_copilot_panel#S3 — Document context binding
/// Given a workspace containing product_vision.md,
/// When the user selects that file on the workspace tree and submits a co-pilot prompt,
/// Then the canvas renders the file content and the prompt sent upstream carries the bound
/// document path and its content as context.
#[tokio::test]
async fn test_epic_02_conversational_copilot_panel_s3_document_context_binding() {
    let tmp = tempdir().unwrap();
    let root = tmp.path().to_path_buf();
    std::fs::write(
        root.join("product_vision.md"),
        "# Product Vision\n\nTarget audience: senior architects.",
    )
    .unwrap();

    let (base_url, captured) = spawn_stub_provider(GEMINI_SSE_BODY, StatusCode::OK).await;
    let state = state_with_provider(root, &base_url);

    // The canvas sources its content from the workspace file endpoint.
    let canvas = create_router_with_state(state.clone())
        .oneshot(
            Request::builder()
                .uri("/api/workspace/file?path=product_vision.md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(canvas.status(), StatusCode::OK);
    let canvas_body = body_to_string(canvas).await;
    assert!(canvas_body.contains("Target audience: senior architects."));

    // The co-pilot turn binds that same document (R1) and resolves it server-side (R5).
    let res = create_router_with_state(state)
        .oneshot(chat_request(serde_json::json!({
            "messages": [{ "role": "user", "text": "Clarify our target audience" }],
            "context_path": "product_vision.md"
        })))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let _ = body_to_string(res).await;

    let upstream_payload = captured.lock().unwrap().clone().expect("no upstream call");
    let instruction = upstream_payload["systemInstruction"]["parts"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(instruction.contains("product_vision.md"));
    assert!(instruction.contains("Target audience: senior architects."));
    assert_eq!(
        upstream_payload["contents"][0]["parts"][0]["text"],
        "Clarify our target audience"
    );
}

/// Test scenario S3 (guard):
/// epic_02_conversational_copilot_panel#S3 — context binding honours the workspace path guard
/// Given a workspace root,
/// When a co-pilot turn binds a context path escaping that root,
/// Then the server responds 403 with error code TRAVERSAL_DETECTED.
#[tokio::test]
async fn test_epic_02_conversational_copilot_panel_s3_context_traversal_rejected() {
    let tmp = tempdir().unwrap();
    let (base_url, _) = spawn_stub_provider(GEMINI_SSE_BODY, StatusCode::OK).await;
    let app = create_router_with_state(state_with_provider(tmp.path().to_path_buf(), &base_url));

    let res = app
        .oneshot(chat_request(serde_json::json!({
            "messages": [{ "role": "user", "text": "Read this" }],
            "context_path": "../../etc/passwd"
        })))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
    let json: serde_json::Value = serde_json::from_str(&body_to_string(res).await).unwrap();
    assert_eq!(json["error"], "TRAVERSAL_DETECTED");
}

/// Test scenario S4:
/// epic_02_conversational_copilot_panel#S4 — Provider credential is never disclosed
/// Given a credential saved through POST /api/settings/copilot,
/// When a client requests GET /api/settings/copilot,
/// Then the response reports configured: true with the selected model and contains no fragment of
/// the stored key.
#[tokio::test]
async fn test_epic_02_conversational_copilot_panel_s4_credential_never_disclosed() {
    let tmp = tempdir().unwrap();
    let state = AppState::in_memory(tmp.path().to_path_buf());
    let secret = "AIzaSy-super-secret-key";

    let saved = create_router_with_state(state.clone())
        .oneshot(
            Request::builder()
                .uri("/api/settings/copilot")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({ "api_key": secret, "model": "gemini-2.5-pro" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(saved.status(), StatusCode::OK);
    assert!(!body_to_string(saved).await.contains(secret));

    let read = create_router_with_state(state)
        .oneshot(
            Request::builder()
                .uri("/api/settings/copilot")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(read.status(), StatusCode::OK);

    let body = body_to_string(read).await;
    assert!(
        !body.contains(secret),
        "credential leaked in response: {body}"
    );
    assert!(!body.contains("AIzaSy"));

    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["provider"], "gemini");
    assert_eq!(json["configured"], true);
    assert_eq!(json["model"], "gemini-2.5-pro");
    assert_eq!(json["credential_source"], "database");
}

/// Test scenario S5:
/// epic_02_conversational_copilot_panel#S5 — Missing credential rejected before any outbound call
/// Given no provider credential configured,
/// When the user submits a co-pilot prompt,
/// Then the server responds 424 with error code PROVIDER_NOT_CONFIGURED.
// Justified exception: the guard serializes process-wide environment mutation for the whole test
// body, including its await points. A std Mutex is correct here because no other task contends
// for it and the test is single-threaded by construction.
#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn test_epic_02_conversational_copilot_panel_s5_provider_not_configured() {
    let _guard = env_lock().lock().unwrap();
    let previous = std::env::var(liquid_ade::copilot::ENV_API_KEY).ok();
    std::env::remove_var(liquid_ade::copilot::ENV_API_KEY);

    let tmp = tempdir().unwrap();
    // The provider is reachable; the request must still never leave the binary.
    let (base_url, captured) = spawn_stub_provider(GEMINI_SSE_BODY, StatusCode::OK).await;
    let state = AppState::in_memory(tmp.path().to_path_buf()).with_copilot_config(CopilotConfig {
        base_url,
        default_model: "gemini-2.5-flash".to_string(),
    });

    let res = create_router_with_state(state)
        .oneshot(chat_request(serde_json::json!({
            "messages": [{ "role": "user", "text": "Clarify our target audience" }]
        })))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::FAILED_DEPENDENCY);
    let json: serde_json::Value = serde_json::from_str(&body_to_string(res).await).unwrap();
    assert_eq!(json["error"], "PROVIDER_NOT_CONFIGURED");
    assert!(captured.lock().unwrap().is_none(), "provider was called");

    if let Some(value) = previous {
        std::env::set_var(liquid_ade::copilot::ENV_API_KEY, value);
    }
}

/// Malformed conversations are rejected at the boundary (R4, fail-fast contract).
#[tokio::test]
async fn test_epic_02_conversational_copilot_panel_rejects_empty_prompt() {
    let tmp = tempdir().unwrap();
    let (base_url, captured) = spawn_stub_provider(GEMINI_SSE_BODY, StatusCode::OK).await;
    let app = create_router_with_state(state_with_provider(tmp.path().to_path_buf(), &base_url));

    let res = app
        .oneshot(chat_request(serde_json::json!({
            "messages": [{ "role": "user", "text": "   " }]
        })))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let json: serde_json::Value = serde_json::from_str(&body_to_string(res).await).unwrap();
    assert_eq!(json["error"], "EMPTY_PROMPT");
    assert!(captured.lock().unwrap().is_none());
}
