use serde::{Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct UserModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}