use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct RegisterInput {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(
        min = 6,
        max = 50,
        message = "Password must be between 6 and 50 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginInput {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(
        min = 6,
        max = 50,
        message = "Password must be between 6 and 50 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct VerificationToken {
    pub user_id: Thing,
    pub token: Uuid,
    pub expires_at: Datetime,
}
