use crate::schema::exercises::table as exercises_table;
use crate::service::database::models::Exercise;
use crate::shared::errors::{
    CreateExerciseError, GetExerciseError,
    RepositoryError::{FailedToCreateExercise, FailedToGetExercise},
};
use crate::shared::primitives::{Difficulty, Status};
use crate::shared::utils::string_to_uuid;
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl Exercise {
    pub fn new(
        title: &str,
        description: &str,
        difficulty: Difficulty,
        test_runner: &str,
        status: Status,
        challenge_id: &str,
    ) -> Self {
        Exercise {
            id: Uuid::new_v4(),
            title: title.to_lowercase().trim().to_string(),
            description: description.to_string(),
            difficulty: difficulty.to_str().to_string(),
            test_runner: test_runner.to_string(),
            challenge_id: string_to_uuid(challenge_id).unwrap(),
            status: status.to_str().to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create_exercise(connection: &mut PgConnection, exercise: Exercise) -> Result<Exercise> {
        let exercise = exercise
            .insert_into(exercises_table)
            .returning(Exercise::as_returning())
            .get_result(connection)
            .map_err(|e| {
                error!("Error creating exercise: {}", e);
                FailedToCreateExercise(CreateExerciseError(e))
            })?;

        Ok(exercise)
    }

    pub fn get_exercise(
        connection: &mut PgConnection,
        id: Option<String>,
        challenge_id: Option<String>,
    ) -> Result<Vec<Exercise>> {
        use crate::schema::exercises::dsl::challenge_id as challenge_id_col;

        match (id, challenge_id) {
            (Some(id), _) => {
                let exercise_uuid = string_to_uuid(&id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("Exercise ID is not valid")
                })?;
                let exercise = exercises_table
                    .find(exercise_uuid)
                    .first::<Exercise>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting exercise: {}", e);
                        FailedToGetExercise(GetExerciseError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("Exercise not found"))?;
                Ok(vec![exercise])
            }
            (_, Some(challenge_id)) => {
                let challenge_uuid = string_to_uuid(&challenge_id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("Challenge ID is not valid")
                })?;
                let exercises = exercises_table
                    .filter(challenge_id_col.eq(challenge_uuid))
                    .load::<Exercise>(connection)
                    .map_err(|e| {
                        error!("Error getting exercises: {}", e);
                        FailedToGetExercise(GetExerciseError(e))
                    })?;
                if exercises.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Exercises for challenge id {} not found",
                        challenge_id
                    ));
                }
                Ok(exercises)
            }
            (None, None) => Err(anyhow::anyhow!("No input provided")),
        }
    }
}
