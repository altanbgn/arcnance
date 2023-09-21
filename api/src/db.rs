use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn conn() -> PgPool {
    let db_url = std::env::var("DB_URL").expect("DB_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool!");

    pool
}

pub async fn load_tables(pool: &sqlx::Pool<sqlx::Postgres>) {

    let _ = sqlx::query(
        r#"
            CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
        "#,
    )
        .execute(pool)
        .await;

    let _ = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS "users" (
                id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
                first_name TEXT,
                last_name TEXT,
                user_name TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL,
                password TEXT
            );
        "#,
    )
        .execute(pool)
        .await;
}
