use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::{FromRow, Type};

#[derive(Clone, Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "role")]
pub enum Role {
    ADMIN,
    CLIENT
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: String,
    pub email: String,
    pub role: Option<Role>,
    pub transactions: Vec<Uuid>,
    pub password: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}
