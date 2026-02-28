use thiserror::Error;

/// Stage3 fetching and management errors
#[derive(Debug, Error)]
pub enum Stage3Error {
    #[error("Failed to fetch stage3 list: {0}")]
    FetchError(String),

    #[error("Failed to parse stage3 metadata: {0}")]
    ParseError(String),

    #[error("Failed to download stage3 image: {0}")]
    DownloadError(String),

    #[error("Failed to verify stage3 image: {0}")]
    VerifyError(String),

    #[error("Failed to extract stage3 image: {0}")]
    ExtractError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
