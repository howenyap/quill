use axum::extract::{Form, Path};
use axum::http::StatusCode;
use axum::{
    Router,
    routing::{get, post},
};
use serde::Deserialize;
use tokio::net::TcpListener;

pub struct Server {
    router: Router,
    address: String,
}

impl Server {
    fn new() -> Self {
        Self::with_address("127.0.0.1:0".to_string())
    }

    pub fn with_address(address: String) -> Self {
        Self {
            router: server(),
            address,
        }
    }

    pub async fn run(self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        axum::serve(listener, self.router)
            .await
            .expect("Failed to start server");

        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}!", name)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct SubscriptionForm {
    name: String,
    email: String,
}

async fn subscribe(Form(_payload): Form<SubscriptionForm>) -> StatusCode {
    StatusCode::OK
}

pub fn server() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/greet/{name}", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}
