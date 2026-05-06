use uuid::Uuid;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub fn verify_owner(claims_sub: &str, target_id: Uuid) -> Result<(), (StatusCode, Json<Value>)> {
    if claims_sub != target_id.to_string() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "status": "error",
                "message": "Accès refusé : vous n'êtes pas le propriétaire de ce bot"
            })),
        ));
    }
    Ok(())
}