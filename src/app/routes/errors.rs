use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;

use crate::shared::errors::RepositoryError;

impl ResponseError for RepositoryError {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_message = self.to_string();

        HttpResponse::build(status).json(json!({
            "status": "error",
            "message": error_message
        }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            RepositoryError::BadRequest(_) => StatusCode::BAD_REQUEST,
            RepositoryError::RepositoryAlreadyExists => StatusCode::CONFLICT,
            RepositoryError::UserAlreadyExists => StatusCode::CONFLICT,
            RepositoryError::UserNotFound => StatusCode::NOT_FOUND,
            RepositoryError::FailedToGetUser(_) => StatusCode::NOT_FOUND,
            RepositoryError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RepositoryError::ServerConfigurationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
