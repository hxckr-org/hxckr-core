use crate::schema::users;
use crate::service::database::models::User;
use crate::shared::errors::{CreateUserError, GetUserError, RepositoryError::*};
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
        role: &str,
    ) -> Self {
        User {
            id: Uuid::new_v4(),
            username: username.to_lowercase(),
            github_username: github_username.to_lowercase(),
            email: email.to_lowercase(),
            profile_pic_url: profile_pic_url.to_string(),
            role: role.to_lowercase(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn create_user(connection: &mut PgConnection, user: User) -> Result<User> {
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
        username: Option<&str>,
        email: Option<&str>,
        github_username: Option<&str>,
    ) -> Result<User> {
        use crate::schema::users::dsl::{
            email as email_col, github_username as github_username_col, username as username_col,
        };
        let user = users::table
            .filter(username_col.eq(username.unwrap_or_default()))
            .or_filter(email_col.eq(email.unwrap_or_default()))
            .or_filter(github_username_col.eq(github_username.unwrap_or_default()))
            .first::<User>(connection)
            .optional()
            .map_err(|e| {
                error!("Error getting user: {}", e);
                FailedToGetUser(GetUserError(e))
            })?;

        user.ok_or_else(|| {
            error!("User not found");
            UserNotFound.into()
        })
    }
}
