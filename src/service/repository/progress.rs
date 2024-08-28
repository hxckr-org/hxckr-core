use crate::schema::progress::table as progress_table;
use crate::service::database::models::Progress;
use crate::shared::errors::{
    CreateProgressError, GetProgressError,
    RepositoryError::{FailedToCreateProgress, FailedToGetProgress, FailedToUpdateProgress},
    UpdateProgressError,
};
use crate::shared::primitives::Status;
use crate::shared::utils::string_to_uuid;
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl Progress {
    pub fn new(user_id: &str, challenge_id: &str, status: Status) -> Self {
        Progress {
            id: Uuid::new_v4(),
            user_id: string_to_uuid(user_id).unwrap(),
            challenge_id: string_to_uuid(challenge_id).unwrap(),
            status: status.to_str().to_string(),
            progress_details: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create_progress(connection: &mut PgConnection, progress: Progress) -> Result<Progress> {
        let progress = progress
            .insert_into(progress_table)
            .returning(Progress::as_returning())
            .get_result(connection)
            .map_err(|e| {
                error!("Error creating progress: {}", e);
                FailedToCreateProgress(CreateProgressError(e))
            })?;

        Ok(progress)
    }

    pub fn update_progress(
        connection: &mut PgConnection,
        id: &str,
        status: &str,
    ) -> Result<Progress> {
        use crate::schema::progress::dsl::status as status_col;
        let id_uuid = string_to_uuid(id).map_err(|e| {
            error!("Error parsing UUID: {}", e);
            anyhow::anyhow!("Progress ID is not valid")
        })?;
        let updated_progress = diesel::update(progress_table.find(id_uuid))
            .set(status_col.eq(status))
            .returning(Progress::as_returning())
            .get_result(connection)
            .map_err(|e| {
                error!("Error updating progress: {}", e);
                FailedToUpdateProgress(UpdateProgressError(e))
            })?;
        Ok(updated_progress)
    }

    pub fn get_progress(
        connection: &mut PgConnection,
        id: Option<String>,
        user_id: Option<String>,
        challenge_id: Option<String>,
    ) -> Result<Vec<Progress>> {
        use crate::schema::progress::dsl::{
            challenge_id as challenge_id_col, user_id as user_id_col,
        };

        match (id, user_id, challenge_id) {
            (Some(id), None, None) => {
                let id_uuid = string_to_uuid(&id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("Progress ID is not valid")
                })?;
                let progress = progress_table
                    .find(id_uuid)
                    .first::<Progress>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting progress: {}", e);
                        FailedToGetProgress(GetProgressError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Progress not found"))?;
                Ok(vec![progress])
            }
            (None, Some(user_id), None) => {
                let user_id_uuid = string_to_uuid(&user_id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("User ID is not valid")
                })?;
                let progress = progress_table
                    .filter(user_id_col.eq(user_id_uuid))
                    .load::<Progress>(connection)
                    .map_err(|e| {
                        error!("Error getting progress: {}", e);
                        FailedToGetProgress(GetProgressError(e))
                    })?;
                if progress.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Progress for user id {} not found",
                        user_id
                    ));
                }
                Ok(progress)
            }
            (None, None, Some(challenge_id)) => {
                let challenge_id_uuid = string_to_uuid(&challenge_id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("Challenge ID is not valid")
                })?;
                let progress = progress_table
                    .filter(challenge_id_col.eq(challenge_id_uuid))
                    .load::<Progress>(connection)
                    .map_err(|e| {
                        error!("Error getting progress: {}", e);
                        FailedToGetProgress(GetProgressError(e))
                    })?;
                if progress.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Progress for challenge id {} not found",
                        challenge_id
                    ));
                }
                Ok(progress)
            }
            (None, None, None) => Err(anyhow::anyhow!("No input provided")),
            _ => Err(anyhow::anyhow!("Invalid input")),
        }
    }
}
