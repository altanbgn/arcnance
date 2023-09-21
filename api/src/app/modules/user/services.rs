use actix_web::{Responder, HttpRequest, web};
use actix_web::get;

use crate::AppState;

#[get("/user")]
pub async fn get_users(_req: HttpRequest, _state: web::Data<AppState>) -> impl Responder {
    format!("pog")
}
