use axum::{routing::post, Router};

static APP_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = 6666;

    let app = Router::new().route("/shutdown", post(shutdown));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("Starting {APP_NAME} on port {port}");
    axum::serve(listener, app).await.unwrap();
}

async fn shutdown() -> String {
    if cfg!(debug_assertions) {
        "fake shutdown in debug mode".to_string()
    } else {
        match system_shutdown::shutdown() {
            Ok(_) => "success".to_string(),
            Err(e) => e.to_string(),
        }
    }
}
