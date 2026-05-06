use axum::{extract::{State, Path}, Json, http::StatusCode, response::IntoResponse};
use shared::repositories::api_key_repo;
use crate::schema::bot::{CreateApiKeySchema, StartBotSchema};
use crate::schema::AppState; 
use uuid::Uuid;
use std::sync::Arc;

use bot_investo::brain::run_brain_bot; 
use bot_investo::exchange::cryptocom::CryptoComExchange;
use crate::schema::BotControl;

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

pub async fn status_bot(
    State(state): State<AppState>,
    Path(id): Path<Uuid>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
        let is_active = state.active_bots.contains_key(&id);
        Ok(Json(serde_json::json!({
            "status": "success",
            "message": if is_active { "Le bot travaille" } else { "Le bot est au repos" },
            "is_active": is_active,
        
        })))
}

pub async fn start_bot(
    State(state): State<AppState>,
    Json(body): Json<StartBotSchema>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>{
        // Vérifie si le bot ne tourne pas déjà
        if state.active_bots.contains_key(&body.user_id) {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"message": "Bot déjà lancé", "status": "error"})).to_string()));
        }

        // Récupère les clés en BDD (déchiffrées)
        let keys = api_key_repo::get_decrypted_keys(
            &state.pool, 
            body.user_id, 
            state.master_encryption_key.as_bytes()
        ).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur BDD".to_string()))?;

        let key_info = keys.first().ok_or((StatusCode::NOT_FOUND, "Pas de clés".to_string()))?;
        let url_brocker = "https://api.crypto.com/v2/";
        
        
        let exchange = Arc::new(CryptoComExchange::new(
            key_info.key.clone(),
            key_info.secret.clone(),
            url_brocker.to_string()
        ));

        // 4. Spawn
        let id_str = body.user_id;
        let handle = tokio::spawn(async move {
            run_brain_bot(&id_str.to_string(), exchange, "BTC_USDT".to_string()).await;
        });

        // Enregistre dans la DashMap
        state.active_bots.insert(body.user_id, BotControl { handle });


        Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Le bot est au repos" ,
            "is_active": "is_active",
        
        })))
    

}