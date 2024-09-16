use crate::{
    service::database::{
        conn::DbPool,
        models::{Session, User},
    },
    shared::{
        errors::{CreateUserError, RepositoryError},
        primitives::UserRole,
        utils::generate_session_token,
    },
};
use actix_web::{web, HttpResponse, Result, Scope};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    Connection, PgConnection,
};
use serde_json::json;
use log::error;
#[derive(serde::Deserialize)]
struct NewUser {
    username: String,
    github_username: String,
    email: String,
    profile_pic_url: String,
    role: String,
    provider: String,
}

pub fn init() -> Scope {
    web::scope("/sign-up").route("", web::post().to(signup))
}

async fn signup(
    user: Result<web::Json<NewUser>, actix_web::Error>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, RepositoryError> {
    let user = match user {
        Ok(user) => {
            if user.role.to_lowercase() != UserRole::Admin.to_str().to_lowercase()
                && user.role.to_lowercase() != UserRole::User.to_str().to_lowercase()
            {
                return Err(RepositoryError::BadRequest(String::from(
                    "Invalid role! Must be admin or user",
                )));
            }
            user
        }
        Err(e) => return Err(RepositoryError::BadRequest(e.to_string())),
    };

    let mut conn = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;
    let result = conn.transaction::<_, RepositoryError, _>(
        |conn: &mut PooledConnection<ConnectionManager<PgConnection>>| {
            let new_user = User::new(
                &user.username,
                &user.github_username,
                &user.email,
                &user.profile_pic_url,
                UserRole::from_str(&user.role.to_lowercase())
                    .map_err(|e| RepositoryError::BadRequest(e.to_string()))?,
            );
            let created_user = User::create(conn, new_user).map_err(|e| {
                match e.downcast_ref::<RepositoryError>() {
                    Some(RepositoryError::UserAlreadyExists) => RepositoryError::UserAlreadyExists,
                    Some(RepositoryError::BadRequest(e)) => {
                        RepositoryError::BadRequest(e.to_string())
                    }
                    _ => RepositoryError::FailedToCreateUser(CreateUserError(
                        diesel::result::Error::DatabaseError(
                            diesel::result::DatabaseErrorKind::Unknown,
                            Box::new(e.to_string()),
                        ),
                    )),
                }
            })?;
            let token = generate_session_token();
            let user_id = created_user.id;

            let session = Session::new(&user_id, &token, &user.provider.to_lowercase());
            Session::create(conn, session)
                .map_err(|e| RepositoryError::BadRequest(e.to_string()))?;

            Ok(HttpResponse::Ok().json(json!({
                "user_id": user_id,
                "session_token": token
            })))
        },
    );

    match result {
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}
