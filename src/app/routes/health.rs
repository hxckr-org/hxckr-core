use actix_web::{web, HttpResponse, Responder, Scope};
use serde_json::json;

use crate::service::database::conn::DbPool;

pub fn init() -> Scope {
    web::scope("/health").route("", web::get().to(health_check))
}

async fn health_check(pool: web::Data<DbPool>) -> impl Responder {
    match pool.get() {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Server and database are healthy"
        })),
        Err(_) => HttpResponse::ServiceUnavailable().json("Server or Database connection failed"),
    }
}
