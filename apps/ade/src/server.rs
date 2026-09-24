use axum::{
    body::Body,
    http::{header, Method, Response, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use rust_embed::RustEmbed;
use serde::Serialize;

#[derive(RustEmbed)]
#[folder = "ui/dist/"]
pub struct Assets;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct HealthResponse {
    pub status: &'static str,
    pub app: &'static str,
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

pub fn create_router() -> Router {
    Router::new()
        .route("/api/health", get(health_handler))
        .fallback(static_or_spa_handler)
}
