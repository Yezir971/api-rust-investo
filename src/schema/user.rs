use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};


/// Schema for creating or updating a user
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct AuthUserSchema{
    pub email: String, 
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateUserModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub virtual_balance: f64,

}


#[derive(Serialize)]
pub struct UserModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub virtual_balance: f64,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub virtual_balance : f64,
}

