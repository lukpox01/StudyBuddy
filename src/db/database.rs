use crate::db::errors::DatabaseError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Datetime, Thing, Value};
use surrealdb::syn::v1::literal::datetime;
use surrealdb::Surreal;

use crate::models::users::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub struct Database {
    db: Surreal<Client>,
}

impl Database {
    pub async fn new(ns: &str, db_n: &str) -> Result<Self, DatabaseError> {
        let db = match Surreal::new::<Ws>("127.0.0.1:8000").await {
            Ok(v) => v,
            Err(_) => return Err(DatabaseError::ConnectionError),
        };

        match db
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await
        {
            Ok(_) => (),
            Err(_) => return Err(DatabaseError::AuthenticationError),
        };

        match db.use_ns(ns).use_db(db_n).await {
            Ok(_) => (),
            Err(_) => return Err(DatabaseError::SwitchingError),
        };

        match db
            .query(
                "
                DEFINE TABLE user SCHEMAFULL;
                DEFINE FIELD username ON user TYPE string;
                DEFINE FIELD email ON user TYPE string;
                DEFINE FIELD password_hash ON user TYPE string;
                DEFINE FIELD created_at ON user TYPE datetime;
                DEFINE FIELD last_login ON user TYPE option<datetime>;
                DEFINE FIELD status ON user TYPE string;
                DEFINE INDEX email_idx ON user FIELDS email UNIQUE;
            ",
            )
            .await
        {
            Ok(_) => (),
            Err(_) => return Err(DatabaseError::QueryError),
        };

        match db
            .query(
                "
                DEFINE TABLE session SCHEMAFULL;
                DEFINE FIELD user_id ON session TYPE record(user);
                DEFINE FIELD refresh_token ON session TYPE string;
                DEFINE FIELD expires_at ON session TYPE datetime;
                DEFINE INDEX refresh_token_idx ON session FIELDS refresh_token UNIQUE;
            ",
            )
            .await
        {
            Ok(_) => (),
            Err(_) => return Err(DatabaseError::QueryError),
        };

        match db
            .query(
                "
                DEFINE TABLE password_reset SCHEMAFULL;
                DEFINE FIELD user_id ON password_reset TYPE record(user);
                DEFINE FIELD token ON password_reset TYPE string;
                DEFINE FIELD expires_at ON password_reset TYPE datetime;
                DEFINE INDEX reset_token_idx ON password_reset FIELDS token UNIQUE;
            ",
            )
            .await
        {
            Ok(_) => (),
            Err(_) => return Err(DatabaseError::QueryError),
        };

        Ok(Database { db })
    }

    pub async fn create_user(&self, user_data: User) -> Result<Vec<Record>, DatabaseError> {
        match self.db.create("user").content(user_data).await {
            Ok(v) => Ok(v),
            Err(_) => Err(DatabaseError::CreationError),
        }
    }

    async fn select_by_id(&self, id: Thing) -> Result<Option<Record>, DatabaseError> {
        match self.db.select(id).await {
            Ok(v) => Ok(v),
            Err(_) => Err(DatabaseError::QueryError),
        }
    }

    pub async fn select_user_by_id(&self, id: &str) {}
}
