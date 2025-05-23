use meilisearch_types::error::{Code, ErrorCode};

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error("The Authorization header is missing. It must use the bearer authorization method.")]
    MissingAuthorizationHeader,
    #[error("The provided API key is invalid.")]
    InvalidToken,
    // Triggered on configuration error.
    #[error("An internal error has occurred. `Irretrievable state`.")]
    IrretrievableState,
    #[error("Meilisearch is running without a master key. To access this API endpoint, you must have set a master key at launch.")]
    MissingMasterKey,
}

impl ErrorCode for AuthenticationError {
    fn error_code(&self) -> Code {
        match self {
            Self::MissingAuthorizationHeader => Code::MissingAuthorizationHeader,
            Self::InvalidToken => Code::InvalidApiKey,
            Self::IrretrievableState => Code::Internal,
            Self::MissingMasterKey => Code::MissingMasterKey,
        }
    }
}
