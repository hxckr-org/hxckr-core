use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
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

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub id: Uuid,
    pub username: String,
    pub github_username: String,
    pub email: String,
    pub profile_pic_url: String,
    pub role: String,
}

impl NewUser {
    pub fn new(
        username: &str,
        github_username: &str,
        email: &str,
        profile_pic_url: &str,
        role: &str,
    ) -> Self {
        NewUser {
            id: Uuid::new_v4(),
            username: username.to_lowercase(),
            github_username: github_username.to_lowercase(),
            email: email.to_lowercase(),
            profile_pic_url: profile_pic_url.to_string(),
            role: role.to_lowercase(),
        }
    }
}
