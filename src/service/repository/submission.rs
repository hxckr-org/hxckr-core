use crate::schema::submissions::table as submissions_table;
use crate::service::database::models::Submission;
use crate::shared::errors::{
    CreateSubmissionError, GetSubmissionError,
    RepositoryError::{FailedToCreateSubmission, FailedToGetSubmission},
};
use crate::shared::primitives::SubmissionStatus;
use crate::shared::utils::string_to_uuid;
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl Submission {
    pub fn new(
        exercise_id: &str,
        user_id: &str,
        status: SubmissionStatus,
        repository_id: &str,
        commit_id: &str,
    ) -> Self {
        Submission {
            id: Uuid::new_v4(),
            exercise_id: string_to_uuid(exercise_id).unwrap(),
            user_id: string_to_uuid(user_id).unwrap(),
            status: status.to_str().to_string(),
            repository_id: string_to_uuid(repository_id).unwrap(),
            commit_id: commit_id.to_string(),
            feedback: None,
            submitted_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create_submission(
        connection: &mut PgConnection,
        submission: Submission,
    ) -> Result<Submission> {
        let submission = submission
            .insert_into(submissions_table)
            .returning(Submission::as_returning())
            .get_result(connection)
            .map_err(|e| {
                error!("Error creating submission: {}", e);
                FailedToCreateSubmission(CreateSubmissionError(e))
            })?;
        Ok(submission)
    }

    pub fn get_submission(
        connection: &mut PgConnection,
        id: Option<String>,
        user_id: Option<String>,
    ) -> Result<Vec<Submission>> {
        use crate::schema::submissions::dsl::user_id as user_id_col;

        match (id, user_id) {
            (Some(id), None) => {
                let id_uuid = string_to_uuid(&id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("Submission ID is not valid")
                })?;
                let submission = submissions_table
                    .find(id_uuid)
                    .first::<Submission>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting submission: {}", e);
                        FailedToGetSubmission(GetSubmissionError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Submission not found"))?;
                Ok(vec![submission])
            }
            (None, Some(user_id)) => {
                let user_id_uuid = string_to_uuid(&user_id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("User ID is not valid")
                })?;
                let submissions = submissions_table
                    .filter(user_id_col.eq(user_id_uuid))
                    .load::<Submission>(connection)
                    .map_err(|e| {
                        error!("Error getting submissions: {}", e);
                        FailedToGetSubmission(GetSubmissionError(e))
                    })?;
                if submissions.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Submissions for user id {} not found",
                        user_id
                    ));
                }
                Ok(submissions)
            }
            _ => Err(anyhow::anyhow!("Invalid input")),
        }
    }
}
