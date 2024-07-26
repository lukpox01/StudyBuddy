use actix_web::web::Buf;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Datetime, Id, Thing};
use surrealdb::Surreal;
use uuid::Uuid;

use crate::db::error::DatabaseError;
use crate::models::auth::VerificationToken;
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

        match db
            .query(
                "
                DEFINE TABLE verification_token SCHEMAFULL;
                DEFINE FIELD token ON verification_token TYPE string;
                DEFINE FIELD user_id ON verification_token TYPE record(user);
                DEFINE FIELD expires_at ON verification_token TYPE datetime;
                DEFINE INDEX token_idx ON verification_token FIELDS token UNIQUE;
            "
            )
            .await
        {
            Ok(_) => (),
            Err(_) => return Err(DatabaseError::QueryError),
        };

        Ok(Database { db })
    }

    pub async fn create_user(&self, user_data: User) -> Result<Vec<User>, DatabaseError> {
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

    pub async fn select_user_by_id(&self, id: &str) -> Result<Option<Record>, DatabaseError> {
        let id = Thing::from(("user", id));
        match self.select_by_id(id).await {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    pub async fn select_user_by_email(&self, email: &str) -> Result<User, DatabaseError> {
        match self.db.query("SELECT * FROM user WHERE email = $email").bind(("email", email)).await {
            Ok(mut v) => {
                match v.take::<Option<User>>(0) {
                    Ok(v) => {
                        
                        match v {
                            Some(v) => Ok(v),
                            None => Err(DatabaseError::QueryError),
                        }
                    },
                    Err(_) => Err(DatabaseError::QueryError),
                }
            }
            Err(_) => Err(DatabaseError::QueryError),
        }
    }

    pub async fn create_verification_token(&self, user_id: &str, token: Uuid) -> Result<Option<Record>, DatabaseError> {
        let expires_at = Datetime(Utc::now() + chrono::Duration::days(1));
        let user_id = Thing::from(("user", user_id));

        let verification_token = VerificationToken {
            user_id,
            token,
            expires_at,
        };

        match self.db.create(("verification_token", token.to_string().as_str())).content(verification_token).await {
            Ok(v) => Ok(v),
            Err(_) => Err(DatabaseError::CreationError),
        }
    }

    pub async fn verify_email_token(&self, token: Uuid) -> Result<Option<Record>, DatabaseError> {
        let token = Thing::from(("verification_token", token.to_string().as_str()));
        match self.select_by_id(token).await {
            Ok(Some(v)) => {
                let id: String = match v.id.id {
                    Id::Number(v) => { v.to_string() }
                    Id::String(v) => { v }
                    Id::Array(v) => { return Err(DatabaseError::UnknownID) }
                    Id::Object(v) => { return Err(DatabaseError::UnknownID) }
                    Id::Generate(V) => { return Err(DatabaseError::UnknownID) }
                };
                let user = match self.select_user_by_id(id.as_str()).await {
                    Ok(Some(v)) => v,
                    Ok(None) => return Err(DatabaseError::QueryError),
                    Err(e) => return Err(e),
                };

                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn activate_user(&self, id: Thing) -> Result<(), DatabaseError> {
        todo!()
    }
}
