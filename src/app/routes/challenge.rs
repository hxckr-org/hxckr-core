use crate::{
    app::auth::middleware::SessionInfo,
    service::database::{
        conn::DbPool,
        models::{Challenge, Repository, User},
    },
    shared::{
        errors::{CreateChallengeError, GetChallengeError, GetRepositoryError, RepositoryError},
        primitives::{ChallengeMode, Difficulty, Period, UserRole},
    },
};
use actix_web::{
    http::StatusCode, web, Error, HttpMessage, HttpRequest, HttpResponse, Result, Scope,
};
use log::error;
use serde_json::json;
use uuid::Uuid;

pub fn init() -> Scope {
    web::scope("/challenge")
        .route("", web::get().to(get_challenge))
        .route("", web::post().to(create_challenge))
        .route("/{id}", web::put().to(update_challenge))
        .route("/attempts", web::get().to(get_all_attempts))
        .route("", web::delete().to(delete_challenge))
}

#[derive(serde::Deserialize, serde::Serialize)]
struct NewChallenge {
    title: String,
    description: String,
    difficulty: Difficulty,
    module_count: i32,
    mode: ChallengeMode,
    repo_urls: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct GetChallengeQuery {
    id: Option<Uuid>,
    title: Option<String>,
    difficulty: Option<Difficulty>,
    mode: Option<ChallengeMode>,
}

#[derive(serde::Deserialize)]
struct GetAllAttemptsQuery {
    period: Option<Period>,
    challenge_id: Option<Uuid>,
}

#[derive(serde::Deserialize)]
struct DeleteChallengeQuery {
    id: Uuid,
}

#[derive(serde::Deserialize)]
struct UpdateChallengeRequest {
    title: Option<String>,
    description: Option<String>,
    difficulty: Option<Difficulty>,
    module_count: Option<i32>,
    mode: Option<ChallengeMode>,
    repo_urls: Option<serde_json::Value>,
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
            if challenge.title.is_empty() || challenge.description.is_empty() {
                return Err(Error::from(RepositoryError::BadRequest(
                    "Empty fields are not allowed".to_string(),
                )));
            }

            let repo_urls = challenge.repo_urls.as_object().ok_or_else(|| {
                RepositoryError::BadRequest("repo_urls must be a valid JSON object".to_string())
            })?;

            if repo_urls.is_empty() {
                return Err(Error::from(RepositoryError::BadRequest(
                    "repo_urls cannot be empty".to_string(),
                )));
            }

            // Validate all URLs are accessible
            let client = reqwest::Client::new();
            for (lang, url) in repo_urls {
                let url_str = url.as_str().ok_or_else(|| {
                    RepositoryError::BadRequest(format!("URL for {} must be a string", lang))
                })?;

                match client.get(url_str).send().await {
                    Ok(res) if !res.status().is_success() => {
                        return Err(Error::from(RepositoryError::BadRequest(
                            format!("Repository URL for {} is not accessible", lang)
                        )));
                    }
                    Err(_) => {
                        return Err(Error::from(RepositoryError::BadRequest(
                            format!("Failed to validate repository URL for {}", lang)
                        )));
                    }
                    _ => {}
                }
            }

            challenge
        }
        Err(e) => return Err(Error::from(RepositoryError::BadRequest(e.to_string()))),
    };

    if Challenge::get_challenge(
        &mut conn,
        None,
        Some(&challenge.title.to_lowercase()),
        None,
        None,
    ).is_ok() {
        return Err(Error::from(RepositoryError::BadRequest(
            "Challenge with this title already exists".to_string(),
        )));
    }
    let new_challenge = Challenge::new(
        &challenge.title.to_lowercase(),
        &challenge.description.to_lowercase(),
        &challenge.repo_urls,
        &challenge.module_count,
        &challenge.difficulty.to_str(),
        &challenge.mode.to_str(),
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
                    "module_count": challenge.module_count,
                    "mode": challenge.mode,
                    "repo_urls": challenge.repo_urls,
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

async fn get_challenge(
    query: web::Query<GetChallengeQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;

    // If no parameters are provided, return all challenges
    if query.id.is_none()
        && query.title.is_none()
        && query.difficulty.is_none()
        && query.mode.is_none()
    {
        return Ok(Challenge::get_all_challenges(&mut conn)
            .map(|challenges| HttpResponse::Ok().json(challenges))
            .map_err(|e| match e.downcast_ref() {
                Some(RepositoryError::FailedToGetChallenge(GetChallengeError(e))) => {
                    RepositoryError::FailedToGetChallenge(GetChallengeError(
                        diesel::result::Error::DatabaseError(
                            diesel::result::DatabaseErrorKind::Unknown,
                            Box::new(e.to_string()),
                        ),
                    ))
                }
                _ => RepositoryError::DatabaseError(e.to_string()).into(),
            })?);
    }

    let param_count = query.id.is_some() as u8
        + query.title.is_some() as u8
        + query.difficulty.is_some() as u8
        + query.mode.is_some() as u8;
    if param_count != 1 {
        return Err(Error::from(RepositoryError::BadRequest(
            "Only one of the parameters must be passed".to_string(),
        )));
    }

    let challenge = Challenge::get_challenge(
        &mut conn,
        query.id.as_ref(),
        query.title.as_deref(),
        query.difficulty.as_ref(),
        query.mode.as_ref(),
    )
    .map(|challenge| HttpResponse::Ok().json(challenge))
    .map_err(|e| match e.downcast_ref() {
        Some(RepositoryError::FailedToGetChallenge(GetChallengeError(e))) => {
            RepositoryError::FailedToGetChallenge(GetChallengeError(
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::Unknown,
                    Box::new(e.to_string()),
                ),
            ))
        }
        _ => RepositoryError::DatabaseError(e.to_string()).into(),
    })?;
    Ok(challenge)
}

