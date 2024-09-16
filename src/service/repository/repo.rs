use crate::schema::repositories::table as repositories;
use crate::service::database::models::Repository;
use crate::shared::errors::{
    CreateRepositoryError, GetRepositoryError,
    RepositoryError::{FailedToCreateRepository, FailedToGetRepository},
};
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl Repository {
    pub fn new(user_id: &Uuid, challenge_id: &Uuid, repo_url: &str, soft_serve_url: &str) -> Self {
        Repository {
            id: Uuid::new_v4(),
            user_id: user_id.to_owned(),
            challenge_id: challenge_id.to_owned(),
            repo_url: repo_url.to_string(),
            soft_serve_url: soft_serve_url.to_string(),
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
}
