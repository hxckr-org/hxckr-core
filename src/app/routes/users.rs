use crate::{
    service::database::models::User,
    shared::{
        errors::{CreateUserError, GetUserError, RepositoryError},
        primitives::UserRole,
    },
};
use actix_web::{web, HttpResponse, Result, Scope};
use anyhow::Error as AnyhowError;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

#[derive(serde::Deserialize)]
struct UserQuery {
    username: Option<String>,
    email: Option<String>,
    github_username: Option<String>,
}

#[derive(serde::Deserialize)]
struct NewUser {
    username: String,
    github_username: String,
    email: String,
    profile_pic_url: String,
    role: String,
}

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init() -> Scope {
    web::scope("/users")
        .route("", web::get().to(get_user))
        .route("", web::post().to(create_user))
}

async fn get_user(
    query: web::Query<UserQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, RepositoryError> {
    if query.username.is_none() && query.email.is_none() && query.github_username.is_none() {
        return Err(RepositoryError::BadRequest(
            "No query parameters provided".to_string(),
        ));
    }

    let param_count = query.username.is_some() as u8
        + query.email.is_some() as u8
        + query.github_username.is_some() as u8;
    if param_count != 1 {
        return Err(RepositoryError::BadRequest(
            "Only one of the parameters must be passed".to_string(),
        ));
    }

    let mut connection = pool.get().expect("couldn't get db connection from pool");

    User::get_user(
        &mut connection,
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

async fn create_user(
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

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let new_user = User::new(
        &user.username,
        &user.github_username,
        &user.email,
        &user.profile_pic_url,
        UserRole::from_str(&user.role.to_lowercase())
            .map_err(|e| RepositoryError::BadRequest(e.to_string()))?,
    );

    User::create(&mut conn, new_user)
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e: AnyhowError| match e.downcast_ref::<RepositoryError>() {
            Some(RepositoryError::UserAlreadyExists) => RepositoryError::UserAlreadyExists,
            _ => RepositoryError::FailedToCreateUser(CreateUserError(
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::Unknown,
                    Box::new(e.to_string()),
                ),
            )),
        })
}
