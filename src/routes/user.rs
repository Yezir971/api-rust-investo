
use axum::{routing::{post, get}, Router};
use sqlx::PgPool;
use crate::handlers::user::{create_user_handler,get_user_handler };

pub fn user_routes() -> Router<PgPool> {
    Router::new()
        .route("/", post(create_user_handler))
        .route("/{id}", get(get_user_handler))
}