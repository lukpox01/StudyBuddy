use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum DatabaseError {
    #[error("Failed to connect to database")]
    ConnectionError,
    #[error("Failed to authenticate")]
    AuthenticationError,
    #[error("Failed to switch namespace or database")]
    SwitchingError,
    #[error("Failed to execute query")]
    QueryError,
    #[error("Failed to create record")]
    CreationError,
    #[error("Unknown identifier")]
    UnknownID,
}
