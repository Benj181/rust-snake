use axum::Router;
use tower_http::services::ServeDir;

const ADDR: &str = "127.0.0.1:3000";
/// Output of `trunk build` in the ui crate (path relative to the workspace root).
const STATIC_DIR: &str = "ui/dist";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Later: add `.route("/ws", ...)` here for multiplayer.
    let app = Router::new().fallback_service(ServeDir::new(STATIC_DIR));

    let listener = tokio::net::TcpListener::bind(ADDR).await?;
    println!("serving {STATIC_DIR} on http://{ADDR}");
    axum::serve(listener, app).await
}
