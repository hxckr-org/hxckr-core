use crate::schema::repositories::table as repositories;
use crate::service::database::models::{
    AttemptInfo, ChallengeInfo, ProgressInfo, Repository, RepositoryWithRelations,
};
use crate::shared::errors::{
    CreateRepositoryError, GetRepositoryError,
    RepositoryError::{FailedToCreateRepository, FailedToGetRepository},
};
use crate::shared::primitives::{PaginatedResponse, PaginationParams, Period, Status};
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl Repository {
    pub fn new(
        user_id: &Uuid,
        challenge_id: &Uuid,
        repo_url: &str,
        soft_serve_url: &str,
        language: &str,
    ) -> Self {
        Repository {
            id: Uuid::new_v4(),
            user_id: user_id.to_owned(),
            challenge_id: challenge_id.to_owned(),
            repo_url: repo_url.to_string(),
            soft_serve_url: soft_serve_url.to_string(),
            language: language.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create_repo(connection: &mut PgConnection, repo: Repository) -> Result<Repository> {
        let repo = repo
            .insert_into(repositories)
            .returning(Repository::as_returning())
            .get_result(connection)
            .map_err(|e| {
                error!("Error creating repository: {}", e);
                FailedToCreateRepository(CreateRepositoryError(e))
            })?;

        Ok(repo)
    }

    pub fn get_repo(
        connection: &mut PgConnection,
        id: Option<&Uuid>,
        user_id: Option<&Uuid>,
        repo_url: Option<&str>,
        soft_serve_url: Option<&str>,
    ) -> Result<Vec<Repository>> {
        use crate::schema::repositories::dsl::{
            repo_url as repo_url_col, soft_serve_url as soft_serve_url_col, user_id as user_id_col,
        };

        let param_count = id.is_some() as u8
            + user_id.is_some() as u8
            + repo_url.is_some() as u8
            + soft_serve_url.is_some() as u8;

        if param_count != 1 {
            return Err(anyhow::anyhow!(
                "Multiple parameters are not allowed. Please provide only one parameter."
            ));
        }

        match (id, user_id, repo_url, soft_serve_url) {
            (Some(id), None, None, None) => {
                let repo = repositories
                    .find(id)
                    .select(Repository::as_select())
                    .first::<Repository>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting repository: {}", e);
                        FailedToGetRepository(GetRepositoryError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Repository not found"))?;
                Ok(vec![repo])
            }
            (None, Some(user_id), None, None) => {
                let repo = repositories
                    .filter(user_id_col.eq(user_id))
                    .select(Repository::as_select())
                    .load::<Repository>(connection)
                    .map_err(|e| {
                        error!("Error getting repository: {}", e);
                        FailedToGetRepository(GetRepositoryError(e))
                    })?;
                if repo.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Repository for user id {} not found",
                        user_id
                    ));
                }
                Ok(repo)
            }
            (None, None, Some(repo_url), None) => {
                let repo = repositories
                    .filter(repo_url_col.eq(&repo_url))
                    .select(Repository::as_select())
                    .load::<Repository>(connection)
                    .map_err(|e| {
                        error!("Error getting repository: {}", e);
                        FailedToGetRepository(GetRepositoryError(e))
                    })?;
                if repo.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Repository for repo url {} not found",
                        &repo_url
                    ));
                }
                Ok(repo)
            }
            (None, None, None, Some(soft_serve_url)) => {
                let repo = repositories
                    .filter(soft_serve_url_col.eq(&soft_serve_url))
                    .select(Repository::as_select())
                    .load::<Repository>(connection)
                    .map_err(|e| {
                        error!("Error getting repository: {}", e);
                        FailedToGetRepository(GetRepositoryError(e))
                    })?;
                if repo.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Repository for soft serve url {} not found",
                        &soft_serve_url
                    ));
                }
                Ok(repo)
            }
            _ => Err(anyhow::anyhow!("No input provided")),
        }
    }

    pub fn get_repo_with_relations(
        connection: &mut PgConnection,
        user_id: &Uuid,
        repo_url: Option<&str>,
        soft_serve_url: Option<&str>,
        language: Option<&str>,
        status: Option<&Status>,
        pagination: &PaginationParams,
    ) -> Result<PaginatedResponse<RepositoryWithRelations>> {
        use crate::schema::{challenges, progress, repositories};

        if repo_url.is_some() && soft_serve_url.is_some() {
            return Err(anyhow::anyhow!(
                "Multiple parameters are not allowed. Please provide only one parameter."
            ));
        }

        let page = pagination.page.unwrap_or(1);
        let per_page = pagination.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;

        // Build base query
        let base_query = repositories::table
            .inner_join(challenges::table)
            .left_join(progress::table.on(progress::repository_id.eq(repositories::id)))
            .filter(repositories::user_id.eq(user_id));

        // Build filters separately
        let repo_filter = repo_url.map(|url| repositories::repo_url.eq(url));
        let soft_serve_filter = soft_serve_url.map(|url| repositories::soft_serve_url.eq(url));
        let status_filter = status.map(|s| progress::status.eq(s.to_str()));
        let language_filter = language.map(|lang| repositories::language.eq(lang));

        // Apply filters to count query
        let mut count_query = base_query.into_boxed();
        if let Some(f) = repo_filter {
            count_query = count_query.filter(f);
        }
        if let Some(f) = soft_serve_filter {
            count_query = count_query.filter(f);
        }
        if let Some(f) = status_filter {
            count_query = count_query.filter(f);
        }
        if let Some(f) = language_filter {
            count_query = count_query.filter(f);
        }

        let total: i64 = count_query.count().get_result(connection)?;

        let total_pages = (total as f64 / per_page as f64).ceil() as i64;

        // Apply same filters to results query
        let mut results_query = base_query.into_boxed();
        if let Some(f) = repo_filter {
            results_query = results_query.filter(f);
        }
        if let Some(f) = soft_serve_filter {
            results_query = results_query.filter(f);
        }
        if let Some(f) = status_filter {
            results_query = results_query.filter(f);
        }
        if let Some(f) = language_filter {
            results_query = results_query.filter(f);
        }

        let results: Vec<(
            (
                Uuid,
                Uuid,
                Uuid,
                String,
                String,
                String,
                NaiveDateTime,
                NaiveDateTime,
            ),
            (
                String,
                String,
                serde_json::Value,
                String,
                i32,
                String,
                NaiveDateTime,
                NaiveDateTime,
            ),
            Option<(
                Uuid,
                String,
                Option<serde_json::Value>,
                NaiveDateTime,
                NaiveDateTime,
            )>,
        )> = results_query
            .offset(offset)
            .limit(per_page)
            .select((
                (
                    repositories::id,
                    repositories::user_id,
                    repositories::challenge_id,
                    repositories::repo_url,
                    repositories::soft_serve_url,
                    repositories::language,
                    repositories::created_at,
                    repositories::updated_at,
                ),
                (
                    challenges::title,
                    challenges::description,
                    challenges::repo_urls,
                    challenges::difficulty,
                    challenges::module_count,
                    challenges::mode,
                    challenges::created_at,
                    challenges::updated_at,
                ),
                (
                    progress::id,
                    progress::status,
                    progress::progress_details,
                    progress::created_at,
                    progress::updated_at,
                )
                    .nullable(),
            ))
            .load(connection)?;

        Ok(PaginatedResponse {
            data: results
                .into_iter()
                .map(|(repo, challenge, progress)| RepositoryWithRelations {
                    id: repo.0,
                    user_id: repo.1,
                    challenge_id: repo.2,
                    repo_url: repo.3,
                    soft_serve_url: repo.4,
                    language: repo.5,
                    created_at: repo.6,
                    updated_at: repo.7,
                    challenge: ChallengeInfo {
                        title: challenge.0,
                        description: challenge.1,
                        repo_urls: challenge.2,
                        difficulty: challenge.3,
                        module_count: challenge.4,
                        mode: challenge.5,
                        created_at: challenge.6,
                        updated_at: challenge.7,
                    },
                    progress: progress
                        .map(|p| ProgressInfo {
                            id: p.0,
                            status: p.1,
                            progress_details: p.2,
                            created_at: p.3,
                            updated_at: p.4,
                        })
                        .unwrap_or_default(),
                })
                .collect(),
            total,
            page,
            per_page,
            total_pages,
        })
    }

    pub fn get_repo_by_id(
        connection: &mut PgConnection,
        id: &Uuid,
        user_id: &Uuid,
    ) -> Result<RepositoryWithRelations> {
        use crate::schema::{challenges, progress, repositories};

        let result: (
            Uuid,
            Uuid,
            Uuid,
            String,
            String,
            String,
            NaiveDateTime,
            NaiveDateTime,
            (
                String,
                String,
                serde_json::Value,
                String,
                i32,
                String,
                NaiveDateTime,
                NaiveDateTime,
            ),
            Option<(
                Uuid,
                String,
                Option<serde_json::Value>,
                NaiveDateTime,
                NaiveDateTime,
            )>,
        ) = repositories::table
            .inner_join(challenges::table)
            .left_join(progress::table.on(progress::repository_id.eq(repositories::id)))
            .filter(repositories::user_id.eq(user_id))
            .filter(repositories::id.eq(id))
            .select((
                repositories::id,
                repositories::user_id,
                repositories::challenge_id,
                repositories::repo_url,
                repositories::soft_serve_url,
                repositories::language,
                repositories::created_at,
                repositories::updated_at,
                (
                    challenges::title,
                    challenges::description,
                    challenges::repo_urls,
                    challenges::difficulty,
                    challenges::module_count,
                    challenges::mode,
                    challenges::created_at,
                    challenges::updated_at,
                ),
                (
                    progress::id,
                    progress::status,
                    progress::progress_details,
                    progress::created_at,
                    progress::updated_at,
                )
                    .nullable(),
            ))
            .first(connection)?;

        // Transform the raw result into our nested structure
        Ok(RepositoryWithRelations {
            id: result.0,
            user_id: result.1,
            challenge_id: result.2,
            repo_url: result.3,
            soft_serve_url: result.4,
            language: result.5,
            created_at: result.6,
            updated_at: result.7,
            challenge: ChallengeInfo {
                title: result.8 .0,
                description: result.8 .1,
                repo_urls: result.8 .2,
                difficulty: result.8 .3,
                module_count: result.8 .4,
                mode: result.8 .5,
                created_at: result.8 .6,
                updated_at: result.8 .7,
            },
            progress: result
                .9
                .map(|p| ProgressInfo {
                    id: p.0,
                    status: p.1,
                    progress_details: p.2,
                    created_at: p.3,
                    updated_at: p.4,
                })
                .unwrap_or_default(),
        })
    }

    pub fn get_all_repos(
        connection: &mut PgConnection,
        period: &Period,
        challenge_id: &Uuid,
    ) -> Result<Vec<AttemptInfo>> {
        use crate::schema::{challenges, progress, repositories, users};
        use chrono::{Datelike, Duration, Utc};

        let now = Utc::now().naive_utc();
        let start_date = match period {
            Period::Today => now
                .date()
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow::anyhow!("Failed to create start date for today"))?,
            Period::ThisWeek => {
                let week_start = now - Duration::days(now.weekday().num_days_from_monday() as i64);
                week_start
                    .date()
                    .and_hms_opt(0, 0, 0)
                    .ok_or_else(|| anyhow::anyhow!("Failed to create start date for week"))?
            }
            Period::ThisMonth => now
                .with_day(1)
                .ok_or_else(|| anyhow::anyhow!("Failed to set day to 1"))?
                .date()
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow::anyhow!("Failed to create start date for month"))?,
            Period::AllTime => chrono::DateTime::from_timestamp(0, 0)
                .ok_or_else(|| anyhow::anyhow!("Failed to create Unix epoch timestamp"))?
                .naive_utc(),
        };

        let results = repositories::table
            .inner_join(challenges::table)
            .inner_join(users::table)
            .left_join(progress::table.on(progress::repository_id.eq(repositories::id)))
            .filter(repositories::created_at.ge(start_date))
            .filter(repositories::challenge_id.eq(challenge_id))
            .select((
                repositories::challenge_id,
                users::username,
                progress::progress_details.nullable(),
                challenges::module_count,
            ))
            .load::<(Uuid, String, Option<serde_json::Value>, i32)>(connection)?;

        Ok(results
            .into_iter()
            .map(|(challenge_id, username, progress_details, module_count)| {
                let total_score = progress_details
                    .and_then(|details| details.get("current_step").cloned())
                    .and_then(|step| step.as_i64())
                    .unwrap_or(0) as i32;

                AttemptInfo {
                    challenge_id,
                    username,
                    total_score,
                    module_count,
                }
            })
            .collect())
    }
}
