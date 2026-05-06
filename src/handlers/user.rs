use axum::{Json, extract::{State, Path}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::utils::utils::verify_owner;
use crate::schema::{UserModel, UserResponse,CreateUserSchema, AuthUserSchema,Claims, AppState, CreateUserModel};
use hashed_password::HashedPassword;
use uuid::Uuid;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};



pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let hp = HashedPassword::from_plain(&body.password, &state.jwt_secret);
    let hashed_password_str = hp.to_string();
    let id = uuid::Uuid::new_v4();
    let result = sqlx::query_as!(
        CreateUserModel,
        r#"
        INSERT INTO users (id, name, lastname, password, email, virtual_balance) 
        VALUES ($1, $2, $3, $4, $5, $6) 
        RETURNING 
            id, 
            name, 
            lastname, 
            password, 
            email, 
            virtual_balance as "virtual_balance!"
        "#,
        &id,
        &body.name,
        &body.lastname,
        &hashed_password_str,
        &body.email,
        1000.0
    ).fetch_one(&state.pool).await;

    match result {
        Ok(user) => {
            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                virtual_balance: user.virtual_balance,
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
pub async fn get_user_handler(
    State(state): State<AppState>,
    extensions: axum::Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    verify_owner(&extensions.sub, id)?;
    
    let result = sqlx::query_as!(
        UserModel,
        r#"SELECT id, name, lastname, password, email,created_at , virtual_balance as "virtual_balance!" FROM users WHERE id = $1"#,
        &id,
    )
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(user) => {
            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                virtual_balance: user.virtual_balance
            };
            Ok((StatusCode::OK, Json(json!({
                "status": "success",
                "data": user_response
            }))))
        },
        Err(e) => {
            let err_msg = e.to_string();
            let (status, message) = if err_msg.contains("no rows returned") {
                (StatusCode::NOT_FOUND, "User not found".to_string())
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, err_msg)
            };
            
            Err((status, Json(json!({
                "status": "error",
                "message": message
            }))))
        }
    }
}

pub async fn auth_user_handler(
    State(state): State<AppState>,
    Json(body): Json<AuthUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    let result = sqlx::query_as!(
        UserModel, 
        r#"SELECT id, name, lastname, password, email,created_at , virtual_balance as "virtual_balance!" FROM users WHERE email=$1"#,
        &body.email
    ).fetch_one(&state.pool).await;

    match result {
        Ok(user) => {
            let stored_hp = HashedPassword::new(&user.password);
            if !stored_hp.validate(&body.password, &state.jwt_secret){
                return Err((
                    StatusCode::UNAUTHORIZED, 
                    Json(json!({"status": "error", "message": "Email ou mot de passe incorrect"}))
                ));
            }

            // date d'expration du token 
            let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

            let my_claims = Claims {
                sub: user.id.to_string(),
                exp: expiration,
            };
            let jwt_user = encode(
                &Header::default(), 
                &my_claims, 
                &EncodingKey::from_secret(&state.jwt_secret.as_ref())
            ).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Erreur génération token"}))))?;

            Ok((StatusCode::OK, Json(json!({
                "status": "success",
                "data": jwt_user,
                "user": UserResponse {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    virtual_balance: user.virtual_balance,
                }
            }))))
        }
        Err(_e) => {
            Err((
                StatusCode::UNAUTHORIZED, 
                Json(json!({"status": "error", "message": "Email ou mot de passe incorrect"}))
            ))

        }
    }

}