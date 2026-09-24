use liquid_ade::{db, server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting ACP Engine: Liquid ADE...");

    // Initialize embedded SQLite database at ~/.liquid/liquid.db
    let _conn = db::init_db().map_err(|e| format!("Failed to initialize database: {e}"))?;
    println!("Embedded SQLite database initialized successfully at ~/.liquid/liquid.db");

    let app = server::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Frontend and Engine running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
