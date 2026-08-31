use axum::{routing::get, Router};
use rust_embed::RustEmbed;
use std::net::SocketAddr;

#[derive(RustEmbed)]
#[folder = "ui/dist/"]
struct Assets;

#[tokio::main]
async fn main() {
    println!("Starting ACP Engine: Liquid ADE...");

    // API Routes / ACP Server + Catch-all route for Frontend (React)
    let app = Router::new()
        .route("/api/health", get(|| async { "ACP Engine (Rust) is online." }));
        // TODO: Plug the rust_embed static file router here

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Frontend and Engine running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
