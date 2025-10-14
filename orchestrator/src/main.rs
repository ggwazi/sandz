// Entry point
mod api;
mod runner;
mod db;
use axum::{Router, routing::post};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
 let app = Router::new().route("/run", post(api::run));
 let addr = SocketAddr::from(([0,0,0,0], 8080));
 axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
