use axum::{Router, routing::{get_service, post}, http::StatusCode, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use axum::extract::ConnectInfo;

#[derive(Deserialize)]
struct RsvpForm {
    name: String,
    email: String,
    attending: String,
    comments: Option<String>,
}

#[derive(Serialize)]
struct SendGridEmail {
    personalizations: Vec<Personalization>,
    from: EmailAddress,
    content: Vec<Content>,
}

#[derive(Serialize)]
struct Personalization {
    to: Vec<EmailAddress>,
    subject: String,
}

#[derive(Serialize)]
struct EmailAddress {
    email: String,
}

#[derive(Serialize)]
struct Content {
    #[serde(rename = "type")]
    content_type: String,
    value: String,
}

async fn register(ConnectInfo(addr): ConnectInfo<SocketAddr>, Json(payload): Json<RsvpForm>) -> StatusCode {
    println!(
        "RSVP Received: Name: {}, Email: {}, Attending: {}, Comments: {:?} IP {}",
        payload.name, payload.email, payload.attending, payload.comments, addr.ip()
    );

    // Send email via SendGrid
    let api_key = env::var("SENDGRID_API_KEY").unwrap_or("".to_string());
    let from_email = env::var("FROM_EMAIL").unwrap_or("".to_string());

    let email_body = format!(
        "Name: {}\nEmail: {}\nAttending: {}\nComments: {}\nIP Address: {}",
        payload.name, payload.email, payload.attending, payload.comments.as_ref().unwrap_or(&"".to_string()), addr.ip()
    );

    let sendgrid_payload = SendGridEmail {
        personalizations: vec![Personalization {
            to: vec![EmailAddress {
                email: "".to_string(),
            }],
            subject: "New RSVP Received".to_string(),
        }],
        from: EmailAddress {
            email: from_email,
        },
        content: vec![Content {
            content_type: "text/plain".to_string(),
            value: email_body,
        }],
    };

    let client = Client::new();
    let response = client
        .post("https://api.sendgrid.com/v3/mail/send")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&sendgrid_payload)
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            println!("Email sent successfully via SendGrid");
        }
        Ok(resp) => {
            println!("Failed to send email: {}", resp.status());
        }
        Err(e) => {
            println!("Error sending email: {:?}", e);
        }
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

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
