use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn conn() -> PgPool {
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool!");

    pool
}
