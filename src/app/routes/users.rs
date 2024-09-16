use crate::{
    service::database::{conn::DbPool, models::User},
    shared::errors::{GetUserError, RepositoryError},
};
use actix_web::{web, HttpResponse, Result, Scope};
use anyhow::Error as AnyhowError;
use log::error;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct UserQuery {
    username: Option<String>,
    email: Option<String>,
    github_username: Option<String>,
    id: Option<Uuid>,
}

pub fn init() -> Scope {
    web::scope("/users").route("", web::get().to(get_user))
}

async fn get_user(
    query: web::Query<UserQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, RepositoryError> {
    if query.username.is_none()
        && query.email.is_none()
        && query.github_username.is_none()
        && query.id.is_none()
    {
        return Err(RepositoryError::BadRequest(
            "No query parameters provided".to_string(),
        ));
    }

    let param_count = query.username.is_some() as u8
        + query.email.is_some() as u8
        + query.github_username.is_some() as u8
        + query.id.is_some() as u8;
    if param_count != 1 {
        return Err(RepositoryError::BadRequest(
            "Only one of the parameters must be passed".to_string(),
        ));
    }

    let mut connection = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;

    User::get_user(
        &mut connection,
        query.id.as_ref(),
        query.username.as_deref(),
        query.email.as_deref(),
        query.github_username.as_deref(),
    )
    .map(|response| HttpResponse::Ok().json(response))
    .map_err(|e: AnyhowError| match e.downcast_ref::<RepositoryError>() {
        Some(RepositoryError::UserNotFound) => RepositoryError::UserNotFound,
        _ => RepositoryError::FailedToGetUser(GetUserError(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(e.to_string()),
        ))),
    })
}
