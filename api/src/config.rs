use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub secret_key: String,
}

pub async fn load_config() -> Config {
    dotenv().ok();

    let host = std::env::var("HOST").expect("HOST must be set");
    let port = std::env::var("PORT").expect("PORT must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    Config {
        host,
        port,
        database_url,
        secret_key,
    }
}
