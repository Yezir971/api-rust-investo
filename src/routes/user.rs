
use axum::{routing::{post, get}, Router};
use crate::schema::{AppState};
use crate::handlers::{create_user_handler,get_user_handler, auth_user_handler };

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user_handler))
        .route("/{id}", get(get_user_handler))
        .route("/auth", post(auth_user_handler))
}