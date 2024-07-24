#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError,
    AuthenticationError,
    SwitchingError,
    QueryError,
    CreationError,
}
