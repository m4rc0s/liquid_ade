use axum::{routing::get, Router};
use rust_embed::RustEmbed;
use std::net::SocketAddr;

#[derive(RustEmbed)]
#[folder = "ui/dist/"]
struct Assets;

#[tokio::main]
async fn main() {
    println!("Iniciando Motor ACP: Liquid ADE...");

    // Rotas da API / ACP Server + Rota Catch-all para o Frontend (React)
    let app = Router::new()
        .route("/api/health", get(|| async { "O Motor ACP (Rust) está online." }));
        // TODO: Plugar o router de arquivos estáticos do rust_embed aqui

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Frontend e Motor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
