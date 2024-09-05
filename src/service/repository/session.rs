use crate::schema::sessions::{token as session_token, user_id as session_user_id};
use anyhow::{Ok, Result};
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

use crate::{schema::sessions::table as session_table, service::database::models::Session};

impl Session {
    pub fn new(user_id: &Uuid, token: &str, provider: &str) -> Self {
        Session {
            id: Uuid::new_v4(),
            user_id: user_id.to_owned(),
            token: token.to_string(),
            provider: provider.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::days(30),
        }
    }

    pub fn create(connection: &mut PgConnection, session: Session) -> Result<Session> {
        let session = session
            .insert_into(session_table)
            .returning(Session::as_returning())
            .get_result(connection)
            .map_err(|e| {
                error!("Error creating session: {:?}", e);
                e
            })?;
        Ok(session)
    }

    pub fn get_by_token(connection: &mut PgConnection, token: String) -> Result<Session> {
        let session = session_table
            .filter(session_token.eq(token))
            .first(connection)
            .map_err(|e| {
                error!("Error getting session: {:?}", e);
                e
            })?;
        Ok(session)
    }

    pub fn get_by_userid(connection: &mut PgConnection, user_id: &Uuid) -> Result<Option<Session>> {
        let session = session_table
            .filter(session_user_id.eq(user_id))
            .first(connection)
            .optional()
            .map_err(|e| {
                error!("Error getting session: {:?}", e);
                e
            })?;
        Ok(session)
    }

    pub fn delete(connection: &mut PgConnection, token: String) -> Result<usize> {
        diesel::delete(session_table.filter(session_token.eq(token)))
            .execute(connection)
            .map_err(|e| {
                error!("Error deleting session: {}", e);
                e.into()
            })
    }
}
