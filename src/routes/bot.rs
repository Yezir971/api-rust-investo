use axum::{routing::post, Router};
use crate::schema::{AppState};
use crate::handlers::bot::add_api_key;

pub fn bot_routes() -> Router<AppState> {
    Router::new()
        .route("/keys", post(add_api_key))
}