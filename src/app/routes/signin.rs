use crate::{
    service::database::{
        conn::DbPool,
        models::{Session, User},
    },
    shared::{
        errors::{GetUserError, RepositoryError},
        utils::generate_session_token,
    },
};
use actix_web::{web, HttpResponse, Result, Scope};
use serde_json::json;
use log::error;

#[derive(serde::Deserialize)]
struct UserBody {
    username: Option<String>,
    github_username: Option<String>,
    email: Option<String>,
    provider: String,
}

pub fn init() -> Scope {
    web::scope("/sign-in").route("", web::post().to(signin))
}

async fn signin(
    user: Result<web::Json<UserBody>, actix_web::Error>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, RepositoryError> {
    let user = match user {
        Ok(user) => {
            let param_count = user.username.is_some() as u8
                + user.email.is_some() as u8
                + user.github_username.is_some() as u8;
            if param_count != 1 {
                return Err(RepositoryError::BadRequest(
                    "Only one of the parameters must be passed".to_string(),
                ));
            }
            user
        }
        Err(e) => return Err(RepositoryError::BadRequest(e.to_string())),
    };

    let mut conn = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;
    let db_user = User::get_user(
        &mut conn,
        None,
        user.username.as_deref(),
        user.email.as_deref(),
        user.github_username.as_deref(),
    )
    .map_err(|e| match e.downcast_ref() {
        Some(RepositoryError::UserNotFound) => RepositoryError::UserNotFound,
        _ => RepositoryError::FailedToGetUser(GetUserError(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(e.to_string()),
        ))),
    })?;

    let del_existing_session = Session::get_by_userid(&mut conn, &db_user.id).and_then(|session| {
        if let Some(existing_session) = session {
            Session::delete(&mut conn, existing_session.token)
        } else {
            Ok(0)
        }
    });
    if let Err(err) = del_existing_session {
        return Err(RepositoryError::DatabaseError(format!(
            "Failed to delete existing session: {}",
            err
        )));
    };

    let token = generate_session_token();
    let user_id = db_user.id;

    let new_session = Session::new(&user_id, &token, &user.provider.to_lowercase());
    Session::create(&mut conn, new_session)
        .map_err(|e| RepositoryError::BadRequest(e.to_string()))?;

    Ok(HttpResponse::Ok().json(json!({
        "user_id": user_id,
        "session_token": token
    })))
}
