pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
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
}

// Had to add this because I couldn't use the `diesel::result::Error`
// in the `#[from]` attribute
// in the `RepositoryError` enum for more than one variant.
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
