use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
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
