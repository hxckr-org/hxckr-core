use crate::schema::leaderboard::table as leaderboard_table;
use crate::service::database::models::Leaderboard;
use crate::shared::errors::{
    CreateLeaderboardError, GetLeaderboardError,
    RepositoryError::{
        FailedToCreateLeaderboard, FailedToGetLeaderboard, FailedToUpdateLeaderboard,
    },
    UpdateLeaderboardError,
};
use crate::shared::utils::string_to_uuid;
use anyhow::Result;
use diesel::prelude::*;
use log::error;

impl Leaderboard {
    pub fn new(user_id: &str, achievements: Option<serde_json::Value>, score: i32) -> Self {
        Leaderboard {
            id: Default::default(),
            user_id: string_to_uuid(user_id).unwrap(),
            score,
            achievements,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create(
        connection: &mut PgConnection,
        new_leaderboard: Leaderboard,
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
        user_id: Option<String>,
    ) -> Result<Vec<Leaderboard>> {
        use crate::schema::leaderboard::dsl::user_id as user_id_col;

        match user_id {
            Some(user_id) => {
                let user_id_uuid = string_to_uuid(&user_id).map_err(|e| {
                    error!("Error parsing UUID: {}", e);
                    anyhow::anyhow!("User ID is not valid")
                })?;
                let leaderboard = leaderboard_table
                    .filter(user_id_col.eq(user_id_uuid))
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
        user_id: &str,
        new_score: i32,
        new_achievements: Option<serde_json::Value>,
    ) -> Result<Leaderboard> {
        use crate::schema::leaderboard::dsl::{
            achievements as achievements_col, score as score_col, updated_at as updated_at_col,
            user_id as user_id_col,
        };

        let user_id_uuid = string_to_uuid(user_id).map_err(|e| {
            error!("Error parsing UUID: {}", e);
            anyhow::anyhow!("User ID is not valid")
        })?;

        let updated_leaderboard =
            diesel::update(leaderboard_table.filter(user_id_col.eq(user_id_uuid)))
                .set((
                    score_col.eq(new_score),
                    achievements_col.eq(serde_json::to_value(new_achievements).unwrap()),
                    updated_at_col.eq(chrono::Utc::now().naive_utc()),
                ))
                .returning(Leaderboard::as_returning())
                .get_result(connection)
                .map_err(|e| {
                    error!("Error updating leaderboard: {}", e);
                    FailedToUpdateLeaderboard(UpdateLeaderboardError(e))
                })?;

        Ok(updated_leaderboard)
    }
}