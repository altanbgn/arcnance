use actix_web::web;

use crate::app::modules::auth::services;

pub fn load(cfg: &mut web::ServiceConfig) {
    cfg.service(services::login);
    cfg.service(services::register);
}
