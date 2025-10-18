use thiserror::Error;

pub type TogglrResult<T> = Result<T, TogglrError>;

#[derive(Error, Debug)]
pub enum TogglrError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Feature not found: {0}")]
    FeatureNotFound(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("TLS error: {0}")]
    TlsError(#[from] rustls::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
