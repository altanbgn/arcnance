mod app;
mod db;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;

use crate::app::user;

#[derive(Debug)]
pub struct AppState {
    pool: sqlx::PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = std::env::var("HOST").expect("HOST must be set");
    let port = std::env::var("PORT").expect("PORT must be set");

    let pool = db::conn().await;
    db::load_tables(&pool).await;

    println!("Starting server at http://{}:{}", host, port);

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
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .wrap(cors)
            .configure(user::routes::load)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
