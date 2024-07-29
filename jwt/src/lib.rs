use chrono::{Duration, Utc};
use error::JwtError;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

mod error;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
    token_type: String,
}

pub struct JwtManager {
    access_encoding_key: EncodingKey,
    access_decoding_key: DecodingKey,
    refresh_encoding_key: EncodingKey,
    refresh_decoding_key: DecodingKey,
}

impl JwtManager {
    pub fn new(access_secret: &[u8], refresh_secret: &[u8]) -> Self {
        Self {
            access_encoding_key: EncodingKey::from_secret(access_secret),
            access_decoding_key: DecodingKey::from_secret(access_secret),
            refresh_encoding_key: EncodingKey::from_secret(refresh_secret),
            refresh_decoding_key: DecodingKey::from_secret(refresh_secret),
        }
    }

    pub fn create_access_token(&self, user_id: &str) -> Result<String, JwtError> {
        self.create_token(
            user_id,
            Duration::minutes(15),
            "access",
            &self.access_encoding_key,
        )
    }

    pub fn create_refresh_token(&self, user_id: &str) -> Result<String, JwtError> {
        self.create_token(
            user_id,
            Duration::days(7),
            "refresh",
            &self.refresh_encoding_key,
        )
    }

    fn create_token(
        &self,
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

    pub fn verify_access_token(&self, token: &str) -> Result<Claims, JwtError> {
        self.verify_token(token, &self.access_decoding_key, "access")
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims, JwtError> {
        self.verify_token(token, &self.refresh_decoding_key, "refresh")
    }

    fn verify_token(
        &self,
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
}
