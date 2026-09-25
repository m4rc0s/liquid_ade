use crate::copilot::{
    self, ChatRequest, CopilotConfig, CredentialSource, SseFrameParser, PROVIDER_GEMINI,
};
use crate::db::{self, ProjectRecord};
use crate::workspace::{self, FileNode, PathGuardError};
use axum::{
    body::Body,
    extract::{Query, State},
    http::{header, Method, Response, StatusCode, Uri},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    routing::{get, post},
    Json, Router,
};
use futures_util::{Stream, StreamExt};
use rusqlite::Connection;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

#[derive(RustEmbed)]
#[folder = "ui/dist/"]
pub struct Assets;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub workspace_root: Arc<RwLock<PathBuf>>,
    pub copilot: Arc<CopilotConfig>,
    pub http: reqwest::Client,
}

impl AppState {
    pub fn new(db: Connection, workspace_root: PathBuf) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            workspace_root: Arc::new(RwLock::new(workspace_root)),
            copilot: Arc::new(CopilotConfig::from_env()),
            http: reqwest::Client::new(),
        }
    }

    pub fn in_memory(workspace_root: PathBuf) -> Self {
        let conn = db::init_db_at(":memory:").expect("Failed to init in-memory database");
        Self::new(conn, workspace_root)
    }

    /// Overrides the Multi-LLM Gateway configuration, allowing a stub provider to be injected.
    pub fn with_copilot_config(mut self, config: CopilotConfig) -> Self {
        self.copilot = Arc::new(config);
        self
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct HealthResponse {
    pub status: &'static str,
    pub app: &'static str,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

pub async fn health_handler() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok",
            app: "liquid-ade",
        }),
    )
}

pub async fn workspace_tree_handler(
    State(state): State<AppState>,
) -> Result<Json<FileNode>, (StatusCode, Json<ErrorResponse>)> {
    let root = state.workspace_root.read().unwrap().clone();
    match workspace::scan_workspace_tree(&root) {
        Ok(tree) => Ok(Json(tree)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "SCAN_ERROR".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct FileQuery {
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct FileContentResponse {
    pub path: String,
    pub content: String,
}

pub async fn read_file_handler(
    State(state): State<AppState>,
    Query(query): Query<FileQuery>,
) -> Result<Json<FileContentResponse>, (StatusCode, Json<ErrorResponse>)> {
    let root = state.workspace_root.read().unwrap().clone();
    match workspace::read_workspace_file(&root, &query.path) {
        Ok(content) => Ok(Json(FileContentResponse {
            path: query.path,
            content,
        })),
        Err(PathGuardError::TraversalDetected) => Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "TRAVERSAL_DETECTED".to_string(),
                message: "Path traversal attempt detected".to_string(),
            }),
        )),
        Err(PathGuardError::NotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "NOT_FOUND".to_string(),
                message: "File not found in workspace".to_string(),
            }),
        )),
        Err(PathGuardError::Io(msg)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "IO_ERROR".to_string(),
                message: msg,
            }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct WriteFileRequest {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct WriteFileResponse {
    pub status: &'static str,
    pub path: String,
}

pub async fn write_file_handler(
    State(state): State<AppState>,
    Json(payload): Json<WriteFileRequest>,
) -> Result<Json<WriteFileResponse>, (StatusCode, Json<ErrorResponse>)> {
    let root = state.workspace_root.read().unwrap().clone();
    match workspace::write_workspace_file(&root, &payload.path, &payload.content) {
        Ok(_) => Ok(Json(WriteFileResponse {
            status: "ok",
            path: payload.path,
        })),
        Err(PathGuardError::TraversalDetected) => Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "TRAVERSAL_DETECTED".to_string(),
                message: "Path traversal attempt detected".to_string(),
            }),
        )),
        Err(PathGuardError::Io(msg)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "IO_ERROR".to_string(),
                message: msg,
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "WRITE_FAILED".to_string(),
                message: "Failed to write file".to_string(),
            }),
        )),
    }
}

