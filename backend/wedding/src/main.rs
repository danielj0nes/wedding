use axum::Router;
use axum::http::StatusCode;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::env;
use tower_http::services::ServeDir;
use axum::routing::get_service;

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

    // Serve static files from wedding_frontend_dir
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
