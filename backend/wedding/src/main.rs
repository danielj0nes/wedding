use axum::{routing::get, Router, response::Html, response::IntoResponse};
use axum::http::StatusCode;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::fs;
use std::env;

#[tokio::main]
async fn main() {
    async fn svelte_home() -> impl IntoResponse {
    // Require WEDDING_FRONTEND_DIR env var (no fallback) so runtime layout is explicit
    let wedding_frontend_dir = match env::var("WEDDING_FRONTEND_DIR") {
            Ok(v) if !v.is_empty() => v,
            _ => {
        let msg = "Environment variable 'WEDDING_FRONTEND_DIR' is not set";
                eprintln!("{}", msg);
                return (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response();
            }
        };

        let path = format!("{}/test.html", wedding_frontend_dir);

        match fs::read_to_string(&path).await {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                eprintln!("Failed to read {}: {}", path, err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }

    let app = Router::new()
        .route("/", get(|| async { "Hello from Axum!" }))
        .route("/home", get(svelte_home));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
