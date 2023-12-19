use std::path::PathBuf;

use askama::Template;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("routing initialized...");
    let app = Router::new()
        .route("/", get(hello))
        // Serve static files from the "assets" directory
        .nest_service("/assets", ServeDir::new(PathBuf::from("./assets")))
        .layer(tower_http::trace::TraceLayer::new_for_http());
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> HelloTemplate {
    HelloTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate;
