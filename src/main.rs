use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Hello form Axum"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listener on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