pub async fn list_projects_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectRecord>>, (StatusCode, Json<ErrorResponse>)> {
    let conn = state.db.lock().unwrap();
    match db::list_projects(&conn) {
        Ok(projects) => Ok(Json(projects)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "DB_ERROR".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenProjectRequest {
    pub path: String,
}

pub async fn open_project_handler(
    State(state): State<AppState>,
    Json(payload): Json<OpenProjectRequest>,
) -> Result<Json<ProjectRecord>, (StatusCode, Json<ErrorResponse>)> {
    let p = Path::new(&payload.path);
    if !p.exists() || !p.is_dir() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "DIRECTORY_NOT_FOUND".to_string(),
                message: format!("Directory does not exist: {}", payload.path),
            }),
        ));
    }

    let canonical_path = match p.canonicalize() {
        Ok(c) => c,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "CANONICALIZE_ERROR".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    };
    let canonical_str = canonical_path.to_string_lossy().to_string();

    let record = {
        let conn = state.db.lock().unwrap();
        match db::open_or_register_project(&conn, &canonical_str, None) {
            Ok(rec) => rec,
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "DB_ERROR".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    };

    *state.workspace_root.write().unwrap() = canonical_path;
    Ok(Json(record))
}

#[derive(Debug, Deserialize)]
pub struct NewProjectRequest {
    pub name: String,
    pub path: String,
}

pub async fn new_project_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewProjectRequest>,
) -> Result<(StatusCode, Json<ProjectRecord>), (StatusCode, Json<ErrorResponse>)> {
    let name = payload.name.trim();
    if !crate::scaffold::is_valid_slug(name) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "INVALID_PROJECT_NAME".to_string(),
                message: format!("Invalid project slug '{name}'. Must match ^[a-z0-9][a-z0-9_-]*$"),
            }),
        ));
    }

    let target_path = Path::new(&payload.path);
    if let Err(e) = crate::scaffold::scaffold_scpe_workspace(target_path, name) {
        let status = if e.kind() == std::io::ErrorKind::AlreadyExists {
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        let err_code = if e.kind() == std::io::ErrorKind::AlreadyExists {
            "DIRECTORY_NOT_EMPTY"
        } else {
            "IO_ERROR"
        };
        return Err((
            status,
            Json(ErrorResponse {
                error: err_code.to_string(),
                message: e.to_string(),
            }),
        ));
    }

    let canonical_path = match target_path.canonicalize() {
        Ok(c) => c,
        Err(_) => target_path.to_path_buf(),
    };
    let canonical_str = canonical_path.to_string_lossy().to_string();

    let record = {
        let conn = state.db.lock().unwrap();
        match db::open_or_register_project(&conn, &canonical_str, Some(name)) {
            Ok(rec) => rec,
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "DB_ERROR".to_string(),
                        message: e.to_string(),
                    }),
                ))
            }
        }
    };

    *state.workspace_root.write().unwrap() = canonical_path;
    Ok((StatusCode::CREATED, Json(record)))
}

#[derive(Debug, Serialize)]
pub struct CopilotSettingsResponse {
    pub provider: &'static str,
    pub model: String,
    pub configured: bool,
    pub credential_source: CredentialSource,
}

#[derive(Debug, Deserialize)]
pub struct CopilotSettingsRequest {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
}

/// Resolves the provider credential in force: database first, environment as fallback (R3).
fn resolve_credential(state: &AppState) -> Result<(Option<String>, CredentialSource), String> {
    let stored = {
        let conn = state.db.lock().unwrap();
        db::get_setting(&conn, copilot::SETTING_API_KEY).map_err(|e| e.to_string())?
    };

    if let Some(key) = stored
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty())
    {
        return Ok((Some(key), CredentialSource::Database));
    }

    if let Some(key) = std::env::var(copilot::ENV_API_KEY)
        .ok()
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty())
    {
        return Ok((Some(key), CredentialSource::Environment));
    }

    Ok((None, CredentialSource::None))
}

/// Resolves the model in force: workspace setting first, gateway default as fallback.
fn resolve_model(state: &AppState) -> String {
    let stored = {
        let conn = state.db.lock().unwrap();
        db::get_setting(&conn, copilot::SETTING_MODEL)
            .ok()
            .flatten()
    };
    stored
        .map(|m| m.trim().to_string())
        .filter(|m| !m.is_empty())
        .unwrap_or_else(|| state.copilot.default_model.clone())
}

