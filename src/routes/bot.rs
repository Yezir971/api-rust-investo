use axum::{routing::{post, get}, Router};
use crate::schema::{AppState};
use crate::handlers::bot::{add_api_key, status_bot, start_bot, stop_bot, get_user_balance};

pub fn bot_routes() -> Router<AppState> {
    Router::new()
        .route("/keys", post(add_api_key))
        .route("/status/{id}", get(status_bot))
        .route("/start", post(start_bot))
        .route("/stop", post(stop_bot))
        .route("/solde/{id}", get(get_user_balance))
        // ---------- Must have --------------
        // ---------- Could have ------------ 
        // .route("/buy/:user_id", post(add_api_key))
        // .route("/delete", delete(add_api_key))
}
