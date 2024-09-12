use crate::schema::repositories::table as repositories;
use crate::shared::errors::{
    CreateRepositoryError, GetRepositoryError,
    RepositoryError::{FailedToCreateRepository, FailedToGetRepository},
};
use crate::{service::database::models::Repository, shared::utils::string_to_uuid};
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl Repository {
    pub fn new(user_id: &str, challenge_id: &str, repo_url: &str) -> Self {
        Repository {
            id: Uuid::new_v4(),
            user_id: string_to_uuid(user_id).unwrap(),
            challenge_id: string_to_uuid(challenge_id).unwrap(),
            repo_url: repo_url.to_string(),
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
        id: Option<String>,
        user_id: Option<String>,
        repo_url: Option<String>,
    ) -> Result<Vec<Repository>> {
        use crate::schema::repositories::dsl::{repo_url as repo_url_col, user_id as user_id_col};

        match (id, user_id, repo_url) {
            (Some(id), None, None) => {
                let id_uuid = string_to_uuid(&id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("Repository ID is not valid")
                })?;
                let repo = repositories
                    .find(id_uuid)
                    .first::<Repository>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting repository: {}", e);
                        FailedToGetRepository(GetRepositoryError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Repository not found"))?;
                Ok(vec![repo])
            }
            (None, Some(user_id), None) => {
                let user_id_uuid = string_to_uuid(&user_id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("User ID is not valid")
                })?;
                let repo = repositories
                    .filter(user_id_col.eq(user_id_uuid))
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
            (None, None, Some(repo_url)) => {
                let repo = repositories
                    .filter(repo_url_col.eq(&repo_url))
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
            (None, None, None) => Err(anyhow::anyhow!("No input provided")),
            _ => Err(anyhow::anyhow!("Invalid input")),
        }
    }
}