async fn get_all_attempts(
    query: web::Query<GetAllAttemptsQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;

    let period = match query.period.as_ref() {
        Some(period) => period,
        None => &Period::AllTime,
    };
    let challenge_id = match query.challenge_id.as_ref() {
        Some(challenge_id) => challenge_id,
        None => {
            return Err(Error::from(RepositoryError::BadRequest(
                "Challenge id is required".to_string(),
            )))
        }
    };

    let attempts = Repository::get_all_repos(&mut conn, period, challenge_id)
        .map(|attempts| HttpResponse::Ok().json(attempts))
        .map_err(|e| match e.downcast_ref() {
            Some(RepositoryError::FailedToGetRepository(GetRepositoryError(e))) => {
                RepositoryError::FailedToGetRepository(GetRepositoryError(
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        Box::new(e.to_string()),
                    ),
                ))
            }
            _ => RepositoryError::DatabaseError(e.to_string()).into(),
        })?;
    Ok(attempts)
}

async fn delete_challenge(
    req: HttpRequest,
    query: web::Query<DeleteChallengeQuery>,
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
            "message": "Forbidden. Only admins can delete challenges."
        })));
    }

    let challenge_id = query.id;

    // Check if challenge exists
    match Challenge::get_challenge(&mut conn, Some(&challenge_id), None, None, None) {
        Ok(_) => (),
        Err(_) => {
            return Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(json!({
                "status": "error",
                "message": "Challenge not found"
            })));
        }
    }

    match Challenge::delete(&mut conn, &challenge_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Challenge deleted successfully"
        }))),
        Err(e) => {
            error!("Error deleting challenge: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to delete challenge"
            })))
        }
    }
}

async fn update_challenge(
    req: HttpRequest,
    id: web::Path<Uuid>,
    body: web::Json<UpdateChallengeRequest>,
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
            "message": "Forbidden. Only admins can update challenges."
        })));
    }

    // Validate repo_urls if provided
    if let Some(ref repo_urls) = body.repo_urls {
        let urls = repo_urls.as_object().ok_or_else(|| {
            RepositoryError::BadRequest("repo_urls must be a valid JSON object".to_string())
        })?;

        if urls.is_empty() {
            return Err(Error::from(RepositoryError::BadRequest(
                "repo_urls cannot be empty".to_string(),
            )));
        }

        // Validate all URLs
        for (lang, url) in urls {
            let url_str = url.as_str().ok_or_else(|| {
                RepositoryError::BadRequest(format!("URL for {} must be a string", lang))
            })?;

            if !url_str.to_lowercase().starts_with("https://github.com/") {
                return Err(Error::from(RepositoryError::BadRequest(format!(
                    "URL for {} must start with 'https://github.com/'",
                    lang
                ))));
            }
        }
    }

    let challenge = Challenge::update(
        &mut conn,
        &id,
        body.title.as_deref(),
        body.description.as_deref(),
        body.repo_urls.as_ref(),
        body.module_count.as_ref(),
        body.difficulty.as_ref(),
        body.mode.as_ref(),
    )
    .map_err(|e| {
        error!("Error updating challenge: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Challenge updated successfully",
        "challenge": challenge
    })))
}
