use chrono::{Duration, Utc};

use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::jwt::error::JwtError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
    token_type: String,
}
pub fn create_access_token(user_id: &str, access_secret: &[u8]) -> Result<String, JwtError> {
    create_token(
        user_id,
        Duration::minutes(15),
        "access",
        &EncodingKey::from_secret(access_secret),
    )
}

pub fn create_refresh_token(user_id: &str, refresh_secret: &[u8]) -> Result<String, JwtError> {
    create_token(
        user_id,
        Duration::days(7),
        "refresh",
        &EncodingKey::from_secret(refresh_secret),
    )
}

fn create_token(
    user_id: &str,
    duration: Duration,
    token_type: &str,
    key: &EncodingKey,
) -> Result<String, JwtError> {
    let now = Utc::now();
    let expires_at = now + duration;
    let claims = Claims {
        sub: user_id.to_owned(),
        iat: now.timestamp(),
        exp: expires_at.timestamp(),
        token_type: token_type.to_owned(),
    };
    let token = match encode(&Header::default(), &claims, key) {
        Ok(v) => v,
        Err(_) => return Err(JwtError::TokenCreationError),
    };
    Ok(token)
}

pub fn verify_access_token(token: &str, access_secret: &[u8]) -> Result<Claims, JwtError> {
    verify_token(token, &DecodingKey::from_secret(access_secret), "access")
}

pub fn verify_refresh_token(token: &str, refresh_secret: &[u8]) -> Result<Claims, JwtError> {
    verify_token(token, &DecodingKey::from_secret(refresh_secret), "refresh")
}

fn verify_token(
    token: &str,
    key: &DecodingKey,
    expected_type: &str,
) -> Result<Claims, JwtError> {
    let validation = Validation::default();
    let token_data = match decode::<Claims>(token, key, &validation) {
        Ok(v) => v,
        Err(_) => return Err(JwtError::TokenDecodingError),
    };

    let now = Utc::now().timestamp();
    if token_data.claims.exp < now {
        return Err(JwtError::TokenExpired);
    }

    if token_data.claims.token_type != expected_type {
        return Err(JwtError::InvalidTokenType);
    }

    Ok(token_data.claims)
}
