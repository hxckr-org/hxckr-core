use crate::schema::progress::table as progress_table;
use crate::service::database::models::Progress;
use crate::shared::errors::{
    CreateProgressError, GetProgressError,
    RepositoryError::{FailedToCreateProgress, FailedToGetProgress, FailedToUpdateProgress},
    UpdateProgressError,
};
use crate::shared::primitives::Status;
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use serde_json::Value;
use uuid::Uuid;

impl Progress {
    pub fn new(
        user_id: &Uuid,
        challenge_id: &Uuid,
        status: Status,
        progress_details: Option<Value>,
    ) -> Self {
        Progress {
            id: Uuid::new_v4(),
            user_id: user_id.clone(),
            challenge_id: challenge_id.clone(),
            status: status.to_str().to_string(),
            progress_details,
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
        id: &Uuid,
        status: Status,
        progress_details: Option<Value>,
    ) -> Result<Progress> {
        use crate::schema::progress::dsl::{
            progress_details as progress_details_col, status as status_col,
        };

        match Some(progress_details) {
            Some(details) => {
                let updated_progress = diesel::update(progress_table.find(id))
                    .set((
                        status_col.eq(status.to_str()),
                        progress_details_col.eq(details),
                    ))
                    .returning(Progress::as_returning())
                    .get_result(connection)
                    .map_err(|e| {
                        error!("Error updating progress: {}", e);
                        FailedToUpdateProgress(UpdateProgressError(e))
                    })?;
                Ok(updated_progress)
            }
            None => {
                let updated_progress = diesel::update(progress_table.find(id))
                    .set(status_col.eq(status.to_str()))
                    .returning(Progress::as_returning())
                    .get_result(connection)
                    .map_err(|e| {
                        error!("Error updating progress: {}", e);
                        FailedToUpdateProgress(UpdateProgressError(e))
                    })?;
                Ok(updated_progress)
            }
        }
    }

    pub fn get_progress(
        connection: &mut PgConnection,
        id: Option<&Uuid>,
        user_id: Option<&Uuid>,
        challenge_id: Option<&Uuid>,
    ) -> Result<Progress> {
        use crate::schema::progress::dsl::{
            challenge_id as challenge_id_col, user_id as user_id_col,
        };

        match (id, user_id, challenge_id) {
            (Some(id), None, None) => {
                let progress = progress_table
                    .find(id)
                    .first::<Progress>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting progress: {}", e);
                        FailedToGetProgress(GetProgressError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Progress not found"))?;
                Ok(progress)
            }
            (None, Some(user_id), None) => {
                let progress = progress_table
                    .filter(user_id_col.eq(user_id))
                    .first::<Progress>(connection)
                    .map_err(|e| {
                        error!("Error getting progress: {}", e);
                        FailedToGetProgress(GetProgressError(e))
                    })?;
                Ok(progress)
            }
            (None, None, Some(challenge_id)) => {
                let progress = progress_table
                    .filter(challenge_id_col.eq(challenge_id))
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
                Ok(progress[0].clone())
            }
            (None, None, None) => Err(anyhow::anyhow!("No input provided")),
            _ => Err(anyhow::anyhow!("Invalid input")),
        }
    }
}
