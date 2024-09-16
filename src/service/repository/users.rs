use crate::schema::users;
use crate::service::database::models::User;
use crate::shared::errors::{CreateUserError, GetUserError, RepositoryError::*};
use crate::shared::primitives::UserRole;
use anyhow::Result;
use diesel::prelude::*;
use log::error;
use uuid::Uuid;

impl User {
    pub fn new(
        username: &str,
        github_username: &str,
        email: &str,
        profile_pic_url: &str,
        role: UserRole,
    ) -> Self {
        User {
            id: Uuid::new_v4(),
            username: username.to_lowercase(),
            github_username: github_username.to_lowercase(),
            email: email.to_lowercase(),
            profile_pic_url: profile_pic_url.to_string(),
            role: role.to_str().to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create(connection: &mut PgConnection, user: User) -> Result<User> {
        use crate::schema::users::dsl::{email, github_username, username};

        let existing_user = users::table
            .filter(username.eq(user.username.to_lowercase().clone()))
            .or_filter(github_username.eq(user.github_username.to_lowercase().clone()))
            .or_filter(email.eq(user.email.to_lowercase().clone()))
            .first::<User>(connection)
            .optional()
            .map_err(|e| {
                error!("Error getting user: {}", e);
                FailedToCreateUser(CreateUserError(e))
            })?;

        if existing_user.is_some() {
            return Err(UserAlreadyExists.into());
        }

        match diesel::insert_into(users::table)
            .values(user)
            .returning(User::as_returning())
            .get_result(connection)
        {
            Ok(new_user) => Ok(new_user),
            Err(e) => {
                error!("Error creating user: {}", e);
                Err(FailedToCreateUser(CreateUserError(e)).into())
            }
        }
    }

    pub fn get_user(
        connection: &mut PgConnection,
        user_id: Option<&Uuid>,
        username: Option<&str>,
        email: Option<&str>,
        github_username: Option<&str>,
    ) -> Result<User> {
        use crate::schema::users::dsl::{
            email as email_col, github_username as github_username_col, username as username_col,
        };

        let param_count = user_id.is_some() as u8
            + username.is_some() as u8
            + email.is_some() as u8
            + github_username.is_some() as u8;

        if param_count != 1 {
            return Err(anyhow::anyhow!(
                "Multiple parameters are not allowed. Please provide only one parameter."
            ));
        }

        match (user_id, username, email, github_username) {
            (Some(user_id), None, None, None) => {
                let user = users::table
                    .find(user_id)
                    .first::<User>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting user: {}", e);
                        FailedToGetUser(GetUserError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("User not found"))?;
                Ok(user)
            }
            (None, Some(username), None, None) => {
                let user = users::table
                    .filter(username_col.eq(username))
                    .first::<User>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting user: {}", e);
                        FailedToGetUser(GetUserError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("User not found"))?;
                Ok(user)
            }
            (None, None, Some(email), None) => {
                let user = users::table
                    .filter(email_col.eq(email))
                    .first::<User>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting user: {}", e);
                        FailedToGetUser(GetUserError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("User not found"))?;
                Ok(user)
            }
            (None, None, None, Some(github_username)) => {
                let user = users::table
                    .filter(github_username_col.eq(github_username))
                    .first::<User>(connection)
                    .optional()
                    .map_err(|e| {
                        error!("Error getting user: {}", e);
                        FailedToGetUser(GetUserError(e))
                    })?
                    .ok_or_else(|| anyhow::anyhow!("User not found"))?;
                Ok(user)
            }
            _ => Err(anyhow::anyhow!("No input provided")),
        }
    }
}
