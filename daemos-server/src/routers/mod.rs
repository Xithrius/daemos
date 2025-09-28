mod health;
mod not_found;

use actix_web::{http::StatusCode, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(health::config)
        .default_service(web::get().to(not_found::not_found));
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Environment Error")]
    Env(#[from] dotenvy::Error),
    #[error("Invalid Input: {0}")]
    InvalidInput(String),
    #[error("Error while validating input: {0}")]
    Validation(String),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Resource not found")]
    NotFound,
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Env(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidInput(..) | Self::Validation(..) | Self::Io(..) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
