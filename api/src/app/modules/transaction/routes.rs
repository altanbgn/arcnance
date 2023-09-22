use actix_web::web;

use crate::app::modules::transaction::services;

pub fn load(cfg: &mut web::ServiceConfig) {
    cfg.service(services::get_transaction);
    // cfg.service(services::get_user_transactions);
    // cfg.service(services::update_transaction);
    // cfg.service(services::delete_transaction);
}
