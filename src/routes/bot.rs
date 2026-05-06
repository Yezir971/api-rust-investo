use axum::{routing::post, Router};
use crate::schema::{AppState};
use crate::handlers::bot::add_api_key;

pub fn bot_routes() -> Router<AppState> {
    Router::new()
        .route("/keys", post(add_api_key))
        // ---------- Must have --------------
        // .route("/start", post(add_api_key))
        // .route("/stop", post(add_api_key))
        // .route("/solde/:user_id", get(add_api_key))
        // .route("/status/:user_id", get(add_api_key))
        // ---------- Could have ------------ 
        // .route("/buy/:user_id", post(add_api_key))
        // .route("/delete", delete(add_api_key))
    }