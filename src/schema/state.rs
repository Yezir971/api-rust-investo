use dashmap::DashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;
use uuid::Uuid;

// On définit une petite structure pour garder la trace du bot
pub struct BotControl {
    pub handle: JoinHandle<()>,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub jwt_secret: String,
    pub master_encryption_key: String,
    // note les bots actifs ici 
    pub active_bots: Arc<DashMap<Uuid, BotControl>>,
}