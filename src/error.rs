use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShilpError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("API error: {message} (status: {status})")]
    ApiError { message: String, status: u16 },

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Failed to serialize request body: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, ShilpError>;
