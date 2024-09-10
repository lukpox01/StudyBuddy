use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Datetime, Id, Thing};
use surrealdb::Surreal;
use uuid::Uuid;

use crate::db::error::DatabaseError;
use crate::models::auth::{VerificationToken, AddSecret, GetSecret};
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
                DEFINE TABLE secret SCHEMAFULL;
                DEFINE FIELD user_id ON secret TYPE record(user);
                DEFINE FIELD token ON secret TYPE string;
                DEFINE INDEX token_idx ON secret FIELDS token UNIQUE;
            ",
            )
            .await
        {
            Ok(_) => (),
            Err(_) => return Err(DatabaseError::QueryError),
        };

        Ok(Database { db })
    }

    pub async fn create_user(&self, user_data: User) -> Result<Vec<User>, DatabaseError> {
        let input = User {
            id: Thing::from(("user", Uuid::new_v4().to_string().as_str())),
            username: user_data.username,
            email: user_data.email,
            password_hash: user_data.password_hash,
            created_at: Datetime(Utc::now()),
            last_login: None,
            status: user_data.status,
        };
        match self.db.create("user").content(input).await {
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
        match self
            .db
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", email))
            .await
        {
            Ok(mut v) => match v.take::<Option<User>>(0) {
                Ok(v) => match v {
                    Some(v) => Ok(v),
                    None => Err(DatabaseError::QueryError),
                },
                Err(_) => Err(DatabaseError::QueryError),
            },
            Err(_) => Err(DatabaseError::QueryError),
        }
    }

    pub async fn add_secret(&self, user_id: Thing, token: &str) -> Result<Vec<Record>, DatabaseError> {
        let input = AddSecret {
            token: token.to_string(),
            user_id,
        };
        match self.db.create("secret").content(input).await {
            Ok(v) => Ok(v),
            Err(_) => Err(DatabaseError::SecretError),
        }
    }

    pub async fn get_secret_by_email(&self, email: &str) -> Result<GetSecret, DatabaseError> {
        match self
            .db
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", email))
            .await
        {
            Ok(mut v) => match v.take::<Option<User>>(0) {
                Ok(v) => match v {
                    Some(v) => {
                        match self
                            .db
                            .query("SELECT * FROM secret WHERE user_id = $user_id")
                            .bind(("user_id", v.id))
                            .await
                        {
                            Ok(mut v) => match v.take::<Option<GetSecret>>(0) {
                                Ok(v) => match v {
                                    Some(v) => Ok(v),
                                    None => Err(DatabaseError::SecretNotFound),
                                },
                                Err(_) => Err(DatabaseError::SecretNotFound),
                            },
                            Err(_) => Err(DatabaseError::SecretNotFound),
                        }
                    },
                    None => Err(DatabaseError::UserNotFound),
                },
                Err(_) => Err(DatabaseError::UserNotFound),
            },
            Err(_) => Err(DatabaseError::UserNotFound),
        }

    }
}
