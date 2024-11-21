use crate::schema::leaderboard::table as leaderboard_table;
use crate::service::database::models::{Leaderboard, NewLeaderboard};
use crate::shared::errors::{
    CreateLeaderboardError, GetLeaderboardError,
    RepositoryError::{
        FailedToCreateLeaderboard, FailedToGetLeaderboard, FailedToUpdateLeaderboard,
    },
    UpdateLeaderboardError,
};
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl Leaderboard {
    pub fn new(
        user_id: &Uuid,
        achievements: Option<serde_json::Value>,
        score: i32,
        expected_total_score: i32,
    ) -> NewLeaderboard {
        NewLeaderboard {
            user_id: user_id.clone(),
            score,
            expected_total_score,
            achievements,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create(
        connection: &mut PgConnection,
        new_leaderboard: NewLeaderboard,
    ) -> Result<Leaderboard> {
        let leaderboard = new_leaderboard
            .insert_into(leaderboard_table)
            .returning(Leaderboard::as_returning())
            .get_result(connection)
            .map_err(|e| {
                error!("Error creating leaderboard: {}", e);
                FailedToCreateLeaderboard(CreateLeaderboardError(e))
            })?;

        Ok(leaderboard)
    }

    pub fn get_leaderboard(
        connection: &mut PgConnection,
        user_id: Option<&Uuid>,
    ) -> Result<Vec<Leaderboard>> {
        use crate::schema::leaderboard::dsl::user_id as user_id_col;

        match user_id {
            Some(user_id) => {
                let leaderboard = leaderboard_table
                    .filter(user_id_col.eq(user_id))
                    .select(Leaderboard::as_select())
                    .first::<Leaderboard>(connection)
                    .map_err(|e| {
                        error!("Error getting leaderboard: {}", e);
                        FailedToGetLeaderboard(GetLeaderboardError(e))
                    })?;
                Ok(vec![leaderboard])
            }
            None => {
                let leaderboard = leaderboard_table
                    .select(Leaderboard::as_select())
                    .load::<Leaderboard>(connection)
                    .map_err(|e| {
                        error!("Error getting leaderboard: {}", e);
                        FailedToGetLeaderboard(GetLeaderboardError(e))
                    })?;
                Ok(leaderboard)
            }
        }
    }

    pub fn update(
        connection: &mut PgConnection,
        user_id: &Uuid,
        new_score: Option<i32>,
        new_expected_total_score: Option<i32>,
        new_achievements: Option<serde_json::Value>,
    ) -> Result<Leaderboard> {
        use crate::schema::leaderboard::dsl::{
            achievements as achievements_col, expected_total_score as expected_total_score_col,
            score as score_col, user_id as user_id_col,
        };

        match (new_score, new_achievements, new_expected_total_score) {
            (Some(new_score), Some(new_achievements), None) => {
                let updated_leaderboard =
                    diesel::update(leaderboard_table.filter(user_id_col.eq(user_id)))
                        .set((
                            score_col.eq(new_score),
                            achievements_col.eq(new_achievements),
                        ))
                        .returning(Leaderboard::as_returning())
                        .get_result(connection)
                        .map_err(|e| {
                            error!("Error updating leaderboard: {}", e);
                            FailedToUpdateLeaderboard(UpdateLeaderboardError(e))
                        })?;
                Ok(updated_leaderboard)
            }
            (Some(new_score), None, None) => {
                let updated_leaderboard =
                    diesel::update(leaderboard_table.filter(user_id_col.eq(user_id)))
                        .set((score_col.eq(new_score),))
                        .returning(Leaderboard::as_returning())
                        .get_result(connection)
                        .map_err(|e| {
                            error!("Error updating leaderboard: {}", e);
                            FailedToUpdateLeaderboard(UpdateLeaderboardError(e))
                        })?;
                Ok(updated_leaderboard)
            }
            (None, Some(new_achievements), None) => {
                let updated_leaderboard =
                    diesel::update(leaderboard_table.filter(user_id_col.eq(user_id)))
                        .set((achievements_col.eq(new_achievements),))
                        .returning(Leaderboard::as_returning())
                        .get_result(connection)
                        .map_err(|e| {
                            error!("Error updating leaderboard: {}", e);
                            FailedToUpdateLeaderboard(UpdateLeaderboardError(e))
                        })?;
                Ok(updated_leaderboard)
            }
            (None, None, Some(new_expected_total_score)) => {
                let updated_leaderboard =
                    diesel::update(leaderboard_table.filter(user_id_col.eq(user_id)))
                        .set((expected_total_score_col.eq(new_expected_total_score),))
                        .returning(Leaderboard::as_returning())
                        .get_result(connection)
                        .map_err(|e| {
                            error!("Error updating leaderboard: {}", e);
                            FailedToUpdateLeaderboard(UpdateLeaderboardError(e))
                        })?;
                Ok(updated_leaderboard)
            }
            (Some(score), Some(achievements), Some(expected_total_score)) => {
                let updated_leaderboard =
                    diesel::update(leaderboard_table.filter(user_id_col.eq(user_id)))
                        .set((
                            score_col.eq(score),
                            achievements_col.eq(achievements),
                            expected_total_score_col.eq(expected_total_score),
                        ))
                        .returning(Leaderboard::as_returning())
                        .get_result(connection)
                        .map_err(|e| {
                            error!("Error updating leaderboard: {}", e);
                            FailedToUpdateLeaderboard(UpdateLeaderboardError(e))
                        })?;
                Ok(updated_leaderboard)
            }
            (Some(score), None, Some(expected_total_score)) => {
                let updated_leaderboard =
                    diesel::update(leaderboard_table.filter(user_id_col.eq(user_id)))
                        .set((
                            score_col.eq(score),
                            expected_total_score_col.eq(expected_total_score),
                        ))
                        .returning(Leaderboard::as_returning())
                        .get_result(connection)
                        .map_err(|e| {
                            error!("Error updating leaderboard: {}", e);
                            FailedToUpdateLeaderboard(UpdateLeaderboardError(e))
                        })?;
                Ok(updated_leaderboard)
            }
            (None, Some(achievements), Some(expected_total_score)) => {
                let updated_leaderboard =
                    diesel::update(leaderboard_table.filter(user_id_col.eq(user_id)))
                        .set((
                            achievements_col.eq(achievements),
                            expected_total_score_col.eq(expected_total_score),
                        ))
                        .returning(Leaderboard::as_returning())
                        .get_result(connection)
                        .map_err(|e| {
                            error!("Error updating leaderboard: {}", e);
                            FailedToUpdateLeaderboard(UpdateLeaderboardError(e))
                        })?;
                Ok(updated_leaderboard)
            }
            (None, None, None) => Err(anyhow::anyhow!("No new score or achievements provided")),
        }
    }
}
