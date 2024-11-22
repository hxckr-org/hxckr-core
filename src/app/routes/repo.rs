use crate::{
    app::auth::middleware::SessionInfo,
    service::database::{
        conn::DbPool,
        models::{Challenge, Leaderboard, Progress, Repository, User},
    },
    shared::{
        errors::{
            CreateProgressError, CreateRepositoryError, RepositoryError, UpdateLeaderboardError,
        },
        primitives::{PaginationParams, Status},
    },
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Scope};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    Connection, PgConnection,
};
use log::error;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRepoResponse {
    repo_name: String,
    repo_url: String,
}

#[derive(Deserialize)]
pub struct CreateRepoRequest {
    repo_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetRepoQuery {
    id: Option<Uuid>,
    repo_url: Option<String>,
    soft_serve_url: Option<String>,
    status: Option<Status>,
    per_page: Option<i64>,
    page: Option<i64>,
}

pub fn init() -> Scope {
    web::scope("/repo")
        .route("", web::post().to(create_repo))
        .route("", web::get().to(get_repo))
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
    let git_service_url = std::env::var("GIT_SERVICE_URL").map_err(|_| {
        error!("GIT_SERVICE_URL environment variable not set");
        RepositoryError::ServerConfigurationError(
            "GIT_SERVICE_URL environment variable not set".to_string(),
        )
    })?;
    let mut conn = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;

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

    let challenge = match Challenge::get_challenge(&mut conn, None, Some(&repo_url), None, None) {
        Ok(challenge) => challenge,
        Err(e) => {
            error!("Error getting challenge for repository url: {}", e);
            return Err(RepositoryError::BadRequest(format!(
                "repository url starter code {} not found",
                &repo_url
            )));
        }
    };

    let leaderboard = match Leaderboard::get_leaderboard(&mut conn, Some(&user_id)) {
        Ok(leaderboard) => leaderboard[0].clone(),
        Err(e) => {
            error!("Error getting leaderboard: {}", e);
            return Err(RepositoryError::DatabaseError(e.to_string()));
        }
    };

    let repo_name = repo_url
        .rsplit("/")
        .next()
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .ok_or_else(|| {
            error!("Invalid repository URL format: {}", repo_url);
            RepositoryError::BadRequest("Invalid repository URL format".to_string())
        })?;
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

    // Parse the soft_serve_url from the repo_url by removing the user token in the url
    // This is necessary because the user token is not appended to the repo_url in the
    // response from the git service.
    // So both the repo_url with the token (provided to client for cloning) and and
    // the url without the token (for matching with queue events) are stored in the database
    let (scheme, rest) = create_repo_response
        .repo_url
        .split_once("://")
        .ok_or_else(|| {
            error!(
                "Error parsing repository url: {}",
                create_repo_response.repo_url
            );
            RepositoryError::BadRequest("Error parsing repository url".to_string())
        })?;
    let soft_serve_url = rest
        .split_once("@")
        .map(|(_, path)| format!("{}://{}", scheme, path))
        .ok_or_else(|| {
            error!(
                "Error parsing repository url: {}",
                create_repo_response.repo_url
            );
            RepositoryError::BadRequest("Error parsing repository url".to_string())
        })?;
    let repo = Repository::new(
        &user_id,
        &challenge.id,
        &create_repo_response.repo_url,
        &soft_serve_url,
    );

    // assign progress detail of 1 for new repositories
    // this is used to track the progress of the user
    // in the challenge
    let new_progress = Progress::new(
        &user_id,
        &challenge.id,
        Status::NotStarted,
        Some(json!({
            "current_step": 1,
        })),
    );

    // update leaderboard with expected total score
    let expected_total_score = leaderboard.score + challenge.module_count;

    let result = conn.transaction::<_, RepositoryError, _>(
        |conn: &mut PooledConnection<ConnectionManager<PgConnection>>| {
            let new_repo = match Repository::create_repo(conn, repo) {
                Ok(repo) => repo,
                Err(e) => {
                    error!("Error creating repository in database: {:#?}", e);
                    return Err(RepositoryError::FailedToCreateRepository(
                        CreateRepositoryError(diesel::result::Error::DatabaseError(
                            diesel::result::DatabaseErrorKind::Unknown,
                            Box::new(e.to_string()),
                        )),
                    ));
                }
            };

            if let Err(e) = Progress::create_progress(conn, new_progress) {
                error!("Error creating progress in database: {:#?}", e);
                return Err(RepositoryError::FailedToCreateProgress(
                    CreateProgressError(diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        Box::new(e.to_string()),
                    )),
                ));
            }

            if let Err(e) =
                Leaderboard::update(conn, &user_id, None, Some(expected_total_score), None)
            {
                error!("Error updating leaderboard in database: {:#?}", e);
                return Err(RepositoryError::FailedToUpdateLeaderboard(
                    UpdateLeaderboardError(diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        Box::new(e.to_string()),
                    )),
                ));
            }

            Ok(HttpResponse::Ok().json(json!({
                "repo_name": &create_repo_response.repo_name,
                "repo_url": &create_repo_response.repo_url,
                "id": &new_repo.id,
            })))
        },
    );

    match result {
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}

async fn get_repo(
    req: HttpRequest,
    query: web::Query<GetRepoQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, RepositoryError> {
    if query.repo_url.is_some()
        && query.soft_serve_url.is_some()
        && query.status.is_some()
        && query.id.is_some()
    {
        return Err(RepositoryError::BadRequest(
            "Multiple parameters are not allowed. Please provide only one parameter.".to_string(),
        ));
    }

    let mut conn = pool.get().map_err(|e| {
        error!("Error getting db connection from pool: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;

    let user_id = match req.extensions().get::<SessionInfo>() {
        Some(session_info) => session_info.user_id,
        None => {
            return Err(RepositoryError::BadRequest(
                "User not authenticated".to_string(),
            ));
        }
    };

    if let Some(id) = query.id {
        let repository = Repository::get_repo_by_id(&mut conn, &id, &user_id).map_err(|e| {
            error!("Error getting repository: {}", e);
            RepositoryError::DatabaseError(e.to_string())
        })?;
        return Ok(HttpResponse::Ok().json(repository));
    }

    let pagination = PaginationParams {
        page: query.page,
        per_page: query.per_page,
    };

    let repositories = Repository::get_repo_with_relations(
        &mut conn,
        &user_id,
        query.repo_url.as_deref(),
        query.soft_serve_url.as_deref(),
        query.status.as_ref(),
        &pagination,
    )
    .map_err(|e| {
        error!("Error fetching repositories: {}", e);
        RepositoryError::DatabaseError(e.to_string())
    })?;

    Ok(HttpResponse::Ok().json(repositories))
}
