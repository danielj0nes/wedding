use axum::{Router, routing::{get_service, post}, http::StatusCode, Json};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use serde::Deserialize;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct RsvpForm {
    name: String,
    email: String,
    attending: String,
    comments: Option<String>,
}

async fn register(Json(payload): Json<RsvpForm>) -> StatusCode {
    println!(
        "RSVP Received: Name: {}, Email: {}, Attending: {}, Comments: {:?}",
        payload.name, payload.email, payload.attending, payload.comments
    );

    // Send email
    let smtp_host = env::var("SMTP_HOST").unwrap_or("smtp.gmail.com".to_string());
    let smtp_username = env::var("SMTP_USERNAME").unwrap_or("".to_string());
    let smtp_password = env::var("SMTP_PASSWORD").unwrap_or("".to_string());

    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .to("daniel@jones.ac".parse().unwrap())
        .subject("New RSVP Received")
        .body(format!(
            "Name: {}\nEmail: {}\nAttending: {}\nComments: {}",
            payload.name, payload.email, payload.attending, payload.comments.as_deref().unwrap_or("")
        ))
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => println!("Could not send email: {:?}", e),
    }

    StatusCode::OK
}

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

    // Add the /register route
    let app = Router::new()
        .route("/register", post(register))
        .fallback_service(static_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
