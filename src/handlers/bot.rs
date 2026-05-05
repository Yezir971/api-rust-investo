use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use shared::repositories::api_key_repo;
use crate::schema::bot::CreateApiKeySchema;
use crate::schema::AppState; 


pub async fn add_api_key(
    State(state): State<AppState>,
    Json(payload): Json<CreateApiKeySchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    
    // On récupère la clé de chiffrement
    let master_key = state.master_encryption_key;

    api_key_repo::save_api_key(
        &state.pool,
        payload.user_id, 
        &payload.exchange,
        &payload.api_key,
        &payload.api_secret,
        master_key.as_bytes()
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur BDD: {}", e)))?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({"message": "Clé API enregistrée et chiffrée"}))))
}