use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable, Debug, Serialize)]
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

#[derive(Queryable, Insertable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::challenges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Challenge {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub repo_urls: serde_json::Value,
    pub difficulty: String,
    pub module_count: i32,
    pub mode: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Selectable, Debug, Serialize)]
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
    pub status: String,
}

#[derive(Queryable, Insertable, Selectable, Debug, Serialize, Clone)]
#[diesel(table_name = crate::schema::progress)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Progress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub challenge_id: Uuid,
    pub repository_id: Uuid,
    pub status: String,
    pub progress_details: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = crate::schema::repositories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Repository {
    pub id: Uuid,
    pub user_id: Uuid,
    pub challenge_id: Uuid,
    pub repo_url: String,
    pub soft_serve_url: String,
    pub language: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = crate::schema::submissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Submission {
    pub id: Uuid,
    pub user_id: Uuid,
    pub exercise_id: Uuid,
    pub commit_id: String,
    pub repository_id: Uuid,
    pub status: String,
    pub feedback: Option<String>,
    pub submitted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub provider: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = crate::schema::leaderboard)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Leaderboard {
    pub id: i32,
    pub user_id: Uuid,
    pub score: i32,
    pub expected_total_score: i32,
    pub achievements: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::leaderboard)]
pub struct NewLeaderboard {
    pub user_id: Uuid,
    pub score: i32,
    pub expected_total_score: i32,
    pub achievements: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone, Queryable)]
pub struct LeaderboardWithChallenge {
    pub id: i32,
    pub user_id: Uuid,
    pub score: i32,
    pub expected_total_score: i32,
    pub github_username: String,
    pub completed_challenge_count: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = crate::schema::badges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Badge {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::badges)]
pub struct NewBadge {
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = crate::schema::user_badges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct UserBadge {
    pub id: i32,
    pub user_id: Uuid,
    pub badge_id: i32,
    pub awarded_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_badges)]
pub struct NewUserBadge {
    pub user_id: Uuid,
    pub badge_id: i32,
    pub awarded_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Serialize)]
pub struct RepositoryWithRelations {
    pub id: Uuid,
    pub user_id: Uuid,
    pub challenge_id: Uuid,
    pub repo_url: String,
    pub soft_serve_url: String,
    pub language: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub challenge: ChallengeInfo,
    pub progress: ProgressInfo,
}
#[derive(Debug, Queryable, Serialize)]
pub struct AttemptInfo {
    pub challenge_id: Uuid,
    pub username: String,
    pub total_score: i32,
    pub module_count: i32,
    pub language: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct ChallengeInfo {
    pub title: String,
    pub description: String,
    pub repo_urls: serde_json::Value,
    pub difficulty: String,
    pub module_count: i32,
    pub mode: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Serialize, Default)]
pub struct ProgressInfo {
    pub id: Uuid,
    pub status: String,
    pub progress_details: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
