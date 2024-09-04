use actix_web::web;

pub mod errors;
pub mod users;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(users::init());
}
