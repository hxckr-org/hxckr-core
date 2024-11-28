use crate::{
    service::database::{
        conn::DbPool,
        models::{Progress, Repository},
    },
    shared::errors::RepositoryError,
};
use actix_web::{web, HttpResponse, Result, Scope};
use log::error;

pub fn init() -> Scope {
    web::scope("/progress").route("", web::get().to(get_progress))
}

#[derive(serde::Deserialize)]
struct ProgressQuery {
    repo_url: String,
}

async fn get_progress(
    query: Result<web::Query<ProgressQuery>, actix_web::Error>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, RepositoryError> {
    let query = match query {
        Ok(query) => query,
        Err(_) => {
            return Err(RepositoryError::BadRequest(String::from(
                "Invalid query parameters. repo_url is required",
            )));
        }
    };

    let mut conn = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;

    let repo =
        Repository::get_repo(&mut conn, None, None, None, Some(&query.repo_url)).map_err(|e| {
            error!("Error getting repository: {}", e);
            RepositoryError::NotFound(e.to_string())
        })?;

    let repo = repo.first().ok_or_else(|| {
        RepositoryError::NotFound(format!("Repository not found with URL: {}", query.repo_url))
    })?;

    let progress = Progress::get_progress(&mut conn, None, None, None, Some(&repo.id))
        .map_err(|e| {
            error!("Error getting progress: {}", e);
            RepositoryError::DatabaseError(e.to_string())
        })?;
    Ok(HttpResponse::Ok().json(progress))
}
