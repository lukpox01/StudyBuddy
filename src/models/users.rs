use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Thing,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Datetime,
    pub last_login: Option<Datetime>,
    pub status: String,
}
