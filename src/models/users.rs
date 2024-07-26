use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Thing,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Datetime,
    pub last_login: Option<Datetime>,
    pub status: String,
}
