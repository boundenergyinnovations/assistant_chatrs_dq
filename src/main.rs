use axum::{
    routing::{post, get},
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod openai;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/chat", post(routes::chat))
        .nest_service("/", ServeDir::new("static").fallback(ServeDir::new("static").append_index_html_on_directories(true)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Proxy server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
