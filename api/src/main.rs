mod app;
mod config;
mod db;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use sqlx::{Pool, Postgres};

use crate::app::modules;

#[derive(Debug)]
pub struct AppState {
    pool: Pool<Postgres>,
    config: config::Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::load_config().await;
    let pool = db::conn(&config.database_url).await;

    let host = config.host.clone();
    let port = config.port.clone();

    println!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ]);

        App::new()
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
                config: config.clone(),
            }))
            .wrap(cors)
            .configure(modules::user::routes::load)
            .configure(modules::auth::routes::load)
            .configure(modules::transaction::routes::load)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
