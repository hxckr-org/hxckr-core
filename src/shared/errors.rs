pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Server configuration error: {0}")]
    ServerConfigurationError(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    DatabaseError(String),
    #[error("User with the same username, github username, or email already exists")]
    UserAlreadyExists,
    #[error("Failed to create user")]
    FailedToCreateUser(#[from] CreateUserError),
    #[error("User not found")]
    UserNotFound,
    #[error("Failed to get user")]
    FailedToGetUser(#[from] GetUserError),
    #[error("Failed to create challenge")]
    FailedToCreateChallenge(#[from] CreateChallengeError),
    #[error("Challenge not found")]
    ChallengeNotFound,
    #[error("Failed to get challenge")]
    FailedToGetChallenge(#[from] GetChallengeError),
    #[error("Failed to create exercise")]
    FailedToCreateExercise(#[from] CreateExerciseError),
    #[error("Failed to get exercise")]
    FailedToGetExercise(#[from] GetExerciseError),
    #[error("Failed to create progress")]
    FailedToCreateProgress(#[from] CreateProgressError),
    #[error("Failed to get progress")]
    FailedToGetProgress(#[from] GetProgressError),
    #[error("Failed to update progress")]
    FailedToUpdateProgress(#[from] UpdateProgressError),
    #[error("Repository already exists")]
    RepositoryAlreadyExists,
    #[error("Failed to create repository")]
    FailedToCreateRepository(#[from] CreateRepositoryError),
    #[error("Failed to get repository")]
    FailedToGetRepository(#[from] GetRepositoryError),
    #[error("Failed to create submission")]
    FailedToCreateSubmission(#[from] CreateSubmissionError),
    #[error("Failed to get submission")]
    FailedToGetSubmission(#[from] GetSubmissionError),
    #[error("Failed to create leaderboard")]
    FailedToCreateLeaderboard(#[from] CreateLeaderboardError),
    #[error("Failed to get leaderboard")]
    FailedToGetLeaderboard(#[from] GetLeaderboardError),
    #[error("Failed to update leaderboard")]
    FailedToUpdateLeaderboard(#[from] UpdateLeaderboardError),
    #[error("Failed to create badge")]
    FailedToCreateBadge(#[from] CreateBadgeError),
    #[error("Failed to get badge")]
    FailedToGetBadge(#[from] GetBadgeError),
    #[error("Failed to create user badge")]
    FailedToCreateUserBadge(#[from] CreateUserBadgeError),
    #[error("Failed to get user badge")]
    FailedToGetUserBadge(#[from] GetUserBadgeError),
}

impl From<diesel::result::Error> for RepositoryError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => RepositoryError::NotFound("Record not found".into()),
            diesel::result::Error::DatabaseError(kind, info) => {
                RepositoryError::DatabaseError(format!("Database error: {:?}, {:?}", kind, info))
            }
            _ => RepositoryError::DatabaseError("An unknown database error occurred".into()),
        }
    }
}

// Had to add this because I couldn't use the `diesel::result::Error`
// in the `#[from]` attribute
// in the `RepositoryError` enum for more than one variant.
// However, they're all using the same connection, so it's not like
// they're being used in different contexts.
// And they provide robust context on the database error which is useful
// for debugging.
// E.g.,
// [2024-08-29T10:08:08Z ERROR hxckr_core::service::repository::submission] Error creating submission: new row for relation "submissions" violates check constraint "submissions_status_check"
// Error creating submission: Failed to create submission
// Error: Failed to create submission
// Caused by:
//     0: Database error while creating submission: new row for relation "submissions" violates check constraint "submissions_status_check"
//     1: new row for relation "submissions" violates check constraint "submissions_status_check"
// TODO: Find a better solution.
#[derive(Error, Debug)]
#[error("Database error while creating user: {0}")]
pub struct CreateUserError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while getting user: {0}")]
pub struct GetUserError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while creating challenge: {0}")]
pub struct CreateChallengeError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while getting challenge: {0}")]
pub struct GetChallengeError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while creating exercise: {0}")]
pub struct CreateExerciseError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while getting exercise: {0}")]
pub struct GetExerciseError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while creating progress: {0}")]
pub struct CreateProgressError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while getting progress: {0}")]
pub struct GetProgressError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while updating progress: {0}")]
pub struct UpdateProgressError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while creating repository: {0}")]
pub struct CreateRepositoryError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while getting repository: {0}")]
pub struct GetRepositoryError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while creating submission: {0}")]
pub struct CreateSubmissionError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while getting submission: {0}")]
pub struct GetSubmissionError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while creating leaderboard: {0}")]
pub struct CreateLeaderboardError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while getting leaderboard: {0}")]
pub struct GetLeaderboardError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while updating leaderboard: {0}")]
pub struct UpdateLeaderboardError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while creating badge: {0}")]
pub struct CreateBadgeError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while getting badge: {0}")]
pub struct GetBadgeError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while creating user badge: {0}")]
pub struct CreateUserBadgeError(#[from] pub diesel::result::Error);

#[derive(Error, Debug)]
#[error("Database error while getting user badge: {0}")]
pub struct GetUserBadgeError(#[from] pub diesel::result::Error);