fn db_error(message: String) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: "DB_ERROR".to_string(),
            message,
        }),
    )
}

pub async fn get_copilot_settings_handler(
    State(state): State<AppState>,
) -> Result<Json<CopilotSettingsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let (credential, source) = resolve_credential(&state).map_err(db_error)?;
    Ok(Json(CopilotSettingsResponse {
        provider: PROVIDER_GEMINI,
        model: resolve_model(&state),
        configured: credential.is_some(),
        credential_source: source,
    }))
}

pub async fn put_copilot_settings_handler(
    State(state): State<AppState>,
    Json(payload): Json<CopilotSettingsRequest>,
) -> Result<Json<CopilotSettingsResponse>, (StatusCode, Json<ErrorResponse>)> {
    {
        let conn = state.db.lock().unwrap();

        if let Some(api_key) = payload.api_key.as_ref().map(|k| k.trim()) {
            if api_key.is_empty() {
                db::delete_setting(&conn, copilot::SETTING_API_KEY)
                    .map_err(|e| db_error(e.to_string()))?;
            } else {
                db::set_setting(&conn, copilot::SETTING_API_KEY, api_key)
                    .map_err(|e| db_error(e.to_string()))?;
            }
        }

        if let Some(model) = payload.model.as_ref().map(|m| m.trim()) {
            if model.is_empty() {
                db::delete_setting(&conn, copilot::SETTING_MODEL)
                    .map_err(|e| db_error(e.to_string()))?;
            } else {
                db::set_setting(&conn, copilot::SETTING_MODEL, model)
                    .map_err(|e| db_error(e.to_string()))?;
            }
        }
    }

    get_copilot_settings_handler(State(state)).await
}

/// Normalized SSE error event, so the panel can render an inline banner without parsing HTTP.
fn sse_error_event(code: &str, message: &str) -> Event {
    Event::default()
        .event("error")
        .data(serde_json::json!({ "error": code, "message": message }).to_string())
}

