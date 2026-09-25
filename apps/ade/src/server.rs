use crate::db::{self, ProjectRecord};
use crate::workspace::{self, FileNode, PathGuardError};
use axum::{
    body::Body,
    extract::{Query, State},
    http::{header, Method, Response, StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rusqlite::Connection;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

#[derive(RustEmbed)]
#[folder = "ui/dist/"]
pub struct Assets;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub workspace_root: Arc<RwLock<PathBuf>>,
}

impl AppState {
    pub fn new(db: Connection, workspace_root: PathBuf) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            workspace_root: Arc::new(RwLock::new(workspace_root)),
        }
    }

    pub fn in_memory(workspace_root: PathBuf) -> Self {
        let conn = db::init_db_at(":memory:").expect("Failed to init in-memory database");
        Self::new(conn, workspace_root)
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
