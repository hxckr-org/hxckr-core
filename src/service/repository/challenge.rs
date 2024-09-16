use crate::schema::challenges::table as challenges_table;
use crate::service::database::models::Challenge;
use crate::shared::errors::GetChallengeError;
use crate::shared::errors::{
    CreateChallengeError,
    RepositoryError::{FailedToCreateChallenge, FailedToGetChallenge},
};
use crate::shared::primitives::{ChallengeMode, Difficulty};
use crate::shared::utils::string_to_uuid;
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl Challenge {
    pub fn new(
        title: &str,
        description: &str,
        repo_url: &str,
        difficulty: &Difficulty,
        mode: &ChallengeMode,
    ) -> Self {
        Challenge {
            id: Uuid::new_v4(),
            title: title.to_lowercase().trim().to_string(),
            description: description.to_string(),
            repo_url: repo_url.to_string(),
            difficulty: difficulty.to_str().to_string(),
            mode: mode.to_str().to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create(connection: &mut PgConnection, challenge: Challenge) -> Result<Challenge> {
        match diesel::insert_into(challenges_table)
            .values(challenge)
            .returning(Challenge::as_returning())
            .get_result(connection)
        {
            Ok(challenge) => Ok(challenge),
            Err(e) => {
                error!("Error creating challenge: {}", e);
                Err(FailedToCreateChallenge(CreateChallengeError(e)).into())
            }
        }
    }

    pub fn get_challenge(
        connection: &mut PgConnection,
        id: Option<String>,
        repo_url: Option<&str>,
    ) -> Result<Challenge> {
        use crate::schema::challenges::dsl::repo_url as repo_url_col;

        match (id, repo_url) {
            (Some(id), None) => {
                let uuid = string_to_uuid(&id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("Challenge ID is not valid")
                })?;
                let challenge = challenges_table
                    .find(uuid)
                    .first::<Challenge>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting challenge: {}", e);
                        FailedToGetChallenge(GetChallengeError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Challenge not found"))?;
                Ok(challenge)
            }
            (None, Some(repo_url)) => {
                let challenge = challenges_table
                    .filter(repo_url_col.eq(repo_url))
                    .first::<Challenge>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting challenge: {}", e);
                        FailedToGetChallenge(GetChallengeError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Challenge not found"))?;
                Ok(challenge)
            }
            (Some(_), Some(_)) => Err(anyhow::anyhow!("Cannot provide both id and repo_url")),
            (None, None) => Err(anyhow::anyhow!("No input provided")),
        }
    }
}
