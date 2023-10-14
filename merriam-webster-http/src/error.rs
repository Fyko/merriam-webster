//! Error types for the Merriam Webster HTTP Client.
use thiserror::Error;

/// Represents a list of errors that can be throwen by Merriam Webster API.
#[derive(Debug, Error)]
pub enum MerriamWebsterError {
    /// The error originated from Hyper.
    #[error("Hyper error: {0:?}")]
    HyperError(#[from] hyper::Error),

    /// The error originated from Serde.
    #[error("Serde error: {0:?}")]
    SerdeError(#[from] serde_json::Error),
}
