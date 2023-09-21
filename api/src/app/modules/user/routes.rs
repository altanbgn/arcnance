use actix_web::web;

use crate::app::modules::user::services;

pub fn load(cfg: &mut web::ServiceConfig) {
    cfg.service(services::get_users);
}
