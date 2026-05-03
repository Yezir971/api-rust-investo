// schema.rs 

use serde::{Deserialize, Serialize};

/// Schema for creating or updating a user
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}
