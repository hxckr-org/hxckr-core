use crate::{
    app::auth::middleware::SessionInfo,
    service::database::{
        conn::DbPool,
        models::{Challenge, User},
    },
    shared::{
        errors::{CreateChallengeError, RepositoryError},
        primitives::{ChallengeMode, Difficulty, UserRole},
    },
};
use actix_web::{
    http::StatusCode, web, Error, HttpMessage, HttpRequest, HttpResponse, Result, Scope,
};
use log::error;
use serde_json::json;

pub fn init() -> Scope {
    web::scope("/challenge").route("", web::post().to(create_challenge))
}

#[derive(serde::Deserialize, serde::Serialize)]
struct NewChallenge {
    title: String,
    description: String,
    difficulty: Difficulty,
    mode: ChallengeMode,
    repo_url: String,
}

async fn create_challenge(
    req: HttpRequest,
    challenge: Result<web::Json<NewChallenge>, actix_web::Error>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;
    let user_id = match req.extensions().get::<SessionInfo>() {
        Some(session_info) => session_info.user_id,
        None => {
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).json(json!({
                "status": "error",
                "message": "Unauthorized."
            })));
        }
    };
    let user = match User::get_user(&mut conn, Some(&user_id), None, None, None) {
        Ok(user) => user,
        Err(e) => {
            error!("Error getting user: {}", e);
            return Err(Error::from(RepositoryError::BadRequest(
                "User not found".to_string(),
            )));
        }
    };
    if user.role != UserRole::Admin.to_str() {
        return Ok(HttpResponse::build(StatusCode::FORBIDDEN).json(json!({
            "status": "error",
            "message": "Forbidden. Only admins can create challenges."
        })));
    }

    let challenge = match challenge {
        Ok(challenge) => {
            if challenge.title.is_empty()
                || challenge.description.is_empty()
                || challenge.repo_url.is_empty()
            {
                return Err(Error::from(RepositoryError::BadRequest(
                    "Empty fields are not allowed".to_string(),
                )));
            }
            // Biased assumption: Given that the starter code repo url is a github url,
            // we can assume that it must start with https://github.com/
            if !challenge
                .repo_url
                .to_lowercase()
                .starts_with("https://github.com/")
            {
                return Err(Error::from(RepositoryError::BadRequest(
                    "Repo url must start with 'https://github.com/'".to_string(),
                )));
            }
            // Check if the repo url is valid
            let client = reqwest::Client::new();
            match client.get(&challenge.repo_url).send().await {
                Ok(res) => {
                    if !res.status().is_success() {
                        return Err(Error::from(RepositoryError::BadRequest(
                            "Repo url is not valid".to_string(),
                        )));
                    }
                }
                Err(_) => {
                    return Err(Error::from(RepositoryError::BadRequest(
                        "Repo url is not valid".to_string(),
                    )));
                }
            }
            challenge
        }
        Err(e) => return Err(Error::from(RepositoryError::BadRequest(e.to_string()))),
    };

    if Challenge::get_challenge(&mut conn, None, Some(&challenge.repo_url.to_lowercase())).is_ok() {
        return Err(Error::from(RepositoryError::BadRequest(
            "Challenge with this repo url already exists".to_string(),
        )));
    }
    let new_challenge = Challenge::new(
        &challenge.title.to_lowercase(),
        &challenge.description.to_lowercase(),
        &challenge.repo_url.to_lowercase(),
        &challenge.difficulty,
        &challenge.mode,
    );
    Ok(Challenge::create(&mut conn, new_challenge)
        .map(|challenge| {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Challenge created successfully",
                "challenge": {
                    "id": challenge.id,
                    "title": challenge.title,
                    "description": challenge.description,
                    "difficulty": challenge.difficulty,
                    "mode": challenge.mode,
                    "repo_url": challenge.repo_url,
                },
            }))
        })
        .map_err(|e| match e.downcast_ref() {
            Some(RepositoryError::FailedToCreateChallenge(CreateChallengeError(e))) => {
                RepositoryError::FailedToCreateChallenge(CreateChallengeError(
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        Box::new(e.to_string()),
                    ),
                ))
            }
            _ => RepositoryError::DatabaseError(e.to_string()),
        })?)
}
