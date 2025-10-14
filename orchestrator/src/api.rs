// Handles /run endpoint
use axum::{Json, extract::State};
use serde::Deserialize;
#[derive(Deserialize)]
struct FlowSpec { name: String }
pub async fn run(Json(spec): Json<FlowSpec>) -> String {
 format!("Started workflow: {}", spec.name)
}
