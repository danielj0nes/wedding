use axum::{Router, routing::get_service, http::StatusCode};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Require WEDDING_FRONTEND_DIR env var (no fallback)
    let wedding_frontend_dir = match env::var("WEDDING_FRONTEND_DIR") {
        Ok(v) if !v.is_empty() => v,
        _ => {
            eprintln!("Environment variable 'WEDDING_FRONTEND_DIR' is not set");
            std::process::exit(1);
        }
    };

    // Serve the built Svelte frontend files (from the /build dir)
    let static_service = get_service(ServeDir::new(&wedding_frontend_dir)).handle_error(|_| async move {
        (StatusCode::INTERNAL_SERVER_ERROR, "Static file error")
    });

    // Let the static service serve files at the root (it will serve index.html)
    let app = Router::new().fallback_service(static_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
