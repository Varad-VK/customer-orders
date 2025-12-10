use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Order not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    BadRequest(String),
    #[error("Database error")]
    DbError(#[from] sqlx::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let msg = self.to_string();
        let body = ErrorResponse { error: msg };
        match self {
            ServiceError::NotFound => HttpResponse::NotFound().json(body),
            ServiceError::BadRequest(_) => HttpResponse::BadRequest().json(body),
            ServiceError::DbError(_) => HttpResponse::InternalServerError().json(body),
        }
    }
}
