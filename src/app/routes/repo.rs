use crate::{
    app::auth::middleware::SessionInfo,
    service::database::{
        conn::DbPool,
        models::{Challenge, Repository, User},
    },
    shared::errors::{CreateRepositoryError, RepositoryError},
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Scope};
use log::error;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRepoResponse {
    repo_name: String,
    repo_url: String,
}

#[derive(Deserialize)]
pub struct CreateRepoRequest {
    repo_url: String,
}

pub fn init() -> Scope {
    web::scope("/repo").route("", web::post().to(create_repo))
}

async fn create_repo(
    req: HttpRequest,
    body: Result<web::Json<CreateRepoRequest>, actix_web::Error>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, RepositoryError> {
    let repo_url = match body {
        Ok(body) => body.repo_url.clone(),
        Err(e) => {
            error!("Error getting repository url: {}", e);
            return Err(RepositoryError::BadRequest(
                "Repository url is required".to_string(),
            ));
        }
    };
    if repo_url.is_empty() {
        return Err(RepositoryError::BadRequest(
            "Repository url is required".to_string(),
        ));
    }

    let client = reqwest::Client::new();
    let git_service_url = std::env::var("GIT_SERVICE_URL").expect("GIT_SERVICE_URL not set");
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user_id = match req.extensions().get::<SessionInfo>() {
        Some(session_info) => session_info.user_id,
        None => {
            return Err(RepositoryError::BadRequest(
                "User not authenticated".to_string(),
            ));
        }
    };
    let user = match User::get_user(&mut conn, Some(&user_id), None, None, None) {
        Ok(user) => user,
        Err(e) => {
            error!("Error getting user: {}", e);
            return Err(RepositoryError::BadRequest("User not found".to_string()));
        }
    };

    let challenge = match Challenge::get_challenge(&mut conn, None, Some(&repo_url)) {
        Ok(challenge) => challenge,
        Err(e) => {
            error!("Error getting challenge for repository url: {}", e);
            return Err(RepositoryError::BadRequest(format!(
                "repository url starter code {} not found",
                &repo_url
            )));
        }
    };

    let repo_name = repo_url
        .split("/")
        .last()
        .ok_or(RepositoryError::BadRequest(
            "Error getting repository name from url".to_string(),
        ))?;
    let request_body = json!({
        "repo_name": format!("{}__{}", user.username, repo_name),
        "repo_url": &repo_url,
    });
    let response = client
        .post(format!("{}/create_repo", git_service_url))
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()
        .await
        .map_err(|e| {
            error!("Error creating repository in git service: {:#?}", e);
            RepositoryError::BadRequest("Error creating repository".to_string())
        })?;
    let create_repo_response = match response.status() {
        StatusCode::OK => response.json::<CreateRepoResponse>().await.map_err(|e| {
            error!("Error in git service response: {:#?}", e);
            RepositoryError::BadRequest("Error decoding repository response".to_string())
        })?,
        StatusCode::CONFLICT => {
            return Err(RepositoryError::RepositoryAlreadyExists);
        }
        _ => {
            return Err(RepositoryError::BadRequest(
                "Error creating repository".to_string(),
            ));
        }
    };

    let repo = Repository::new(&user_id, &challenge.id, &create_repo_response.repo_url);
    if let Err(e) = Repository::create_repo(&mut conn, repo) {
        error!("Error creating repository in database: {:#?}", e);
        return Err(RepositoryError::FailedToCreateRepository(
            CreateRepositoryError(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            )),
        ));
    }

    Ok(HttpResponse::Ok().json(json!({
        "repo_name": &create_repo_response.repo_name,
        "repo_url": &create_repo_response.repo_url,
    })))
}
