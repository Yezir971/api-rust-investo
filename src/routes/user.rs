
use axum::{routing::{post}, Router};
use sqlx::PgPool;
use crate::handlers::user::{create_user_handler };

pub fn user_routes() -> Router<PgPool> {
    Router::new()
        .route("/user/create", post(create_user_handler))
}