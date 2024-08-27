use crate::schema::challenges::table as challenges_table;
use crate::service::database::models::Challenge;
use crate::shared::errors::GetChallengeError;
use crate::shared::errors::{
    CreateChallengeError,
    RepositoryError::{ChallengeNotFound, FailedToCreateChallenge, FailedToGetChallenge},
};
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

pub fn create_challenge(connection: &mut PgConnection, challenge: Challenge) -> Result<Challenge> {
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

pub fn get_challenge(connection: &mut PgConnection, id: String) -> Result<Option<Challenge>> {
    let uuid = Uuid::parse_str(&id).map_err(|e| {
        error!("Error parsing UUID: {}", e);
        anyhow::anyhow!("Challenge ID is not valid")
    })?;

    let challenge = challenges_table
        .find(uuid)
        .first::<Challenge>(connection)
        .optional()
        .map_err(|e| {
            error!("Error getting challenge: {}", e);
            FailedToGetChallenge(GetChallengeError(e))
        })?;

    if challenge.is_none() {
        return Err(ChallengeNotFound.into());
    }
    Ok(challenge)
}
