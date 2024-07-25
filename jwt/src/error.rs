use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Failed to create token")]
    TokenCreationError,
    #[error("Failed to decode token")]
    TokenDecodingError,
    #[error("Token expired")]
    TokenExpired,
    #[error("Invalid token type")]
    InvalidTokenType,
}
