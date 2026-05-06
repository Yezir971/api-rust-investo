use serde::Deserialize;
use uuid::Uuid;
use tokio::task::JoinHandle;

#[derive(Deserialize)]
pub struct CreateApiKeySchema{
    pub user_id: Uuid,
    pub exchange: String, 
    pub api_key: String,
    pub api_secret: String,
}
pub struct BotControl {
    pub handle: JoinHandle<()>,
}