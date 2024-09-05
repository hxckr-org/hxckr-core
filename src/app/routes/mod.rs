use actix_web::web;

pub mod errors;
pub mod users;
pub mod signup;
pub mod signin;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(users::init());
    cfg.service(signup::init());
    cfg.service(signin::init());
}
