use crate::schema::{badges::table as badges_table, user_badges::table as user_badges_table};
use crate::service::database::models::{Badge, NewBadge, NewUserBadge, UserBadge};
use crate::shared::errors::{
    CreateBadgeError, CreateUserBadgeError, GetBadgeError, GetUserBadgeError,
    RepositoryError::{
        FailedToCreateBadge, FailedToCreateUserBadge, FailedToGetBadge, FailedToGetUserBadge,
    },
};
use crate::shared::utils::string_to_uuid;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use log::error;

impl Badge {
    pub fn new(name: &str, description: &str) -> NewBadge {
        NewBadge {
            name: name.to_string(),
            description: Some(description.to_string()),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create(badge: NewBadge, connection: &mut PgConnection) -> Result<Badge> {
        match diesel::insert_into(crate::schema::badges::table)
            .values(badge)
            .returning(Badge::as_returning())
            .get_result(connection)
        {
            Ok(badge) => Ok(badge),
            Err(e) => {
                error!("Error creating badge: {}", e);
                Err(FailedToCreateBadge(CreateBadgeError(e)).into())
            }
        }
    }

    pub fn get_badge(id: i32, connection: &mut PgConnection) -> Result<Badge> {
        match badges_table.find(id).first(connection) {
            Ok(badge) => Ok(badge),
            Err(e) => {
                error!("Error getting badge: {}", e);
                Err(FailedToGetBadge(GetBadgeError(e)).into())
            }
        }
    }
}

impl UserBadge {
    pub fn new(user_id: &str, badge_id: i32) -> NewUserBadge {
        NewUserBadge {
            user_id: string_to_uuid(user_id).unwrap(),
            badge_id,
            awarded_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create(user_badge: NewUserBadge, connection: &mut PgConnection) -> Result<UserBadge> {
        match diesel::insert_into(user_badges_table)
            .values(user_badge)
            .returning(UserBadge::as_returning())
            .get_result(connection)
        {
            Ok(user_badge) => Ok(user_badge),
            Err(e) => {
                error!("Error creating user badge: {}", e);
                Err(FailedToCreateUserBadge(CreateUserBadgeError(e)).into())
            }
        }
    }

    pub fn get_user_badges(user_id: &str, connection: &mut PgConnection) -> Result<Vec<UserBadge>> {
        use crate::schema::user_badges::user_id as user_id_column;
        let uuid = string_to_uuid(user_id).map_err(|e| {
            error!("Error converting user id to uuid: {}", e);
            anyhow!("Error converting user id to uuid: {}", e)
        })?;
        let user_badges = user_badges_table
            .filter(user_id_column.eq(uuid))
            .load::<UserBadge>(connection)
            .map_err(|e| {
                error!("Error getting user badges: {}", e);
                FailedToGetUserBadge(GetUserBadgeError(e))
            })?;

        Ok(user_badges)
    }
}
