use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;
use crate::models::user::{UserModel, UserResponse}; 
use crate::schema::user::CreateUserSchema;
use hashed_password::HashedPassword;

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let hp = HashedPassword::from_plain(&body.password, b"my-secret");
    let hashed_password_str = hp.to_string();
    let id = uuid::Uuid::new_v4();
    let result = sqlx::query_as!(
        UserModel,
        r#"INSERT INTO users (id, name, lastname, password, email) VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        &id,
        &body.name,
        &body.lastname,
        &hashed_password_str,
        &body.email
    ).fetch_one(&pool).await;

    match result {
        Ok(user) => {
            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
            };
            
            Ok((StatusCode::CREATED, Json(json!({
                "status": "success",
                "data": user_response
            }))))
        },
        Err(e) => {
            let err_msg = e.to_string();
            let status = if err_msg.contains("duplicate key") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            
            Err((status, Json(json!({
                "status": "error",
                "message": err_msg
            }))))
        }
    }
}
