use actix_web::{web, Responder, Result};
use actix_web::get;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "transaction")]
pub enum Transaction {
    Expense,
    Income
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct FoundTransaction {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub label: Option<Transaction>,
    pub amount: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

#[get("/transactions/by-id/{id}")]
pub async fn get_transactions(
    state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>
) -> Result<impl Responder> {
    let found_transaction = sqlx::query_as(
        r#"SELECT * FROM transactions WHERE id = $1"#,
    )
        .bind(&path)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Ok(web::Json(path.to_string()))
}