/// `POST /api/copilot/chat` — streams a co-pilot answer as normalized `token` / `error` / `done`
/// Server-Sent Events, proxying the configured Multi-LLM Gateway provider.
pub async fn copilot_chat_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, (StatusCode, Json<ErrorResponse>)> {
    // Fail fast on malformed conversations, before any outbound call (R4).
    copilot::validate_chat_request(&payload).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.code().to_string(),
                message: e.message().to_string(),
            }),
        )
    })?;

    let (credential, _) = resolve_credential(&state).map_err(db_error)?;
    let Some(api_key) = credential else {
        return Err((
            StatusCode::FAILED_DEPENDENCY,
            Json(ErrorResponse {
                error: "PROVIDER_NOT_CONFIGURED".to_string(),
                message: "No co-pilot provider credential configured. Save one in Settings or set \
                          the GEMINI_API_KEY environment variable."
                    .to_string(),
            }),
        ));
    };

    // The bound document is read server-side through the workspace path guard (R5).
    let context_content = match payload.context_path.as_deref() {
        Some(path) if !path.trim().is_empty() => {
            let root = state.workspace_root.read().unwrap().clone();
            match workspace::read_workspace_file(&root, path) {
                Ok(content) => Some(content),
                Err(PathGuardError::TraversalDetected) => {
                    return Err((
                        StatusCode::FORBIDDEN,
                        Json(ErrorResponse {
                            error: "TRAVERSAL_DETECTED".to_string(),
                            message: "Path traversal attempt detected in context_path".to_string(),
                        }),
                    ))
                }
                Err(PathGuardError::NotFound) => {
                    return Err((
                        StatusCode::NOT_FOUND,
                        Json(ErrorResponse {
                            error: "CONTEXT_NOT_FOUND".to_string(),
                            message: format!("Bound document not found in workspace: {path}"),
                        }),
                    ))
                }
                Err(PathGuardError::Io(message)) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "IO_ERROR".to_string(),
                            message,
                        }),
                    ))
                }
            }
        }
        _ => None,
    };

    let model = resolve_model(&state);
    let endpoint = state.copilot.stream_endpoint(&model);
    let body = copilot::build_gemini_payload(&payload, context_content.as_deref());

    let upstream = state
        .http
        .post(&endpoint)
        .header("x-goog-api-key", &api_key)
        .header(header::ACCEPT, "text/event-stream")
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse {
                    error: "UPSTREAM_UNAVAILABLE".to_string(),
                    message: format!("Co-pilot provider is unreachable: {e}"),
                }),
            )
        })?;

    if !upstream.status().is_success() {
        let status = upstream.status();
        let detail = upstream.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: "UPSTREAM_UNAVAILABLE".to_string(),
                message: format!("Co-pilot provider rejected the request ({status}): {detail}"),
            }),
        ));
    }

    let mut bytes = upstream.bytes_stream();
    let stream = async_stream::stream! {
        let mut parser = SseFrameParser::new();
        let mut failed = false;

        while let Some(chunk) = bytes.next().await {
            match chunk {
                Ok(chunk) => {
                    let text = String::from_utf8_lossy(&chunk).into_owned();
                    for raw in parser.push(&text) {
                        if raw.trim() == "[DONE]" {
                            continue;
                        }
                        match serde_json::from_str::<serde_json::Value>(&raw) {
                            Ok(value) => {
                                if let Some(error) = value.get("error") {
                                    failed = true;
                                    let message = error
                                        .get("message")
                                        .and_then(|m| m.as_str())
                                        .unwrap_or("Unknown provider error");
                                    yield Ok(sse_error_event("UPSTREAM_ERROR", message));
                                } else if let Some(token) =
                                    copilot::extract_text_from_gemini_chunk(&value)
                                {
                                    yield Ok(Event::default().event("token").data(
                                        serde_json::json!({ "text": token }).to_string(),
                                    ));
                                }
                            }
                            Err(e) => {
                                failed = true;
                                yield Ok(sse_error_event(
                                    "UPSTREAM_MALFORMED_CHUNK",
                                    &format!("Unparsable provider chunk: {e}"),
                                ));
                            }
                        }
                    }
                }
                Err(e) => {
                    failed = true;
                    yield Ok(sse_error_event(
                        "UPSTREAM_STREAM_FAILED",
                        &format!("Provider stream interrupted: {e}"),
                    ));
                    break;
                }
            }
        }

        if !failed {
            yield Ok(Event::default()
                .event("done")
                .data(serde_json::json!({ "status": "ok" }).to_string()));
        }
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

pub async fn static_or_spa_handler(method: Method, uri: Uri) -> impl IntoResponse {
    if method != Method::GET && method != Method::HEAD {
        return Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(Body::from("405 Method Not Allowed"))
            .unwrap();
    }

    let path = uri.path().trim_start_matches('/');

    // 1. Try serving embedded static asset (e.g. css, js, svg, png)
    if !path.is_empty() {
        if let Some(file) = Assets::get(path) {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            return Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(file.data))
                .unwrap();
        }
    }

    // 2. SPA fallback: serve index.html for root ("/") or any unknown path
    if let Some(index_html) = Assets::get("index.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(Body::from(index_html.data))
            .unwrap();
    }

    // 3. Fallback when index.html is missing
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from("404 Not Found"))
        .unwrap()
}

pub fn create_router_with_state(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/workspace/tree", get(workspace_tree_handler))
        .route(
            "/api/workspace/file",
            get(read_file_handler).post(write_file_handler),
        )
        .route("/api/projects", get(list_projects_handler))
        .route("/api/projects/open", post(open_project_handler))
        .route("/api/projects/new", post(new_project_handler))
        .route("/api/workspace/new", post(new_project_handler))
        .route(
            "/api/settings/copilot",
            get(get_copilot_settings_handler).post(put_copilot_settings_handler),
        )
        .route("/api/copilot/chat", post(copilot_chat_handler))
        .fallback(static_or_spa_handler)
        .with_state(state)
}

pub fn create_router() -> Router {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let conn = db::init_db().unwrap_or_else(|_| {
        db::init_db_at(":memory:").expect("Failed to initialize database fallback")
    });
    let state = AppState::new(conn, current_dir);
    create_router_with_state(state)
}
