use crate::schema::challenges::table as challenges_table;
use crate::service::database::models::Challenge;
use crate::shared::errors::{
    CreateChallengeError,
    RepositoryError::{
        FailedToCreateChallenge, FailedToDeleteChallenge, FailedToGetChallenge,
        FailedToUpdateChallenge,
    },
};
use crate::shared::errors::{DeleteChallengeError, GetChallengeError, UpdateChallengeError};
use crate::shared::primitives::{ChallengeMode, Difficulty};
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use serde_json::json;
use uuid::Uuid;

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::challenges)]
struct ChallengeChanges {
    title: Option<String>,
    description: Option<String>,
    repo_urls: Option<serde_json::Value>,
    module_count: Option<i32>,
    difficulty: Option<String>,
    mode: Option<String>,
}

impl Challenge {
    pub fn new(
        title: &str,
        description: &str,
        repo_urls: &serde_json::Value,
        module_count: &i32,
        difficulty: &str,
        mode: &str,
    ) -> Self {
        Challenge {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: description.to_string(),
            repo_urls: repo_urls.clone(),
            difficulty: difficulty.to_string(),
            module_count: *module_count,
            mode: mode.to_string(),
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
        id: Option<&Uuid>,
        title: Option<&str>,
        difficulty: Option<&Difficulty>,
        mode: Option<&ChallengeMode>,
    ) -> Result<Challenge> {
        use crate::schema::challenges::dsl::{
            difficulty as difficulty_col, mode as mode_col, title as title_col,
        };

        match (id, title, difficulty, mode) {
            (Some(id), None, None, None) => {
                let challenge = challenges_table
                    .find(id)
                    .select(Challenge::as_select())
                    .first::<Challenge>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting challenge: {}", e);
                        FailedToGetChallenge(GetChallengeError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Challenge not found"))?;
                Ok(challenge)
            }
            (None, Some(title), None, None) => {
                let challenge = challenges_table
                    .filter(title_col.eq(title.to_lowercase()))
                    .select(Challenge::as_select())
                    .first::<Challenge>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting challenge: {}", e);
                        FailedToGetChallenge(GetChallengeError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Challenge not found"))?;
                Ok(challenge)
            }
            (None, None, Some(difficulty), None) => {
                let challenge = challenges_table
                    .filter(difficulty_col.eq(difficulty.to_str()))
                    .select(Challenge::as_select())
                    .first::<Challenge>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting challenge: {}", e);
                        FailedToGetChallenge(GetChallengeError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Challenge not found"))?;
                Ok(challenge)
            }
            (None, None, None, Some(mode)) => {
                let challenge = challenges_table
                    .filter(mode_col.eq(mode.to_str()))
                    .select(Challenge::as_select())
                    .first::<Challenge>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting challenge: {}", e);
                        FailedToGetChallenge(GetChallengeError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Challenge not found"))?;
                Ok(challenge)
            }
            (Some(_), None, Some(_), _) => {
                Err(anyhow::anyhow!("Cannot provide both id and difficulty"))
            }
            (Some(_), Some(_), _, _) => Err(anyhow::anyhow!("Cannot provide both id and title")),
            (None, Some(_), Some(_), _) => {
                Err(anyhow::anyhow!("Cannot provide both title and difficulty"))
            }
            (None, None, Some(_), _) => {
                Err(anyhow::anyhow!("Cannot provide both title and difficulty"))
            }
            (None, None, None, None) => Err(anyhow::anyhow!("No input provided")),
            (None, Some(_), None, Some(_)) => {
                Err(anyhow::anyhow!("Cannot provide both title and mode"))
            }
            (Some(_), None, None, Some(_)) => {
                Err(anyhow::anyhow!("Cannot provide both id and mode"))
            }
        }
    }

    pub fn get_all_challenges(connection: &mut PgConnection) -> Result<Vec<Challenge>> {
        challenges_table
            .select(Challenge::as_select())
            .load(connection)
            .map_err(|e| {
                error!("Error getting challenges: {}", e);
                FailedToGetChallenge(GetChallengeError(e)).into()
            })
    }
    pub fn delete(connection: &mut PgConnection, challenge_id: &Uuid) -> Result<()> {
        use crate::schema::challenges::dsl::id;

        match diesel::delete(challenges_table.filter(id.eq(challenge_id))).execute(connection) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error deleting challenge: {}", e);
                Err(FailedToDeleteChallenge(DeleteChallengeError(e)).into())
            }
        }
    }
    pub fn update(
        connection: &mut PgConnection,
        challenge_id: &Uuid,
        title: Option<&str>,
        description: Option<&str>,
        repo_urls: Option<&serde_json::Value>,
        module_count: Option<&i32>,
        difficulty: Option<&Difficulty>,
        mode: Option<&ChallengeMode>,
    ) -> Result<Challenge> {
        use crate::schema::challenges::dsl::id;

        let changes = ChallengeChanges {
            title: title.map(|t| t.to_lowercase()),
            description: description.map(|d| d.to_string()),
            repo_urls: repo_urls.cloned(),
            module_count: module_count.copied(),
            difficulty: difficulty.map(|d| d.to_str().to_string()),
            mode: mode.map(|m| m.to_str().to_string()),
        };

        let updated_challenge = diesel::update(challenges_table.filter(id.eq(challenge_id)))
            .set(changes)
            .returning(Challenge::as_returning())
            .get_result(connection)
            .map_err(|e| {
                error!("Error updating challenge: {}", e);
                FailedToUpdateChallenge(UpdateChallengeError(e))
            })?;

        Ok(updated_challenge)
    }
    pub fn get_challenge_by_repo_url(
        connection: &mut PgConnection,
        repo_url: &str,
        language: &str,
    ) -> Result<Challenge> {
        use crate::schema::challenges::dsl::repo_urls;

        challenges_table
            .filter(repo_urls.contains(json!({ language: repo_url })))
            .select(Challenge::as_select())
            .first(connection)
            .map_err(|e| {
                error!("Error getting challenge by repo URL: {}", e);
                FailedToGetChallenge(GetChallengeError(e)).into()
            })
    }
}
