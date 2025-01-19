use axum::{
    routing::{get, post},
    Router,
};
use routes::test::post::*;
mod routes;
#[tokio::main]
async fn main() {
    println!("Server Start");
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/test", post(post_test));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
