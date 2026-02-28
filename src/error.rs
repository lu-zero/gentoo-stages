use thiserror::Error;

/// Stage3 fetching and management errors
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Failed to parse stage3 metadata: {0}")]
    ParseError(String),

    #[error("Stage3 variant not found: {0}")]
    VariantNotFound(String),

    #[error("Failed to extract stage3 image: {0}")]
    ExtractError(String),

    #[error("Stage3 image not found")]
    NotFound,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
