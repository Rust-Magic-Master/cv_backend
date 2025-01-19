use axum::{routing::post, Router};
use routes::test::post::*;
use tower_http::services::ServeDir;
mod routes;
#[tokio::main]
async fn main() {
    println!("Server Start");
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/test", post(post_test));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
