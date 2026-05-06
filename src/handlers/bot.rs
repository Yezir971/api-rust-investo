use axum::{extract::{State, Path}, Json, http::StatusCode, response::IntoResponse};
use shared::repositories::api_key_repo;
use crate::schema::bot::{CreateApiKeySchema, StartBotSchema};
use crate::schema::AppState; 
use uuid::Uuid;
use std::sync::Arc;

use bot_investo::{brain::run_brain_bot, exchange::Exchange}; 
// use bot_investo::exchange::cryptocom::CryptoComExchange;
// use bot_investo::exchange::mock::MockExchange;
use bot_investo::exchange::mock::*;
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
    
    
    let exchange = Arc::new(MockExchange::new(
        key_info.key.clone(),
        key_info.secret.clone()
    ));


    let pool = state.pool.clone(); // On clone la connexion pour le thread du bot
    let handle = tokio::spawn(async move {
        run_brain_bot(body.user_id, pool, exchange, "BTC_USDT".to_string()).await;
    });

    // Enregistre dans la DashMap
    state.active_bots.insert(body.user_id, BotControl { handle });
    

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Le bot est lancé" ,
        "is_active": true,
    
    })))
}

pub async fn stop_bot(
    State(state): State<AppState>,
    Json(body): Json<StartBotSchema>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>{

    // On vérifie si le bot existait bien dans la map
    if let Some((_, bot_control)) = state.active_bots.remove(&body.user_id) {
        
        // 2. On tue le thread Tokio immédiatement
        bot_control.handle.abort();
        
        println!("✅ Bot stoppé proprement pour {}", body.user_id);

        Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Le bot a été arrêté",
            "is_active": false
        })))
    } else {
        // 3. Si l'ID n'était pas dans la map, le bot ne tournait pas
        println!("Aucun bot actif trouvé pour {}", body.user_id);
        
        let error_body = serde_json::json!({ 
            "status": "error",
            "message": "Le bot n'est pas en cours d'exécution",
            "is_active": false 
        }).to_string();

        Err((StatusCode::NOT_FOUND, error_body))
    }

}

pub async fn get_user_balance(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let solde_info = shared::repositories::api_bot_repo::give_solde(&state.pool, id)
        .await
        .map_err(|e| {
            eprintln!("Erreur BDD solde : {}", e);
            (StatusCode::NOT_FOUND, "Utilisateur ou solde introuvable".to_string())
        })?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "asset": "EUR", 
        "balance": solde_info.virtual_balance,
        "user_id": solde_info.id_user
    })))
    
    // // récupérer les clés API déchiffrées de l'utilisateur
    // let keys = api_key_repo::get_decrypted_keys(
    //     &state.pool, 
    //     id, 
    //     state.master_encryption_key.as_bytes()
    // ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // // On prend la première clé (ou on gère l'absence de clé)
    // let key_info = keys.first()
    //     .ok_or((StatusCode::NOT_FOUND, "Aucune clé API configurée pour cet utilisateur".to_string()))?;

    // // créer une instance temporaire de l'exchange pour la requête
    // let exchange = MockExchange::new(
    //     key_info.key.clone(), 
    //     key_info.secret.clone(),
    // );

    // // Pour le MVP, on se concentre sur l'USDT
    // match exchange.get_solde_current("USDT").await {
    //     Ok(balance) => {
    //         Ok(Json(serde_json::json!({
    //             "status": "success",
    //             "asset": "USDT",
    //             "balance": balance,
    //             "user_id": id
    //         })))
    //     },
    //     Err(e) => {
    //         eprintln!("Erreur lors de la récupération du solde : {}", e);
    //         Err((StatusCode::BAD_GATEWAY, "Impossible de joindre Crypto.com ou signature invalide".to_string()))
    //     }
    // }
}