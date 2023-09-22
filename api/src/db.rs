use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn conn(database_url: &String) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool!");

    pool
}
