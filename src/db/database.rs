use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub(crate) struct Database {
    db: Surreal<Client>,
}

impl Database {

    pub(crate) async fn new(ns: &str, db_n: &str) -> Self {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

        db.signin(Root {
            username: "root",
            password: "root",
        }).await.unwrap();

        db.use_ns(ns).use_db(db_n).await.unwrap();

        db.query(
            "
                DEFINE TABLE user SCHEMAFULL;
                DEFINE FIELD username ON user TYPE string;
                DEFINE FIELD email ON user TYPE string;
                DEFINE FIELD password_hash ON user TYPE string;
                DEFINE FIELD created_at ON user TYPE datetime;
                DEFINE FIELD last_login ON user TYPE datetime;
                DEFINE FIELD status ON user TYPE string;
                DEFINE INDEX email_idx ON user FIELDS email UNIQUE;
            "
        ).await.unwrap();

        db.query(
            "
                DEFINE TABLE session SCHEMAFULL;
                DEFINE FIELD user_id ON session TYPE record(user);
                DEFINE FIELD refresh_token ON session TYPE string;
                DEFINE FIELD expires_at ON session TYPE datetime;
                DEFINE INDEX refresh_token_idx ON session FIELDS refresh_token UNIQUE;
            "
        ).await.unwrap();

        db.query(
            "
                DEFINE TABLE password_reset SCHEMAFULL;
                DEFINE FIELD user_id ON password_reset TYPE record(user);
                DEFINE FIELD token ON password_reset TYPE string;
                DEFINE FIELD expires_at ON password_reset TYPE datetime;
                DEFINE INDEX reset_token_idx ON password_reset FIELDS token UNIQUE;
            "
        ).await.unwrap();

        Database {
            db
        }
    }

}