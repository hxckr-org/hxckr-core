use crate::schema::sessions::token as session_token;
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

use crate::{
    schema::sessions::table as session_table, service::database::models::Session,
    shared::utils::string_to_uuid,
};

impl Session {
    pub fn new(user_id: String, token: String, provider: String) -> Self {
        Session {
            id: Uuid::new_v4(),
            user_id: string_to_uuid(&user_id).unwrap(),
            token,
            provider,
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
                error!("Error creating session: {}", e);
                e
            })?;
        Ok(session)
    }

    pub fn get_by_token(connection: &mut PgConnection, token: String) -> Result<Session> {
        let session = session_table
            .filter(session_token.eq(token))
            .first(connection)
            .map_err(|e| {
                error!("Error getting session: {}", e);
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
