use crate::service::database::{conn::DbPool, models::Leaderboard};
use crate::shared::errors::RepositoryError;
use actix_web::{web, HttpResponse, Scope};
use log::error;

pub fn init() -> Scope {
    web::scope("/leaderboard").route("", web::get().to(get_leaderboard))
}

async fn get_leaderboard(pool: web::Data<DbPool>) -> Result<HttpResponse, RepositoryError> {
    let mut conn = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;

    let leaderboard = Leaderboard::get_leaderboard(&mut conn, None)
        .map_err(|e| {
            error!("Error getting leaderboard: {}", e);
            RepositoryError::DatabaseError(e.to_string())
        })?;

    Ok(HttpResponse::Ok().json(leaderboard))
}
