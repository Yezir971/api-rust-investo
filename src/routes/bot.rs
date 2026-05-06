use axum::{routing::{post, get}, Router};
use crate::schema::{AppState};
use crate::handlers::bot::{add_api_key, status_bot, start_bot};

pub fn bot_routes() -> Router<AppState> {
    Router::new()
        .route("/keys", post(add_api_key))
        .route("/status/{id}", get(status_bot))
        .route("/start", post(start_bot))
        // ---------- Must have --------------
        // .route("/stop", post(add_api_key))
        // .route("/solde/:user_id", get(add_api_key))
        // ---------- Could have ------------ 
        // .route("/buy/:user_id", post(add_api_key))
        // .route("/delete", delete(add_api_key))
    }