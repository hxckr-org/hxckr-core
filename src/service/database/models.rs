use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::shared::utils::string_to_uuid;

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub github_username: String,
    pub email: String,
    pub profile_pic_url: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = crate::schema::challenges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Challenge {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub repo_url: String,
    pub difficulty: String,
    pub mode: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Challenge {
    pub fn new(
        title: &str,
        description: &str,
        repo_url: &str,
        difficulty: &str,
        mode: &str,
    ) -> Self {
        Challenge {
            id: Uuid::new_v4(),
            title: title.to_lowercase().trim().to_string(),
            description: description.to_string(),
            repo_url: repo_url.to_string(),
            difficulty: difficulty.to_lowercase(),
            mode: mode.to_lowercase(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = crate::schema::exercises)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Exercise {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub test_runner: String,
    pub challenge_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Exercise {
    pub fn new(
        title: &str,
        description: &str,
        difficulty: &str,
        test_runner: &str,
        challenge_id: &str,
    ) -> Self {
        Exercise {
            id: Uuid::new_v4(),
            title: title.to_lowercase().trim().to_string(),
            description: description.to_string(),
            difficulty: difficulty.to_lowercase(),
            test_runner: test_runner.to_string(),
            challenge_id: string_to_uuid(challenge_id).unwrap(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
